// Phase 226: 用户详情 API（真实数据库实现）
// GET /api/v1/users/{id} — 获取用户详情
// 权限控制：admin 可查看任意用户，普通用户只能查看自己

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::user_store::SqliteUserRepository;
use crate::models::user::UserRepository;

/// 用户详情信息（Phase 226，不含密码）
#[derive(Serialize, Clone, Debug)]
pub struct UserDetail {
    /// 用户 ID
    pub id: u64,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 角色列表
    pub roles: Vec<String>,
    /// 是否激活
    pub is_active: bool,
    /// 创建时间（Unix 时间戳）
    pub created_at: u64,
    /// 更新时间（Unix 时间戳）
    pub updated_at: u64,
    /// 最后登录时间（Unix 时间戳）
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

/// 获取用户详情（Phase 226）
/// 
/// # 接口说明
/// - GET /api/v1/users/{id}
/// - JWT 认证，登录用户可访问
/// 
/// # 权限控制
/// - admin 角色：可查看任意用户
/// - 普通用户：只能查看自己
/// 
/// # 响应
/// - 200 OK: 成功返回用户详情
/// - 401 Unauthorized: 未认证或 Token 无效
/// - 403 Forbidden: 无权限查看该用户
/// - 404 Not Found: 用户不存在
pub async fn get_user_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
    user_repo: web::Data<Arc<SqliteUserRepository>>,
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

    // 3. 获取当前用户 ID
    let current_user_id: u64 = claims.sub.parse()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID in token"))?;

    // 4. 检查权限：admin 可查看任意用户，普通用户只能查看自己
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin && current_user_id != target_user_id {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "You can only view your own user details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 5. 从数据库查询目标用户
    match user_repo.find_by_id(target_user_id) {
        Ok(Some(user)) => {
            // 6. 转换为响应格式（不包含密码）
            let user_detail = UserDetail {
                id: user.id,
                username: user.username,
                email: user.email,
                roles: user.roles,
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
            log::error!("Failed to get user {}: {}", target_user_id, e);
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

    /// 测试 UserDetail 结构体创建
    #[test]
    fn test_user_detail_creation() {
        let detail = UserDetail {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            roles: vec!["admin".to_string()],
            is_active: true,
            created_at: 1710500000,
            updated_at: 1710600000,
            last_login: Some(1710700000),
        };

        assert_eq!(detail.id, 1);
        assert_eq!(detail.username, "admin");
        assert_eq!(detail.roles.len(), 1);
        assert!(detail.is_active);
        assert!(detail.last_login.is_some());
    }

    /// 测试权限检查逻辑
    #[test]
    fn test_permission_check() {
        // admin 可以查看任意用户
        let is_admin = true;
        let current_user_id: u64 = 1;
        let target_user_id: u64 = 2;
        let can_view = is_admin || current_user_id == target_user_id;
        assert!(can_view);

        // 普通用户只能查看自己
        let is_admin = false;
        let current_user_id: u64 = 1;
        let target_user_id: u64 = 1;
        let can_view = is_admin || current_user_id == target_user_id;
        assert!(can_view);

        // 普通用户不能查看其他用户
        let is_admin = false;
        let current_user_id: u64 = 1;
        let target_user_id: u64 = 2;
        let can_view = is_admin || current_user_id == target_user_id;
        assert!(!can_view);
    }

    /// 测试响应序列化
    #[test]
    fn test_response_serialization() {
        let detail = UserDetail {
            id: 1,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["user".to_string()],
            is_active: true,
            created_at: 1710500000,
            updated_at: 1710600000,
            last_login: None,
        };

        let response = UserDetailResponse {
            success: true,
            data: detail,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"username\":\"test\""));
        assert!(json.contains("\"last_login\":null"));
    }

    /// 测试错误响应
    #[test]
    fn test_error_response() {
        let error = ErrorResponse {
            success: false,
            error: "User not found".to_string(),
            code: "NOT_FOUND".to_string(),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"success\":false"));
        assert!(json.contains("\"code\":\"NOT_FOUND\""));
    }
}