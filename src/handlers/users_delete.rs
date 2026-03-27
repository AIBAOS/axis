// Phase 37 删除用户 API
// Phase 51: 增强 JWT 认证和 admin 权限校验（使用 JwtService）
// 连接数据库删除用户

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::rbac::RbacRepository;
use crate::database::user_store::SqliteUserRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::user::UserRepository;
use crate::services::jwt_service::JwtService;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除用户（Phase 51 增强版）
/// - JWT 认证，仅 admin 角色可访问
/// - 用户不存在返回 404 Not Found
/// - 非 admin 访问返回 403 Forbidden
/// - 不能删除自己（返回 400 Bad Request）
/// - 删除成功后返回 204 No Content
pub async fn delete_user(
    req: HttpRequest,
    path: web::Path<u64>,
    user_repo: web::Data<SqliteUserRepository>,
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

    // 2. 权限校验 - 仅 admin 角色可删除用户
    let current_user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(current_user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete users".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let target_user_id = path.into_inner();

    // 3. 检查是否尝试删除自己
    if current_user_id == target_user_id {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Cannot delete yourself".to_string(),
            code: "CANNOT_DELETE_SELF".to_string(),
        }));
    }

    // 4. 查询用户是否存在
    let existing_user = user_repo
        .find_by_id(target_user_id)
        .map_err(|e| {
            log::error!("Failed to get user {}: {}", target_user_id, e);
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    let user = match existing_user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("User {} not found", target_user_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    };

    // 5. 删除用户
    user_repo.delete(target_user_id)
        .map_err(|e| {
            log::error!("Failed to delete user {}: {}", target_user_id, e);
            actix_web::error::ErrorInternalServerError(format!("Failed to delete user: {}", e))
        })?;

    log::info!("User {} ({}) deleted by admin", target_user_id, user.username);

    // 6. 返回 204 No Content
    Ok(HttpResponse::NoContent().finish())
}
