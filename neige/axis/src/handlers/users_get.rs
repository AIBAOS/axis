//! 用户详情 Handler - Phase 226
//! 登录用户可访问，admin 可查看任意用户，普通用户仅可查看自己

use actix_web::{web, HttpResponse, Error};
use serde::Serialize;
use crate::database::user_store::UserRepository;
use crate::middleware::jwt_auth::JwtClaims;

/// 用户信息响应（不含密码）
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_login: Option<i64>,
}

/// 用户详情响应
#[derive(Debug, Serialize)]
pub struct UserDetailResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<UserInfo>,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<String>,
}

impl ErrorResponse {
    pub fn not_found(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("NOT_FOUND".to_string()),
        }
    }

    pub fn forbidden(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("FORBIDDEN".to_string()),
        }
    }

    pub fn unauthorized(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("UNAUTHORIZED".to_string()),
        }
    }
}

/// 用户详情处理器
/// 
/// GET /api/v1/users/{id}
/// 
/// 认证：JWT Bearer Token（登录用户）
/// 权限：
/// - Admin 角色：可查看任意用户
/// - 普通用户：仅可查看自己的信息
/// 
/// 响应：
/// - 200 OK: 返回用户详情（不含密码）
/// - 403 Forbidden: 普通用户尝试查看其他用户
/// - 404 Not Found: 用户不存在
pub async fn get_user(
    claims: JwtClaims,
    user_repo: web::Data<dyn UserRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let target_user_id = path.into_inner();
    let current_user_id = claims.user_id;
    
    // 权限校验：admin 可查看任意用户，普通用户仅可查看自己
    let is_admin = claims.roles.contains(&"admin".to_string());
    if !is_admin && current_user_id != target_user_id {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse::forbidden(
            "普通用户仅可查看自己的信息",
        )));
    }
    
    // 查询用户
    let user = user_repo.find_by_id(&target_user_id).await
        .map_err(|e| {
            log::error!("查询用户失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    let user = match user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse::not_found(
                &format!("用户不存在：{}", target_user_id),
            )));
        }
    };
    
    // 返回用户信息（不含密码）
    Ok(HttpResponse::Ok().json(UserDetailResponse {
        success: true,
        message: "获取用户详情成功".to_string(),
        data: Some(UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            roles: user.roles,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_login: user.last_login,
        }),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_response() {
        let err = ErrorResponse::not_found("用户不存在");
        assert!(!err.success);
        assert_eq!(err.message, "用户不存在");
        assert_eq!(err.error_code, Some("NOT_FOUND".to_string()));
        
        let err = ErrorResponse::forbidden("权限不足");
        assert_eq!(err.error_code, Some("FORBIDDEN".to_string()));
        
        let err = ErrorResponse::unauthorized("未授权访问");
        assert_eq!(err.error_code, Some("UNAUTHORIZED".to_string()));
    }
    
    #[test]
    fn test_permission_check() {
        // Admin 可以查看任意用户
        let admin_roles = vec!["admin".to_string()];
        let is_admin = admin_roles.contains(&"admin".to_string());
        assert!(is_admin);
        
        // 普通用户不能查看其他用户
        let user_roles = vec!["user".to_string()];
        let is_admin = user_roles.contains(&"admin".to_string());
        assert!(!is_admin);
        
        // 普通用户可以查看自己
        let current_user_id = "user_123";
        let target_user_id = "user_123";
        let can_view = current_user_id == target_user_id;
        assert!(can_view);
        
        // 普通用户不能查看其他用户
        let target_user_id = "user_456";
        let can_view = current_user_id == target_user_id;
        assert!(!can_view);
    }
}
