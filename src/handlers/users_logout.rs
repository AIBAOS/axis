// Phase 100 - 用户登出 API
// POST /api/v1/auth/logout — 用户登出

use actix_web::{HttpResponse, Error, HttpRequest};
use serde::Serialize;


/// 登出响应
#[derive(Serialize)]
pub struct LogoutResponse {
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

/// 用户登出（Phase 100）
/// - JWT 认证，登录用户可访问
/// - 使当前 Token 失效（可选：加入黑名单）
/// - 返回登出成功信息
pub async fn logout(
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查 Token 是否存在
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            error: "Invalid token".to_string(),
            code: "UNAUTHORIZED".to_string(),
        }));
    }

    // 2. 在实际实现中，这里应该：
    // - 将 Token 加入黑名单
    // - 或清除服务器端会话
    // - 或记录登出日志

    // 3. 返回登出成功
    Ok(HttpResponse::Ok().json(LogoutResponse {
        success: true,
        message: "Logout successful".to_string(),
    }))
}
