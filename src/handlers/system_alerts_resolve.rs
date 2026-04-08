// Phase 177: 系统告警解决 API
// POST /api/v1/system/alerts/{id}/resolve — 解决系统告警

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 解决告警请求
#[derive(Debug, Deserialize)]
pub struct ResolveAlertRequest {
    pub notes: Option<String>,
}

/// 系统告警信息
#[derive(Serialize, Clone)]
pub struct SystemAlert {
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
}

/// 解决告警响应
#[derive(Serialize)]
pub struct ResolveAlertResponse {
    pub success: bool,
    pub message: String,
    pub data: SystemAlert,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 解决系统告警（Phase 177）
/// - JWT 认证，admin 角色可访问
/// - 验证告警 ID 存在性（404 Not Found）
/// - 验证告警状态（仅 active/acknowledged 状态可解决）
/// - 解决成功返回 200 OK + 更新后的告警详情
pub async fn resolve_system_alert(
    req: HttpRequest,
    path: web::Path<u64>,
    _payload: Option<web::Json<ResolveAlertRequest>>,
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
            error: "Only admin users can resolve system alerts".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 获取当前用户
    let current_user = claims.sub.clone();

    // 5. 模拟现有告警数据
    let mut mock_alerts = vec![
        SystemAlert {
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
        },
        SystemAlert {
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
        },
        SystemAlert {
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
        },
    ];

    // 6. 查找告警
    let alert_index = mock_alerts.iter().position(|a| a.id == alert_id);

    // 7. 验证告警存在性
    let alert_index = match alert_index {
        Some(idx) => idx,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("System alert {} not found", alert_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    };

    let alert = &mock_alerts[alert_index];

    // 8. 验证告警状态（仅 active/acknowledged 状态可解决）
    if alert.status == "resolved" {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Alert {} is already resolved", alert_id),
            code: "ALREADY_RESOLVED".to_string(),
        }));
    }

    // 9. 更新告警状态
    let now = chrono::Utc::now().to_rfc3339();
    let alert = &mut mock_alerts[alert_index];
    alert.status = "resolved".to_string();
    alert.resolved_at = Some(now.clone());
    alert.resolved_by = Some(current_user);

    // 10. 返回解决成功
    Ok(HttpResponse::Ok().json(ResolveAlertResponse {
        success: true,
        message: "System alert resolved successfully".to_string(),
        data: alert.clone(),
    }))
}
