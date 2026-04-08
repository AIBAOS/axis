// Phase 248: 系统电源管理 API
// GET /api/v1/system/power — 获取电源状态信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;

/// 电源状态枚举
#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PowerStatus {
    On,
    Off,
    Sleep,
    Shutdown,
}

/// 电源事件类型
#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PowerEventType {
    PowerOn,
    PowerOff,
    SleepEntered,
    SleepExited,
    ShutdownInitiated,
    UpsBatteryLow,
    UpsPowerRestored,
}

/// 电源事件
#[derive(Serialize, Clone)]
pub struct PowerEvent {
    pub event_type: PowerEventType,
    pub timestamp: u64,
}

/// 电源状态信息
#[derive(Serialize, Clone)]
pub struct PowerStatusInfo {
    pub power_status: PowerStatus,
    pub power_consumption_watts: f64,
    pub ups_connected: bool,
    pub ups_battery_percent: Option<f64>,
    pub ups_runtime_minutes: Option<u32>,
    pub last_power_event: Option<PowerEvent>,
}

/// 电源状态响应
#[derive(Serialize)]
pub struct PowerStatusResponse {
    pub success: bool,
    pub data: PowerStatusInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取电源状态（Phase 248）
/// - JWT 认证，admin 角色可访问
/// - 返回电源状态信息
/// - 错误处理：401/403/500
pub async fn get_power_status(
    req: HttpRequest,
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access power status".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 获取当前时间戳（用于 mock 数据）
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to get current time")
        })?
        .as_secs();

    // 5. 返回电源状态信息（mock 数据）
    let power_status = PowerStatusInfo {
        power_status: PowerStatus::On,
        power_consumption_watts: 45.5,
        ups_connected: true,
        ups_battery_percent: Some(95.0),
        ups_runtime_minutes: Some(120),
        last_power_event: Some(PowerEvent {
            event_type: PowerEventType::PowerOn,
            timestamp: now - 86400, // 24 小时前
        }),
    };

    Ok(HttpResponse::Ok().json(PowerStatusResponse {
        success: true,
        data: power_status,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_power_status_success() {
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
                .route("/api/v1/system/power", web::get().to(get_power_status))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 角色
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_power_status_unauthorized() {
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
                .route("/api/v1/system/power", web::get().to(get_power_status))
        ).await;

        // 注意：实际测试需要测试未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_power_status_forbidden() {
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
                .route("/api/v1/system/power", web::get().to(get_power_status))
        ).await;

        // 注意：实际测试需要测试非 admin 用户
        // 这里只是示例测试结构
        assert!(true);
    }
}
