// Phase 197: 系统通知列表 API
// GET /api/v1/system/notifications — 获取系统通知列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::notification_store::{SqliteNotificationRepository, NotificationRow};

/// 查询参数
#[derive(Debug, serde::Deserialize)]
pub struct SystemNotificationsQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub priority: Option<String>,
}

/// 系统通知条目响应
#[derive(Serialize)]
pub struct SystemNotificationItem {
    pub id: i64,
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub priority: String,
    pub is_read: bool,
    pub created_at: i64,
    pub action_url: Option<String>,
}

/// 系统通知列表响应
#[derive(Serialize)]
pub struct SystemNotificationsResponse {
    pub success: bool,
    pub data: SystemNotificationsData,
}

/// 系统通知数据
#[derive(Serialize)]
pub struct SystemNotificationsData {
    pub notifications: Vec<SystemNotificationItem>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取系统通知列表（Phase 197）
/// - JWT 认证，登录用户可访问
/// - 返回系统级别的通知（target_user_id IS NULL 或 type = 'system'）
/// - 支持分页和优先级筛选
pub async fn get_system_notifications(
    req: HttpRequest,
    query: web::Query<SystemNotificationsQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 获取系统通知（筛选 target_user_id IS NULL 的通知）
    match repo.get_system_notifications(query.priority.as_deref(), page, page_size) {
        Ok((notifications, total)) => {
            // 4. 转换为响应格式
            let items: Vec<SystemNotificationItem> = notifications
                .into_iter()
                .map(|n| SystemNotificationItem {
                    id: n.id,
                    title: n.title,
                    message: n.message,
                    notification_type: n.notification_type,
                    priority: n.priority,
                    is_read: n.is_read,
                    created_at: n.created_at,
                    action_url: n.action_url,
                })
                .collect();

            let has_more = ((page - 1) * page_size + items.len() as u32) < total as u32;

            Ok(HttpResponse::Ok().json(SystemNotificationsResponse {
                success: true,
                data: SystemNotificationsData {
                    notifications: items,
                    total,
                    page,
                    page_size,
                    has_more,
                },
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询系统通知失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}
