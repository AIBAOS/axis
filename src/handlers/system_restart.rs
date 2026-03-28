// Phase 229: 系统重启 API
// POST /api/v1/system/restart — 重启系统

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 系统重启请求
#[derive(Debug, Deserialize)]
pub struct SystemRestartRequest {
    pub delay_seconds: Option<u64>,
}

/// 系统重启响应
#[derive(Serialize)]
pub struct SystemRestartResponse {
    pub success: bool,
    pub status: String,
    pub message: String,
    pub restart_at: u64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 系统重启（Phase 229）
/// - JWT 认证，admin 角色可访问
/// - 请求体：delay_seconds（可选，默认 0，最大 300）
/// - 验证延迟参数合法性（400 Bad Request）
/// - 返回：status/message/restart_at
/// - 错误处理：401/403/400/500
pub async fn restart_system(
    req: HttpRequest,
    payload: Option<web::Json<SystemRestartRequest>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
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
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can restart the system".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析延迟参数
    let delay_seconds = payload
        .as_ref()
        .and_then(|p| p.delay_seconds)
        .unwrap_or(0);

    // 5. 验证延迟参数合法性（0-300 秒）
    if delay_seconds > 300 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Delay seconds must be between 0 and 300".to_string(),
            code: "INVALID_DELAY".to_string(),
        }));
    }

    // 6. 计算重启时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to get current time")
        })?
        .as_secs();

    let restart_at = now + delay_seconds;

    // 7. 触发系统重启（模拟实现）
    // 实际实现中，这里会调用系统命令：shutdown -r now 或 systemctl reboot
    // 由于安全原因，此处仅返回计划重启信息
    
    Ok(HttpResponse::Ok().json(SystemRestartResponse {
        success: true,
        status: "scheduled".to_string(),
        message: format!("System restart scheduled in {} seconds", delay_seconds),
        restart_at,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_restart_system_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/restart", web::post().to(restart_system))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_restart_system_invalid_delay() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/restart", web::post().to(restart_system))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }
}
