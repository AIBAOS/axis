// Phase 226: 用户详情 API
// GET /api/v1/users/{id} — 获取用户详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::user_store::SqliteUserRepository;

/// 用户详情响应
#[derive(Serialize)]
pub struct UserDetailResponse {
    pub success: bool,
    pub data: UserInfo,
}

/// 用户信息（响应用，不包含敏感信息）
#[derive(Serialize, Clone)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_login: Option<u64>,
    pub storage_quota: Option<u64>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取用户详情（Phase 226）
/// - JWT 认证，admin 角色可访问
/// - 验证用户 ID 存在性（404 Not Found）
/// - 返回用户详情（不含密码）
pub async fn get_user_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
    user_repo: web::Data<Arc<SqliteUserRepository>>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view user details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询用户详情
    match user_repo.find_by_id(user_id) {
        Ok(Some(user)) => {
            // 5. 返回用户详情（不含密码）
            let user_info = UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
                roles: user.roles,
                permissions: user.permissions,
                is_active: user.is_active,
                created_at: user.created_at,
                updated_at: user.updated_at,
                last_login: user.last_login,
                storage_quota: user.storage_quota,
            };

            Ok(HttpResponse::Ok().json(UserDetailResponse {
                success: true,
                data: user_info,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("User {} not found", user_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询用户详情失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
