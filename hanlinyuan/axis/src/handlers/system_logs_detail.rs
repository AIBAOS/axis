// Phase 172: 系统日志详情 API
// GET /api/v1/system/logs/{id} — 获取系统日志详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 系统日志详情信息
#[derive(Serialize, Clone)]
pub struct SystemLogDetail {
    pub id: u64,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
    pub details: Option<String>,
    pub context: Option<String>,
    pub user: Option<String>,
}

/// 系统日志详情响应
#[derive(Serialize)]
pub struct SystemLogDetailResponse {
    pub success: bool,
    pub data: SystemLogDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取系统日志详情（Phase 172）
/// - JWT 认证，admin 角色可访问
/// - 验证日志 ID 存在性（404 Not Found）
/// - 返回日志详情
pub async fn get_system_log_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let log_id = path.into_inner();

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
            error: "Only admin users can view system log details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟系统日志数据
    let mock_logs = vec![
        SystemLogDetail {
            id: 1,
            timestamp: "2026-03-27T11:00:00Z".to_string(),
            level: "INFO".to_string(),
            source: "system".to_string(),
            message: "System started successfully".to_string(),
            details: None,
            context: Some("System boot sequence completed".to_string()),
            user: Some("system".to_string()),
        },
        SystemLogDetail {
            id: 2,
            timestamp: "2026-03-27T10:55:00Z".to_string(),
            level: "WARN".to_string(),
            source: "docker".to_string(),
            message: "Container nginx-web high CPU usage detected".to_string(),
            details: Some("CPU usage: 85%".to_string()),
            context: Some("Threshold: 80%".to_string()),
            user: None,
        },
        SystemLogDetail {
            id: 3,
            timestamp: "2026-03-27T10:50:00Z".to_string(),
            level: "ERROR".to_string(),
            source: "network".to_string(),
            message: "Failed to connect to external API".to_string(),
            details: Some("Connection timeout after 30s".to_string()),
            context: Some("API endpoint: https://api.example.com".to_string()),
            user: None,
        },
        SystemLogDetail {
            id: 4,
            timestamp: "2026-03-27T10:45:00Z".to_string(),
            level: "INFO".to_string(),
            source: "backup".to_string(),
            message: "Daily backup completed successfully".to_string(),
            details: Some("Backup size: 2.5 GB".to_string()),
            context: Some("Destination: /srv/backups/daily".to_string()),
            user: Some("backup-service".to_string()),
        },
        SystemLogDetail {
            id: 5,
            timestamp: "2026-03-27T10:40:00Z".to_string(),
            level: "INFO".to_string(),
            source: "system".to_string(),
            message: "User admin logged in".to_string(),
            details: None,
            context: Some("IP: 192.168.1.50".to_string()),
            user: Some("admin".to_string()),
        },
    ];

    // 5. 查找日志
    let log = mock_logs.into_iter().find(|l| l.id == log_id);

    // 6. 验证日志存在性
    match log {
        Some(log) => {
            // 7. 返回日志详情
            Ok(HttpResponse::Ok().json(SystemLogDetailResponse {
                success: true,
                data: log,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("System log {} not found", log_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
