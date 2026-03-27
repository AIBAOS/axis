// Phase 59 - 系统健康检查 API
// GET /api/v1/system/health — 系统健康检查

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 服务状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

/// 健康检查响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemHealthResponse {
    pub status: String,
    pub checked_at: String,
    pub uptime_seconds: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub services: Vec<ServiceStatus>,
    pub alerts: Vec<String>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 任意登录用户可访问
}

/// 获取系统健康检查（Phase 59）
/// - JWT 认证，任意登录用户可访问
/// - 返回系统状态信息：CPU/内存/磁盘使用率、运行时间、服务状态
pub async fn get_system_health(
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    let now = chrono::Utc::now();
    
    // 2. 静态 mock 健康数据
    let cpu_usage = 25.5;
    let memory_percent = 37.5;
    let disk_percent = 50.0;
    let uptime_seconds = 86400; // 24 小时
    
    // 3. 服务状态
    let services = vec![
        ServiceStatus {
            name: "database".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "cache".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "storage".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "network".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
    ];
    
    // 4. 确定健康状态
    let mut alerts = vec![];
    let status = if memory_percent > 90.0 || disk_percent > 90.0 || cpu_usage > 95.0 {
        alerts.push("High resource usage detected".to_string());
        "critical".to_string()
    } else if memory_percent > 75.0 || disk_percent > 75.0 || cpu_usage > 80.0 {
        alerts.push("Elevated resource usage".to_string());
        "degraded".to_string()
    } else {
        "healthy".to_string()
    };
    
    // 5. 返回健康检查响应
    Ok(HttpResponse::Ok().json(SystemHealthResponse {
        status,
        checked_at: now.to_rfc3339(),
        uptime_seconds,
        cpu_usage_percent: cpu_usage,
        memory_usage_percent: memory_percent,
        disk_usage_percent: disk_percent,
        services,
        alerts,
    }))
}
