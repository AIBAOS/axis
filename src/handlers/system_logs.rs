// Phase 170: 系统日志 API
// GET /api/v1/system/logs — 获取系统日志列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 系统日志查询参数
#[derive(Debug, Deserialize)]
pub struct SystemLogsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub level: Option<String>,
    pub source: Option<String>,
    pub since: Option<String>,
}

/// 日志级别
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

/// 系统日志信息
#[derive(Serialize, Clone)]
pub struct SystemLogEntry {
    pub id: u64,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub details: Option<String>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 系统日志列表响应
#[derive(Serialize)]
pub struct SystemLogsResponse {
    pub success: bool,
    pub data: Vec<SystemLogEntry>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证日志级别
fn validate_log_level(level: &str) -> bool {
    matches!(level.to_uppercase().as_str(), "INFO" | "WARN" | "ERROR")
}

/// 获取系统日志列表（Phase 170）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), limit(默认 50, 最大 200)
/// - 支持过滤：level(INFO/WARN/ERROR), source, since(ISO 时间)
/// - 返回日志列表（时间倒序）
pub async fn get_system_logs(
    req: HttpRequest,
    query: web::Query<SystemLogsQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50).min(200);

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
            error: "Only admin users can view system logs".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证日志级别（如果提供）
    if let Some(ref level) = query.level {
        if !validate_log_level(level) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid log level. Valid levels: INFO, WARN, ERROR".to_string(),
                code: "INVALID_LEVEL".to_string(),
            }));
        }
    }

    // 5. 模拟系统日志数据
    let all_logs = vec![
        SystemLogEntry {
            id: 1,
            timestamp: "2026-03-27T11:00:00Z".to_string(),
            level: "INFO".to_string(),
            source: "system".to_string(),
            message: "System started successfully".to_string(),
            details: None,
        },
        SystemLogEntry {
            id: 2,
            timestamp: "2026-03-27T10:55:00Z".to_string(),
            level: "WARN".to_string(),
            source: "docker".to_string(),
            message: "Container nginx-web high CPU usage detected".to_string(),
            details: Some("CPU usage: 85%".to_string()),
        },
        SystemLogEntry {
            id: 3,
            timestamp: "2026-03-27T10:50:00Z".to_string(),
            level: "ERROR".to_string(),
            source: "network".to_string(),
            message: "Failed to connect to external API".to_string(),
            details: Some("Connection timeout after 30s".to_string()),
        },
        SystemLogEntry {
            id: 4,
            timestamp: "2026-03-27T10:45:00Z".to_string(),
            level: "INFO".to_string(),
            source: "backup".to_string(),
            message: "Daily backup completed successfully".to_string(),
            details: Some("Backup size: 2.5 GB".to_string()),
        },
        SystemLogEntry {
            id: 5,
            timestamp: "2026-03-27T10:40:00Z".to_string(),
            level: "INFO".to_string(),
            source: "system".to_string(),
            message: "User admin logged in".to_string(),
            details: None,
        },
        SystemLogEntry {
            id: 6,
            timestamp: "2026-03-27T10:35:00Z".to_string(),
            level: "WARN".to_string(),
            source: "storage".to_string(),
            message: "Disk usage exceeded 80%".to_string(),
            details: Some("Current usage: 82%".to_string()),
        },
        SystemLogEntry {
            id: 7,
            timestamp: "2026-03-27T10:30:00Z".to_string(),
            level: "INFO".to_string(),
            source: "container".to_string(),
            message: "Container postgres-db started".to_string(),
            details: None,
        },
        SystemLogEntry {
            id: 8,
            timestamp: "2026-03-27T10:25:00Z".to_string(),
            level: "ERROR".to_string(),
            source: "auth".to_string(),
            message: "Failed login attempt for user guest".to_string(),
            details: Some("Invalid password".to_string()),
        },
        SystemLogEntry {
            id: 9,
            timestamp: "2026-03-27T10:20:00Z".to_string(),
            level: "INFO".to_string(),
            source: "system".to_string(),
            message: "Configuration reloaded".to_string(),
            details: None,
        },
        SystemLogEntry {
            id: 10,
            timestamp: "2026-03-27T10:15:00Z".to_string(),
            level: "INFO".to_string(),
            source: "network".to_string(),
            message: "Network interface eth0 configured".to_string(),
            details: Some("IP: 192.168.1.100".to_string()),
        },
    ];

    // 6. 应用过滤
    let filtered_logs: Vec<SystemLogEntry> = all_logs
        .into_iter()
        .filter(|log| {
            // 级别过滤
            if let Some(ref level) = query.level {
                if log.level != level.to_uppercase() {
                    return false;
                }
            }
            // 来源过滤
            if let Some(ref source) = query.source {
                if log.source != *source {
                    return false;
                }
            }
            // 时间过滤
            if let Some(ref since) = query.since {
                if log.timestamp < *since {
                    return false;
                }
            }
            true
        })
        .collect();

    // 7. 应用分页
    let total = filtered_logs.len() as u64;
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(filtered_logs.len());
    
    let logs = if start < filtered_logs.len() {
        filtered_logs[start..end].to_vec()
    } else {
        vec![]
    };

    // 8. 返回系统日志列表
    Ok(HttpResponse::Ok().json(SystemLogsResponse {
        success: true,
        data: logs,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}
