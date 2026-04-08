// 日志管理处理器（Phase 29）
// 包含：日志列表、导出、清理接口

use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

use crate::services::jwt_service::JwtService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct LogListResponse {
    pub success: bool,
    pub logs: Vec<LogEntry>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct ExportResponse {
    pub success: bool,
    pub message: String,
    pub filename: String,
    pub format: String,
    pub bytes: u64,
}

#[derive(Debug, Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
    pub deleted_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub level: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteParams {
    pub level: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
}

// 日志根目录
fn get_log_dir() -> String {
    "/var/log/axis".to_string()
}

// 模拟读取日志文件内容
fn read_log_file(_path: &str) -> Vec<LogEntry> {
    let logs = vec![
        LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "info".to_string(),
            source: "system".to_string(),
            message: "System started successfully".to_string(),
            metadata: None,
        },
        LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "warn".to_string(),
            source: "storage".to_string(),
            message: "Low disk space warning".to_string(),
            metadata: Some(serde_json::json!({"free_space": "10GB"})),
        },
        LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: "error".to_string(),
            source: "database".to_string(),
            message: "Connection timeout".to_string(),
            metadata: Some(serde_json::json!({"timeout_ms": 5000})),
        },
    ];
    logs
}

// 读取所有日志文件
fn read_all_logs() -> Vec<LogEntry> {
    let mut all_logs: Vec<LogEntry> = Vec::new();
    let log_dir = get_log_dir();
    
    if let Ok(entries) = fs::read_dir(&log_dir) {
        for entry in entries.flatten() {
            let _path = entry.path();
            if _path.is_file() {
                let _ = _path.to_string_lossy();
                // 模拟读取：实际应解析日志文件
                all_logs.extend(read_log_file(&_path.to_string_lossy()));
            }
        }
    }
    
    all_logs
}

// 解析时间字符串为 UTC timestamp
fn parse_time(time_str: &str) -> Option<i64> {
    // 简化实现：实际应使用 chrono 解析 ISO 8601
    if time_str.is_empty() {
        return Some(chrono::Utc::now().timestamp());
    }
    time_str.parse().ok()
}

pub async fn get_logs(
    req: HttpRequest,
    query: web::Query<QueryParams>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(t) = token {
        if jwt_service.validate_token(t).is_err() {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "success": false,
                "error": "Invalid or expired token",
                "code": "UNAUTHORIZED"
            })));
        }
    } else {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "error": "Missing Authorization header",
            "code": "UNAUTHORIZED"
        })));
    }

    let all_logs = read_all_logs();
    
    // 过滤级别
    let filtered: Vec<LogEntry> = if let Some(ref level) = query.level {
        let level_lower = level.to_lowercase();
        all_logs.into_iter()
            .filter(|log| {
                let log_level = log.level.to_lowercase();
                log_level == level_lower
            })
            .collect()
    } else {
        all_logs
    };
    
    // 过滤时间范围
    let filtered: Vec<LogEntry> = if let Some(from) = query.from.as_ref().or(query.since.as_ref()) {
        if let Some(from_ts) = parse_time(from) {
            filtered.into_iter()
                .filter(|log| {
                    if let Ok(ts) = log.timestamp.parse::<i64>() {
                        ts >= from_ts
                    } else {
                        log.timestamp >= from.to_string()
                    }
                })
                .collect()
        } else {
            filtered
        }
    } else {
        filtered
    };
    
    let filtered: Vec<LogEntry> = if let Some(to) = query.to.as_ref().or(query.until.as_ref()) {
        if let Some(to_ts) = parse_time(to) {
            filtered.into_iter()
                .filter(|log| {
                    if let Ok(ts) = log.timestamp.parse::<i64>() {
                        ts <= to_ts
                    } else {
                        log.timestamp <= to.to_string()
                    }
                })
                .collect()
        } else {
            filtered
        }
    } else {
        filtered
    };
    
    let limit = query.limit.unwrap_or(20).max(1); // Bug #71 修复：防止除零错误
    let offset = query.offset.unwrap_or(0);
    let page = offset / limit;
    let page_size = limit;
    let total = filtered.len() as u32;
    let start_offset = (page as u32) * page_size;
    
    let paginated = filtered
        .into_iter()
        .skip(start_offset as usize)
        .take(page_size as usize)
        .collect::<Vec<LogEntry>>();
    
    Ok(HttpResponse::Ok().json(LogListResponse {
        success: true,
        logs: paginated,
        total,
        page: page as u32 + 1,
        page_size,
        has_more: (start_offset + page_size as u32) < total,
    }))
}

pub async fn export_logs(
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(t) = token {
        if jwt_service.validate_token(t).is_err() {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "success": false,
                "error": "Invalid or expired token",
                "code": "UNAUTHORIZED"
            })));
        }
    } else {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "error": "Missing Authorization header",
            "code": "UNAUTHORIZED"
        })));
    }

    let format = query.get("format").map(|s| s.as_str()).unwrap_or("csv");
    let all_logs = read_all_logs();
    
    let content = match format {
        "csv" => {
            let mut csv = "timestamp,level,source,message\n".to_string();
            for log in &all_logs {
                csv.push_str(&format!(
                    "{},{},{},\"{}\"\n",
                    log.timestamp,
                    log.level,
                    log.source,
                    log.message.replace("\"", "\"\"")
                ));
            }
            csv
        }
        "txt" => {
            let mut txt = String::new();
            for log in &all_logs {
                txt.push_str(&format!(
                    "[{}] {} - {}: {}\n",
                    log.timestamp, log.level, log.source, log.message
                ));
            }
            txt
        }
        _ => return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Invalid format. Use 'csv' or 'txt'"
        }))),
    };
    
    let _bytes = content.len() as u64;
    
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/plain"))
        .insert_header(("Content-Disposition", format!("attachment; filename=\"axis-logs-{}.{}\"", chrono::Utc::now().format("%Y%m%d"), format)))
        .body(content))
}

pub async fn delete_logs(
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if let Some(t) = token {
        if jwt_service.validate_token(t).is_err() {
            return Ok(HttpResponse::Unauthorized().json(json!({
                "success": false,
                "error": "Invalid or expired token",
                "code": "UNAUTHORIZED"
            })));
        }
    } else {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "error": "Missing Authorization header",
            "code": "UNAUTHORIZED"
        })));
    }

    let level = query.get("level").map(|s| s.as_str());
    let before = query.get("before");
    let after = query.get("after");
    
    let deleted_count = if let Some(_lvl) = level {
        // 删除指定级别日志
        0 // 模拟删除
    } else if before.is_some() || after.is_some() {
        // 删除时间范围日志
        0 // 模拟删除
    } else {
        // 删除所有日志
        0 // 模拟删除
    };
    
    // 实际实现应调用 fs::remove_file() 删除日志文件
    
    Ok(HttpResponse::Ok().json(DeleteResponse {
        success: true,
        message: format!("Deleted {} log entries", deleted_count),
        deleted_count,
    }))
}
