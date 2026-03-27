// Phase 93 - 更新共享文件夹 API
// PUT /api/v1/shares/{id} — 更新共享文件夹

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 更新共享文件夹请求
#[derive(Deserialize)]
pub struct UpdateShareRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub protocols: Option<Vec<String>>,
    pub read_only: Option<bool>,
    pub guest_access: Option<bool>,
    pub enabled: Option<bool>,
}

/// 共享文件夹响应
#[derive(Serialize, Clone)]
pub struct ShareInfo {
    pub id: u64,
    pub name: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub path: String,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub read_only: bool,
    pub guest_access: bool,
    pub enabled: bool,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: String,
}

/// 更新共享文件夹响应
#[derive(Serialize)]
pub struct UpdateShareResponse {
    pub success: bool,
    pub message: String,
    pub data: ShareInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 更新共享文件夹（Phase 93）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享 ID 存在性（404）
/// - 验证名称唯一性（排除自身）（409）
/// - 更新成功返回 200 OK
pub async fn update_share(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateShareRequest>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 仅 admin 角色可更新共享
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let share_id = path.into_inner();

    // 3. 模拟共享文件夹数据
    let mut mock_shares = vec![
        ShareInfo {
            id: 1,
            name: "Public".to_string(),
            volume_id: 2,
            volume_name: "Data Volume".to_string(),
            path: "/public".to_string(),
            description: Some("Public shared folder".to_string()),
            protocols: vec!["smb".to_string()],
            read_only: false,
            guest_access: true,
            enabled: true,
            status: "active".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
            created_by: "admin".to_string(),
        },
        ShareInfo {
            id: 2,
            name: "Private".to_string(),
            volume_id: 2,
            volume_name: "Data Volume".to_string(),
            path: "/private".to_string(),
            description: Some("Private shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string()],
            read_only: false,
            guest_access: false,
            enabled: true,
            status: "active".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
            created_by: "admin".to_string(),
        },
        ShareInfo {
            id: 3,
            name: "Media".to_string(),
            volume_id: 3,
            volume_name: "Backup Volume".to_string(),
            path: "/media".to_string(),
            description: Some("Media files".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string(), "afp".to_string()],
            read_only: true,
            guest_access: false,
            enabled: true,
            status: "active".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
            created_by: "admin".to_string(),
        },
    ];

    // 4. 查找共享文件夹（先检查存在性）
    let share_exists = mock_shares.iter().any(|s| s.id == share_id);
    if !share_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Share with ID {} not found", share_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 验证名称唯一性（排除自身）
    if let Some(ref new_name) = payload.name {
        let name_exists = mock_shares.iter().any(|share| share.name == *new_name && share.id != share_id);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("Share '{}' already exists", new_name),
                code: "NAME_CONFLICT".to_string(),
            }));
        }
    }

    // 6. 执行更新
    let share = mock_shares.iter_mut().find(|s| s.id == share_id);
    match share {
        Some(s) => {
            if let Some(ref new_name) = payload.name {
                s.name = new_name.clone();
            }

            // 6. 更新其他字段
            if let Some(ref desc) = payload.description {
                s.description = Some(desc.clone());
            }

            if let Some(ref protocols) = payload.protocols {
                // 验证协议有效性
                let valid_protocols = vec!["smb", "nfs", "afp"];
                for protocol in protocols {
                    if !valid_protocols.contains(&protocol.as_str()) {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Invalid protocol '{}'. Valid protocols: {}", protocol, valid_protocols.join(", ")),
                            code: "INVALID_PROTOCOL".to_string(),
                        }));
                    }
                }
                s.protocols = protocols.clone();
            }

            if let Some(read_only) = payload.read_only {
                s.read_only = read_only;
            }

            if let Some(guest_access) = payload.guest_access {
                s.guest_access = guest_access;
            }

            if let Some(enabled) = payload.enabled {
                s.enabled = enabled;
                s.status = if enabled { "active".to_string() } else { "inactive".to_string() };
            }

            // 7. 更新时间戳
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();
            s.updated_at = now;

            // 8. 返回更新后的共享信息
            Ok(HttpResponse::Ok().json(UpdateShareResponse {
                success: true,
                message: "Share updated successfully".to_string(),
                data: s.clone(),
            }))
        }
        None => {
            // 9. 共享不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
