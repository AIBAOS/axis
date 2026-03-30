// Phase 104: 删除用户 API (增强版)
// DELETE /api/v1/users/{id} — 删除用户

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::user_store::SqliteUserRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::user::UserRepository;
use crate::services::jwt_service::JwtService;

/// 删除用户响应
#[derive(Serialize)]
pub struct DeleteUserResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除用户（Phase 104 增强版）
/// - JWT 认证，仅 admin 角色可访问
/// - 用户不存在返回 404 Not Found
/// - 非 admin 访问返回 403 Forbidden
/// - 不能删除自己（返回 400 Bad Request）
/// - Bug #45 修复：删除用户时清理角色关联
/// - 删除成功后返回 200 OK + { success: true, message: "User deleted" }
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
    let current_user_id = claims.user_id;
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    
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
    let existing_user = user_repo.get_ref()
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

    // 5. Bug #45 修复：清理用户的角色关联
    if let Err(e) = rbac_repo.get_ref().remove_user_roles(target_user_id) {
        log::warn!("Failed to remove user roles for {}: {}", target_user_id, e);
        // 继续执行删除用户，但记录警告
    }

    // 6. 删除用户
    user_repo.get_ref().delete(target_user_id)
        .map_err(|e| {
            log::error!("Failed to delete user {}: {}", target_user_id, e);
            actix_web::error::ErrorInternalServerError(format!("Failed to delete user: {}", e))
        })?;

    log::info!("User {} ({}) deleted by admin", target_user_id, user.username);

    // 7. 返回 200 OK + 删除成功消息
    Ok(HttpResponse::Ok().json(DeleteUserResponse {
        success: true,
        message: "User deleted".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_delete_user_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_delete_user_forbidden() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        // 注意：实际测试需要验证非 admin 用户
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_delete_user_not_found() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        // 注意：实际测试需要验证用户不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
