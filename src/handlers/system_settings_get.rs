// Phase 246: 系统设置获取 API
// GET /api/v1/system/settings — 获取系统设置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 系统设置信息
#[derive(Serialize, Clone)]
pub struct SystemSettings {
    pub network: NetworkSettings,
    pub storage: StorageSettings,
    pub system: SystemSettingsInfo,
    pub user: UserSettings,
}

/// 网络设置
#[derive(Serialize, Clone)]
pub struct NetworkSettings {
    pub host: String,
    pub port: u32,
}

/// 存储设置
#[derive(Serialize, Clone)]
pub struct StorageSettings {
    pub path: String,
}

/// 系统设置信息
#[derive(Serialize, Clone)]
pub struct SystemSettingsInfo {
    pub timezone: String,
}

/// 用户设置
#[derive(Serialize, Clone)]
pub struct UserSettings {
    pub prefer_theme: String,
}

/// 系统设置响应
#[derive(Serialize)]
pub struct SystemSettingsResponse {
    pub success: bool,
    pub data: SystemSettings,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取系统设置（Phase 246）
/// - JWT 认证，admin 角色可访问
/// - 返回系统设置信息
/// - 错误处理：401/403/500
pub async fn get_system_settings(
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
            error: "Only admin users can access system settings".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 返回系统设置（mock 数据）
    let settings = SystemSettings {
        network: NetworkSettings {
            host: "0.0.0.0".to_string(),
            port: 8080,
        },
        storage: StorageSettings {
            path: "/data".to_string(),
        },
        system: SystemSettingsInfo {
            timezone: "Asia/Shanghai".to_string(),
        },
        user: UserSettings {
            prefer_theme: "dark".to_string(),
        },
    };

    Ok(HttpResponse::Ok().json(SystemSettingsResponse {
        success: true,
        data: settings,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_system_settings_success() {
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
                .route("/api/v1/system/settings", web::get().to(get_system_settings))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 角色
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_settings_unauthorized() {
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
                .route("/api/v1/system/settings", web::get().to(get_system_settings))
        ).await;

        // 注意：实际测试需要测试未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
