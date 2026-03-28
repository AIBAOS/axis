// Phase 155: SMB 共享详情 API
// GET /api/v1/shares/smb/{id} — 获取 SMB 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// SMB 共享详情信息
#[derive(Serialize, Clone)]
pub struct SmbShareDetail {
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

/// SMB 共享详情响应
#[derive(Serialize)]
pub struct SmbShareDetailResponse {
    pub success: bool,
    pub data: SmbShareDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 SMB 共享详情（Phase 155）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享 ID 存在性（404 Not Found）
/// - 返回 SMB 共享完整详情
pub async fn get_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
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
            error: "Only admin users can view SMB share details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟 SMB 共享数据
    let mock_shares = vec![
        SmbShareDetail {
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
        SmbShareDetail {
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
        SmbShareDetail {
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

    // 5. 查找共享
    let share = mock_shares.into_iter().find(|s| s.id == share_id);

    // 6. 验证共享存在性
    match share {
        Some(share) => {
            // 7. 返回共享详情
            Ok(HttpResponse::Ok().json(SmbShareDetailResponse {
                success: true,
                data: share,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("SMB share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
