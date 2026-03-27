// Phase 157: SMB 共享更新 API
// PUT /api/v1/shares/smb/{id} — 更新 SMB 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新 SMB 共享请求
#[derive(Debug, Deserialize)]
pub struct UpdateSmbShareRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub comment: Option<String>,
    pub read_only: Option<bool>,
    pub guest_access: Option<bool>,
    pub browseable: Option<bool>,
    pub valid_users: Option<Vec<String>>,
    pub invalid_users: Option<Vec<String>>,
}

/// SMB 共享信息
#[derive(Serialize, Clone)]
pub struct SmbShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub comment: String,
    pub read_only: bool,
    pub guest_access: bool,
    pub browseable: bool,
    pub valid_users: Vec<String>,
    pub invalid_users: Vec<String>,
    pub enabled: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 更新 SMB 共享响应
#[derive(Serialize)]
pub struct UpdateSmbShareResponse {
    pub success: bool,
    pub message: String,
    pub data: SmbShareInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证共享名称格式
fn validate_share_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 64 && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// 验证共享路径格式
fn validate_share_path(path: &str) -> bool {
    path.starts_with('/') && path.len() <= 256
}

/// 更新 SMB 共享（Phase 157）
/// - JWT 认证，仅 admin 角色可访问
/// - 请求体包含：name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users（可选，部分更新）
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 验证名称唯一性（409 Conflict，排除自身）
/// - 更新成功返回 200 OK + 共享详情
pub async fn update_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateSmbShareRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let share_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update SMB shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证名称格式（如果提供）
    if let Some(ref name) = payload.name {
        if !validate_share_name(name) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed".to_string(),
                code: "INVALID_NAME".to_string(),
            }));
        }
    }

    // 5. 验证路径格式（如果提供）
    if let Some(ref path) = payload.path {
        if !validate_share_path(path) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid share path. Must start with / and be <= 256 chars".to_string(),
                code: "INVALID_PATH".to_string(),
            }));
        }
    }

    // 6. 模拟现有 SMB 共享数据
    let mut mock_shares = vec![
        SmbShareInfo {
            id: 1,
            name: "Public".to_string(),
            path: "/srv/samba/public".to_string(),
            comment: "Public shared folder".to_string(),
            read_only: false,
            guest_access: true,
            browseable: true,
            valid_users: vec![],
            invalid_users: vec![],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        SmbShareInfo {
            id: 2,
            name: "Users".to_string(),
            path: "/srv/samba/users".to_string(),
            comment: "Users shared folder".to_string(),
            read_only: false,
            guest_access: false,
            browseable: true,
            valid_users: vec!["user1".to_string(), "user2".to_string()],
            invalid_users: vec![],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        SmbShareInfo {
            id: 3,
            name: "Backup".to_string(),
            path: "/srv/samba/backup".to_string(),
            comment: "Backup shared folder".to_string(),
            read_only: true,
            guest_access: false,
            browseable: false,
            valid_users: vec!["admin".to_string()],
            invalid_users: vec![],
            enabled: false,
            status: "inactive".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 7. 查找共享
    let share_index = mock_shares.iter().position(|s| s.id == share_id);

    // 8. 验证共享存在性
    if share_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("SMB share {} not found", share_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let share_index = share_index.unwrap();

    // 9. 验证名称唯一性（排除自身）
    if let Some(ref new_name) = payload.name {
        let name_exists = mock_shares.iter().any(|s| s.id != share_id && s.name == *new_name);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("SMB share name '{}' already exists", new_name),
                code: "NAME_CONFLICT".to_string(),
            }));
        }
    }

    // 10. 更新共享配置（部分更新）
    let share = &mut mock_shares[share_index];
    
    if let Some(new_name) = &payload.name {
        share.name = new_name.clone();
    }
    if let Some(new_path) = &payload.path {
        share.path = new_path.clone();
    }
    if let Some(ref new_comment) = payload.comment {
        share.comment = new_comment.clone();
    }
    if let Some(new_read_only) = payload.read_only {
        share.read_only = new_read_only;
    }
    if let Some(new_guest_access) = payload.guest_access {
        share.guest_access = new_guest_access;
    }
    if let Some(new_browseable) = payload.browseable {
        share.browseable = new_browseable;
    }
    if let Some(ref new_valid_users) = payload.valid_users {
        share.valid_users = new_valid_users.clone();
    }
    if let Some(ref new_invalid_users) = payload.invalid_users {
        share.invalid_users = new_invalid_users.clone();
    }

    // 11. 更新时间戳
    let now = chrono::Utc::now().to_rfc3339();
    share.updated_at = now.clone();

    // 12. 返回更新成功
    Ok(HttpResponse::Ok().json(UpdateSmbShareResponse {
        success: true,
        message: "SMB share updated successfully".to_string(),
        data: share.clone(),
    }))
}
