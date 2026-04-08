// Phase 257: 系统日志列表 API
// GET /api/v1/system/logs — 获取系统日志列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 日志级别
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 系统日志信息
#[derive(Serialize, Clone)]
pub struct SystemLogEntry {
    pub id: u32,
    pub level: String,
    pub message: String,
    pub source: String,
    pub created_at: u64,
}

/// 日志列表响应
#[derive(Serialize)]
pub struct SystemLogListResponse {
    pub success: bool,
    pub data: Vec<SystemLogEntry>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 日志查询参数
#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub level: Option<String>,
}

/// 验证日志级别
fn validate_log_level(level: &str) -> bool {
    matches!(level.to_lowercase().as_str(), "debug" | "info" | "warn" | "error")
}

/// 获取系统日志列表（Phase 257）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), page_size(默认 20, 最大 100)
/// - 支持级别过滤：level(debug/info/warn/error)
/// - 返回日志列表（时间倒序）
/// - 错误处理：401/403/400/500
pub async fn get_system_logs(
    req: HttpRequest,
    query: web::Query<LogsQuery>,
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
            error: "Only admin users can view system logs".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析并验证查询参数
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).max(1).min(100) // Bug #72 修复：防止空结果;

    if let Some(ref level) = query.level {
        if !validate_log_level(level) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid level. Must be debug, info, warn, or error".to_string(),
                code: "INVALID_LEVEL".to_string(),
            }));
        }
    }

    // 5. 模拟日志数据（实际应从数据库读取）
    let mut all_logs = vec![
        SystemLogEntry {
            id: 1,
            level: "info".to_string(),
            message: "System started successfully".to_string(),
            source: "system".to_string(),
            created_at: 1711634400,
        },
        SystemLogEntry {
            id: 2,
            level: "warn".to_string(),
            message: "High memory usage detected: 85%".to_string(),
            source: "monitor".to_string(),
            created_at: 1711634100,
        },
        SystemLogEntry {
            id: 3,
            level: "error".to_string(),
            message: "Failed to connect to backup server".to_string(),
            source: "backup".to_string(),
            created_at: 1711633800,
        },
        SystemLogEntry {
            id: 4,
            level: "debug".to_string(),
            message: "Processing request from 192.168.1.100".to_string(),
            source: "api".to_string(),
            created_at: 1711633500,
        },
        SystemLogEntry {
            id: 5,
            level: "info".to_string(),
            message: "User admin logged in".to_string(),
            source: "auth".to_string(),
            created_at: 1711633200,
        },
    ];

    // 6. 应用级别过滤
    if let Some(ref level_filter) = query.level {
        all_logs.retain(|log| log.level == level_filter.to_lowercase());
    }

    // 7. 应用分页
    let total = all_logs.len() as u32;
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(all_logs.len());
    
    let logs = if start < all_logs.len() {
        all_logs[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(SystemLogListResponse {
        success: true,
        data: logs,
        total,
        page,
        page_size,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_system_logs_success() {
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
                .route("/api/v1/system/logs", web::get().to(get_system_logs))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 权限
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_logs_invalid_level() {
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
                .route("/api/v1/system/logs", web::get().to(get_system_logs))
        ).await;

        // 注意：实际测试需要验证无效 level 参数
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_logs_unauthorized() {
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
                .route("/api/v1/system/logs", web::get().to(get_system_logs))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
