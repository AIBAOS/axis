//! 更新用户 Handler - Phase 103
//! 仅 admin 角色可访问，支持更新邮箱和角色

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::database::user_store::UserRepository;
use crate::middleware::jwt_auth::JwtClaims;
use crate::services::jwt_service::JwtService;

/// 更新用户请求体
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    /// 邮箱（可选）
    pub email: Option<String>,
    /// 角色（可选）
    pub role: Option<String>,
}

/// 用户信息响应
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 更新用户响应
#[derive(Debug, Serialize)]
pub struct UpdateUserResponse {
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
    pub fn bad_request(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("BAD_REQUEST".to_string()),
        }
    }

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
}

/// 邮箱格式校验
fn validate_email(email: &str) -> bool {
    // 简单邮箱格式校验
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// 更新用户处理器
/// 
/// PUT /api/v1/users/{id}
/// 
/// 权限：仅 admin 角色可访问
/// 请求体：
/// ```json
/// {
///   "email": "optional@example.com",
///   "role": "admin"
/// }
/// ```
/// 
/// 响应：
/// - 200 OK: 更新成功，返回用户信息
/// - 400 Bad Request: 邮箱格式错误或角色无效
/// - 403 Forbidden: 非 admin 角色
/// - 404 Not Found: 用户不存在
pub async fn update_user(
    claims: JwtClaims,
    user_repo: web::Data<dyn UserRepository>,
    jwt_service: web::Data<JwtService>,
    path: web::Path<String>,
    req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();
    
    // 权限校验：仅 admin 角色可更新用户
    if !claims.roles.contains(&"admin".to_string()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse::forbidden(
            "仅 admin 角色可更新用户信息",
        )));
    }
    
    // 验证请求参数
    let mut update_email: Option<String> = None;
    let mut update_role: Option<String> = None;
    
    if let Some(ref email) = req.email {
        if !validate_email(email) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                "邮箱格式无效",
            )));
        }
        update_email = Some(email.clone());
    }
    
    if let Some(ref role) = req.role {
        // 验证角色有效性（检查是否为预定义角色）
        let valid_roles = ["admin", "user", "moderator", "guest"];
        if !valid_roles.contains(&role.as_str()) {
            // 进一步：可以从数据库查询有效角色
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                &format!("无效的角色：{}", role),
            )));
        }
        update_role = Some(role.clone());
    }
    
    // 至少需要一个更新字段
    if update_email.is_none() && update_role.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
            "至少需要提供一个更新字段（email 或 role）",
        )));
    }
    
    // 查询用户是否存在
    let user = user_repo.find_by_id(&user_id).await
        .map_err(|e| {
            log::error!("查询用户失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    let user = match user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse::not_found(
                &format!("用户不存在：{}", user_id),
            )));
        }
    };
    
    // 更新用户信息
    let updated_user = user_repo.update_user(
        &user_id,
        update_email.as_deref(),
        update_role.as_deref(),
    ).await
    .map_err(|e| {
        log::error!("更新用户失败：{}", e);
        actix_web::error::ErrorInternalServerError("数据库更新失败")
    })?;
    
    let updated_user = match updated_user {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse::not_found(
                "更新后无法获取用户信息",
            )));
        }
    };
    
    // 记录审计日志（可选）
    log::info!(
        "用户 {} 被 admin {} 更新：email={:?}, role={:?}",
        user_id,
        claims.user_id,
        update_email,
        update_role,
    );
    
    Ok(HttpResponse::Ok().json(UpdateUserResponse {
        success: true,
        message: "用户信息更新成功".to_string(),
        data: Some(UserInfo {
            id: updated_user.id,
            username: updated_user.username,
            email: updated_user.email,
            role: updated_user.roles.first().cloned().unwrap_or_else(|| "user".to_string()),
            created_at: updated_user.created_at,
            updated_at: updated_user.updated_at,
        }),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user.name+tag@domain.co.uk"));
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@example.com"));
        assert!(!validate_email("test@"));
        assert!(!validate_email("test@example"));
    }
    
    #[test]
    fn test_error_response() {
        let err = ErrorResponse::bad_request("测试错误");
        assert!(!err.success);
        assert_eq!(err.message, "测试错误");
        assert_eq!(err.error_code, Some("BAD_REQUEST".to_string()));
        
        let err = ErrorResponse::not_found("用户不存在");
        assert_eq!(err.error_code, Some("NOT_FOUND".to_string()));
        
        let err = ErrorResponse::forbidden("权限不足");
        assert_eq!(err.error_code, Some("FORBIDDEN".to_string()));
    }
}
