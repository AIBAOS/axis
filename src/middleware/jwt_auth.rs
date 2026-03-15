// JWT 认证辅助模块（actix-web 4.x 兼容）
// 直接在处理器中验证 JWT，无需中间件

use actix_web::{web, HttpResponse, Error};
use actix_web::error::ErrorUnauthorized;
use crate::services::jwt_service::JwtService;

/// 从 Authorization 头提取 JWT Token
pub fn extract_token(auth_header: &str) -> Result<String, Error> {
    let parts: Vec<&str> = auth_header.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err(ErrorUnauthorized("Invalid Authorization header format"));
    }
    Ok(parts[1].to_string())
}

/// 验证 JWT Token（在处理器中调用）
pub async fn verify_jwt(
    jwt_service: web::Data<JwtService>,
    auth_header: Option<String>,
) -> Result<(), Error> {
    let token = match auth_header {
        Some(h) => extract_token(&h)?,
        None => return Err(ErrorUnauthorized("Missing Authorization header")),
    };

    match jwt_service.validate_token(&token) {
        Ok(_) => Ok(()),
        Err(_) => Err(ErrorUnauthorized("Invalid or expired token")),
    }
}
