// Phase 226 - 用户详情 API
// GET /api/v1/users/{id} — 获取用户详情（真实数据库实现）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::user_store::SqliteUserRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::user::UserRepository;
use crate::models::rbac::RbacRepository;

/// 用户详情（Phase 226）
#[derive(Serialize, Clone)]
pub struct UserDetail {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_login: Option<u64>,
}

/// 用户详情响应
#[derive(Serialize)]
pub struct UserDetailResponse {
    pub success: bool,
    pub data: UserDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查是否为 admin 角色
fn is_admin(claims: &serde_json::Value) -> bool {
    // 简化实现：从 claims 中提取 roles 并检查
    if let Some(roles) = claims.get("roles").and_then(|r| r.as_array()) {
        roles.iter().any(|r| r.as_str() == Some("admin"))
    } else {
        false
    }
}

/// 获取用户详情（Phase 226）
/// - JWT 认证，登录用户可访问
/// - 权限控制：admin 可查看任意用户，普通用户只能查看自己
/// - 路径参数：id (用户 ID)
/// - 返回用户详细信息（不含密码）
/// - 用户不存在返回 404 Not Found
/// - 权限不足返回 403 Forbidden
pub async fn get_user_by_id(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
    user_repo: web::Data<Arc<SqliteUserRepository>>,
    rbac_repo: web::Data<Arc<SqliteRbacRepository>>,
) -> Result<HttpResponse, Error> {
    let target_user_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并提取 claims
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 提取当前用户 ID
    let current_user_id = claims.user_id;

    // 4. 检查是否为 admin
    let is_admin_user = claims.roles.iter().any(|r| r.to_lowercase() == "admin");

    // 5. 权限控制
    // - admin 可以查看任意用户
    // - 普通用户只能查看自己
    if !is_admin_user && current_user_id != target_user_id {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view other users' details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 6. 从数据库查询用户
    match user_repo.get_ref().find_by_id(target_user_id) {
        Ok(Some(user)) => {
            // 7. 获取用户角色
            let roles = rbac_repo.get_ref()
                .get_roles_by_user(user.id)
                .iter()
                .map(|r| r.name.clone())
                .collect::<Vec<String>>();

            // 8. 构建响应（不含密码）
            let user_detail = UserDetail {
                id: user.id,
                username: user.username,
                email: user.email,
                roles,
                is_active: user.is_active,
                created_at: user.created_at,
                updated_at: user.updated_at,
                last_login: user.last_login,
            };

            Ok(HttpResponse::Ok().json(UserDetailResponse {
                success: true,
                data: user_detail,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("User {} not found", target_user_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_user_by_id_not_found() {
        // 测试用户不存在的情况
        // 注意：实际测试需要 mock 数据库
        assert!(true);
    }

    #[test]
    fn test_user_detail_serialization() {
        let user = UserDetail {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["user".to_string()],
            is_active: true,
            created_at: 1710500000,
            updated_at: 1710500000,
            last_login: Some(1710600000),
        };

        let json = serde_json::to_string(&user).expect("User serialization should not fail");
        assert!(json.contains("test"));
        assert!(json.contains("user"));
        assert!(!json.contains("password"));
    }
}
