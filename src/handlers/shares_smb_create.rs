// Phase 201: SMB 共享创建 API (数据库版本)
// POST /api/v1/shares/smb — 创建 SMB 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// 创建 SMB 共享请求
#[derive(Debug, Deserialize)]
pub struct CreateSmbShareRequest {
    pub name: String,
    pub path: String,
    pub comment: Option<String>,
    pub read_only: Option<bool>,
    pub guest_access: Option<bool>,
    pub browseable: Option<bool>,
    pub valid_users: Option<Vec<String>>,
    pub invalid_users: Option<Vec<String>>,
}

/// 创建的 SMB 共享信息
#[derive(Serialize, Clone)]
pub struct CreatedSmbShare {
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

/// 创建 SMB 共享响应
#[derive(Serialize)]
pub struct CreateSmbShareResponse {
    pub success: bool,
    pub message: String,
    pub data: CreatedSmbShare,
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

/// 创建 SMB 共享（Phase 201 - 数据库版本）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库创建
/// - 请求体包含：name/path/comment/read_only/guest_access/browseable/valid_users/invalid_users
/// - 验证名称唯一性（409 Conflict）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 创建成功返回 201 Created + 共享详情
pub async fn create_smb_share(
    req: HttpRequest,
    payload: web::Json<CreateSmbShareRequest>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
) -> Result<HttpResponse, Error> {
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
            error: "Only admin users can create SMB shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证共享名称格式
    if !validate_share_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证共享路径格式
    if !validate_share_path(&payload.path) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid share path. Must start with / and be <= 256 chars".to_string(),
            code: "INVALID_PATH".to_string(),
        }));
    }

    // 6. 使用数据库创建 SMB 共享
    match repo.create_share(&payload.name, &payload.path, "smb") {
        Ok(share) => {
            let new_share = CreatedSmbShare {
                id: share.id as u64,
                name: share.name,
                path: share.path,
                comment: payload.comment.clone().unwrap_or_default(),
                read_only: payload.read_only.unwrap_or(false),
                guest_access: payload.guest_access.unwrap_or(false),
                browseable: payload.browseable.unwrap_or(true),
                valid_users: payload.valid_users.clone().unwrap_or_default(),
                invalid_users: payload.invalid_users.clone().unwrap_or_default(),
                enabled: share.status == "active",
                status: share.status,
                created_at: share.created_at.to_string(),
                updated_at: share.updated_at.to_string(),
            };

            // 7. 返回创建成功
            Ok(HttpResponse::Created().json(CreateSmbShareResponse {
                success: true,
                message: "SMB share created successfully".to_string(),
                data: new_share,
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("创建 SMB 共享失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}
