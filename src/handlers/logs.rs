use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub level: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub since: Option<String>,
    pub until: Option<String>,
}

/// GET /api/v1/logs — 获取系统日志
/// 需要登录用户访问（日志包含敏感信息）
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

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(token.unwrap())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let level_filter = query.level.as_ref().and_then(|s| {
        match s.as_str() {
            "info" => Some(LogLevel::Info),
            "warn" => Some(LogLevel::Warn),
            "error" => Some(LogLevel::Error),
            _ => None,
        }
    });

    let limit = query.limit.unwrap_or(20) as usize;
    let offset = query.offset.unwrap_or(0) as usize;

    let mut mock_logs = vec![
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
    ];

    if let Some(level) = level_filter {
        let level_str = match level {
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        };
        mock_logs.retain(|log| log.level == level_str);
    }

    let total = mock_logs.len();
    let start = offset;
    let end = std::cmp::min(start + limit, total);
    
    let paginated_logs = if start < total {
        mock_logs[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(paginated_logs))
}
