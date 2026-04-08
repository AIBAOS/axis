// Phase 175: 系统告警详情 API
// GET /api/v1/system/alerts/{id} — 获取系统告警详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 系统告警详情信息
#[derive(Serialize, Clone)]
pub struct SystemAlertDetail {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub status: String,
    pub source: String,
    pub created_at: String,
    pub acknowledged_at: Option<String>,
    pub acknowledged_by: Option<String>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub metadata: Option<String>,
}

/// 系统告警详情响应
#[derive(Serialize)]
pub struct SystemAlertDetailResponse {
    pub success: bool,
    pub data: SystemAlertDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取系统告警详情（Phase 175）
/// - JWT 认证，admin 角色可访问
/// - 验证告警 ID 存在性（404 Not Found）
/// - 返回告警详情
pub async fn get_system_alert_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let alert_id = path.into_inner();

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
            error: "Only admin users can view system alert details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟系统告警数据
    let mock_alerts = vec![
        SystemAlertDetail {
            id: 1,
            title: "High CPU Usage".to_string(),
            message: "CPU usage exceeded 90% for more than 5 minutes".to_string(),
            severity: "critical".to_string(),
            status: "active".to_string(),
            source: "system".to_string(),
            created_at: "2026-03-27T12:30:00Z".to_string(),
            acknowledged_at: None,
            acknowledged_by: None,
            resolved_at: None,
            resolved_by: None,
            metadata: Some("{\"cpu_usage\": 92.5, \"threshold\": 90, \"duration_seconds\": 300}".to_string()),
        },
        SystemAlertDetail {
            id: 2,
            title: "Disk Space Low".to_string(),
            message: "Disk usage exceeded 80% on /dev/sda1".to_string(),
            severity: "warning".to_string(),
            status: "acknowledged".to_string(),
            source: "storage".to_string(),
            created_at: "2026-03-27T12:00:00Z".to_string(),
            acknowledged_at: Some("2026-03-27T12:05:00Z".to_string()),
            acknowledged_by: Some("admin".to_string()),
            resolved_at: None,
            resolved_by: None,
            metadata: Some("{\"disk_usage\": 82, \"threshold\": 80, \"device\": \"/dev/sda1\"}".to_string()),
        },
        SystemAlertDetail {
            id: 3,
            title: "Network Interface Down".to_string(),
            message: "Network interface eth1 is down".to_string(),
            severity: "critical".to_string(),
            status: "resolved".to_string(),
            source: "network".to_string(),
            created_at: "2026-03-27T11:30:00Z".to_string(),
            acknowledged_at: Some("2026-03-27T11:35:00Z".to_string()),
            acknowledged_by: Some("admin".to_string()),
            resolved_at: Some("2026-03-27T11:50:00Z".to_string()),
            resolved_by: Some("admin".to_string()),
            metadata: Some("{\"interface\": \"eth1\", \"reason\": \"cable_disconnected\"}".to_string()),
        },
    ];

    // 5. 查找告警
    let alert = mock_alerts.into_iter().find(|a| a.id == alert_id);

    // 6. 验证告警存在性
    match alert {
        Some(alert) => {
            // 7. 返回告警详情
            Ok(HttpResponse::Ok().json(SystemAlertDetailResponse {
                success: true,
                data: alert,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("System alert {} not found", alert_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
