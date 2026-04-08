// Phase 174: 系统告警列表 API
// GET /api/v1/system/alerts — 获取系统告警列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 告警列表查询参数
#[derive(Debug, Deserialize)]
pub struct AlertsListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub severity: Option<String>,
    pub source: Option<String>,
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
    pub resolved_at: Option<String>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 告警列表响应
#[derive(Serialize)]
pub struct AlertsListResponse {
    pub success: bool,
    pub data: Vec<SystemAlert>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证告警状态
fn validate_status(status: &str) -> bool {
    matches!(status.to_lowercase().as_str(), "active" | "resolved" | "acknowledged")
}

/// 验证告警级别
fn validate_severity(severity: &str) -> bool {
    matches!(severity.to_lowercase().as_str(), "critical" | "warning" | "info")
}

/// 获取系统告警列表（Phase 174）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
/// - 支持筛选：status(active/resolved/acknowledged), severity(critical/warning/info)
/// - 按 created_at 降序排序
pub async fn list_system_alerts(
    req: HttpRequest,
    query: web::Query<AlertsListQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let per_page = query.per_page.unwrap_or(20).min(100);

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
            error: "Only admin users can view system alerts".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证告警状态（如果提供）
    if let Some(ref status) = query.status {
        if !validate_status(status) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid status. Valid statuses: active, resolved, acknowledged".to_string(),
                code: "INVALID_STATUS".to_string(),
            }));
        }
    }

    // 5. 验证告警级别（如果提供）
    if let Some(ref severity) = query.severity {
        if !validate_severity(severity) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid severity. Valid severities: critical, warning, info".to_string(),
                code: "INVALID_SEVERITY".to_string(),
            }));
        }
    }

    // 6. 模拟系统告警数据（按 created_at 降序）
    let all_alerts = vec![
        SystemAlert {
            id: 1,
            title: "High CPU Usage".to_string(),
            message: "CPU usage exceeded 90% for more than 5 minutes".to_string(),
            severity: "critical".to_string(),
            status: "active".to_string(),
            source: "system".to_string(),
            created_at: "2026-03-27T12:30:00Z".to_string(),
            acknowledged_at: None,
            resolved_at: None,
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
            resolved_at: None,
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
            resolved_at: Some("2026-03-27T11:50:00Z".to_string()),
        },
        SystemAlert {
            id: 4,
            title: "Backup Completed".to_string(),
            message: "Daily backup completed successfully".to_string(),
            severity: "info".to_string(),
            status: "resolved".to_string(),
            source: "backup".to_string(),
            created_at: "2026-03-27T01:30:00Z".to_string(),
            acknowledged_at: None,
            resolved_at: Some("2026-03-27T01:30:00Z".to_string()),
        },
        SystemAlert {
            id: 5,
            title: "Failed Login Attempt".to_string(),
            message: "Multiple failed login attempts for user admin".to_string(),
            severity: "warning".to_string(),
            status: "active".to_string(),
            source: "auth".to_string(),
            created_at: "2026-03-27T01:00:00Z".to_string(),
            acknowledged_at: None,
            resolved_at: None,
        },
    ];

    // 7. 应用筛选
    let filtered_alerts: Vec<SystemAlert> = all_alerts
        .into_iter()
        .filter(|alert| {
            // 状态过滤
            if let Some(ref status) = query.status {
                if alert.status != status.to_lowercase() {
                    return false;
                }
            }
            // 级别过滤
            if let Some(ref severity) = query.severity {
                if alert.severity != severity.to_lowercase() {
                    return false;
                }
            }
            // 来源过滤
            if let Some(ref source) = query.source {
                if alert.source != *source {
                    return false;
                }
            }
            true
        })
        .collect();

    // 8. 应用分页
    let total = filtered_alerts.len() as u64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_alerts.len());
    
    let alerts = if start < filtered_alerts.len() {
        filtered_alerts[start..end].to_vec()
    } else {
        vec![]
    };

    // 9. 返回告警列表
    Ok(HttpResponse::Ok().json(AlertsListResponse {
        success: true,
        data: alerts,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
