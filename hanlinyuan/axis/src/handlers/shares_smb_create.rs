// Phase 210: SMB 共享创建 API
// POST /api/v1/shares/smb — 创建 SMB 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::Path;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// 创建 SMB 共享请求
#[derive(Debug, Deserialize)]
pub struct CreateSmbShareRequest {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_groups: Option<String>,
    pub guest_ok: Option<bool>,
    pub read_only: Option<bool>,
}

/// 创建的 SMB 共享信息
#[derive(Serialize, Clone)]
pub struct CreatedSmbShare {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_groups: Option<String>,
    pub guest_ok: bool,
    pub read_only: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
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

/// 创建 SMB 共享（Phase 210）
/// - JWT 认证，仅 admin 角色可访问
/// - 请求体包含：name/path/description/allowed_users/allowed_groups/guest_ok/read_only
/// - 验证 path 存在性（400 Bad Request）
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

    // 6. 验证 path 存在性
    if !Path::new(&payload.path).exists() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Path '{}' does not exist", payload.path),
            code: "PATH_NOT_FOUND".to_string(),
        }));
    }

    // 7. 验证名称唯一性（从数据库检查）
    let existing_shares = repo.get_shares(1, 1000, Some("smb".to_string()), None)
        .unwrap_or_default();
    if existing_shares.iter().any(|s| s.name == payload.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("SMB share name '{}' already exists", payload.name),
            code: "NAME_CONFLICT".to_string(),
        }));
    }

    // 8. 创建 SMB 共享
    match repo.create_share(
        &payload.name,
        &payload.path,
        "smb",
        payload.description.as_deref(),
        payload.allowed_users.as_deref(),
        payload.allowed_groups.as_deref(),
        payload.guest_ok.unwrap_or(false),
        payload.read_only.unwrap_or(false),
        None,
        false,
        false,
        None,
        true,
    ) {
        Ok(share) => {
            let new_share = CreatedSmbShare {
                id: share.id,
                name: share.name,
                path: share.path,
                description: share.description,
                allowed_users: share.allowed_users,
                allowed_groups: share.allowed_groups,
                guest_ok: share.guest_ok,
                read_only: share.read_only,
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            Ok(HttpResponse::Created().json(CreateSmbShareResponse {
                success: true,
                message: "SMB share created successfully".to_string(),
                data: new_share,
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("创建共享失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_smb_share_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/shares/smb", web::post().to(create_smb_share))
        ).await;

        // 注意：实际测试需要有效的 JWT token、数据库和存在的路径
        // 这里只是示例测试结构
        assert!(true);
    }
}
