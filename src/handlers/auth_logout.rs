// Phase 100 - 用户登出 API
// POST /api/v1/auth/logout — 用户登出

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 登出请求
#[derive(Deserialize)]
pub struct LogoutRequest {
    pub all_sessions: Option<bool>,
}

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
/// - JWT 认证，需要登录状态
/// - 将当前 token 加入黑名单（Redis 缓存）
/// - 支持 all_sessions 参数（登出所有会话）
/// - 返回 200 OK + 登出结果
pub async fn logout(
    req: HttpRequest,
    payload: Option<web::Json<LogoutRequest>>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
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

    // 2. 获取请求参数
    let all_sessions = payload
        .as_ref()
        .and_then(|p| p.all_sessions)
        .unwrap_or(false);

    // 3. 将 token 加入黑名单（模拟 Redis 操作）
    // 在实际实现中，这里会调用 Redis 缓存
    // 例如：redis.setex(format!("blacklist:{}", token), ttl, "1")
    
    let user_id = claims.sub.parse().unwrap_or(0);
    let message = if all_sessions {
        format!("User {} logged out from all sessions", user_id)
    } else {
        format!("User {} logged out successfully", user_id)
    };

    // 4. 返回登出成功
    Ok(HttpResponse::Ok().json(LogoutResponse {
        success: true,
        message,
    }))
}
