// Phase 173: 系统日志导出 API
// POST /api/v1/system/logs/export — 导出系统日志

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 导出日志请求
#[derive(Debug, Deserialize)]
pub struct ExportLogsRequest {
    pub format: String,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub level: Option<String>,
    pub source: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
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

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证导出格式
fn validate_format(format: &str) -> bool {
    matches!(format.to_lowercase().as_str(), "csv" | "json")
}

/// 验证日志级别
fn validate_log_level(level: &str) -> bool {
    matches!(level.to_uppercase().as_str(), "INFO" | "WARN" | "ERROR")
}

/// 将日志转换为 CSV 格式
fn logs_to_csv(logs: &[SystemLogEntry]) -> String {
    let mut csv = String::from("id,timestamp,level,source,message,details\n");
    for log in logs {
        let details = log.details.as_deref().unwrap_or("");
        csv.push_str(&format!(
            "{},{},{},{},\"{}\",\"{}\"\n",
            log.id, log.timestamp, log.level, log.source, log.message, details
        ));
    }
    csv
}

/// 将日志转换为 JSON 格式
fn logs_to_json(logs: &[SystemLogEntry]) -> String {
    serde_json::to_string_pretty(logs).unwrap_or_default()
}

/// 导出系统日志（Phase 173）
/// - JWT 认证，admin 角色可访问
/// - 支持导出格式：CSV/JSON
/// - 支持按时间范围、日志级别、来源筛选
/// - 支持分页
/// - 返回文件下载
pub async fn export_system_logs(
    req: HttpRequest,
    payload: web::Json<ExportLogsRequest>,
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
            error: "Only admin users can export system logs".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证导出格式
    if !validate_format(&payload.format) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid format. Valid formats: csv, json".to_string(),
            code: "INVALID_FORMAT".to_string(),
        }));
    }

    // 5. 验证日志级别（如果提供）
    if let Some(ref level) = payload.level {
        if !validate_log_level(level) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid log level. Valid levels: INFO, WARN, ERROR".to_string(),
                code: "INVALID_LEVEL".to_string(),
            }));
        }
    }

    // 6. 模拟系统日志数据
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
    ];

    // 7. 应用筛选
    let filtered_logs: Vec<SystemLogEntry> = all_logs
        .into_iter()
        .filter(|log| {
            // 级别过滤
            if let Some(ref level) = payload.level {
                if log.level != level.to_uppercase() {
                    return false;
                }
            }
            // 来源过滤
            if let Some(ref source) = payload.source {
                if log.source != *source {
                    return false;
                }
            }
            // 时间范围过滤
            if let Some(ref start_time) = payload.start_time {
                if log.timestamp < *start_time {
                    return false;
                }
            }
            if let Some(ref end_time) = payload.end_time {
                if log.timestamp > *end_time {
                    return false;
                }
            }
            true
        })
        .collect();

    // 8. 应用分页
    let page = payload.page.unwrap_or(1);
    let limit = payload.limit.unwrap_or(100).min(1000);
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(filtered_logs.len());
    
    let logs = if start < filtered_logs.len() {
        filtered_logs[start..end].to_vec()
    } else {
        vec![]
    };

    // 9. 生成导出文件
    let format = payload.format.to_lowercase();
    let (content, content_type) = if format == "csv" {
        (logs_to_csv(&logs), "text/csv")
    } else {
        (logs_to_json(&logs), "application/json")
    };

    // 10. 返回文件下载
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("system_logs_{}.{}", timestamp, format);

    Ok(HttpResponse::Ok()
        .content_type(content_type)
        .insert_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .body(content))
}
