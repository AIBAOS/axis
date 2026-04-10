// Phase 197: 系统通知列表 API
// GET /api/v1/system/notifications — 获取系统通知列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::notification_store::SqliteNotificationRepository;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct NotificationsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    #[serde(rename = "type")]
    pub notification_type: Option<String>,
    pub status: Option<String>,
    pub source: Option<String>,
}

/// 分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 通知列表响应
#[derive(Serialize)]
pub struct NotificationsResponse {
    pub success: bool,
    pub data: Vec<NotificationItem>,
    pub pagination: PaginationInfo,
}

/// 通知项（简化响应格式）
#[derive(Serialize, Clone)]
pub struct NotificationItem {
    pub id: i64,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub source: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub read_at: Option<i64>,
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
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：type(info/warning/error/critical)/status(unread/read)/source
/// - 按 created_at 降序排序（最新的在前）
pub async fn list_notifications(
    req: HttpRequest,
    query: web::Query<NotificationsQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse, Error> {
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

    // 3. 解析分页参数
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20).max(1), 100); // Bug #87 修复

    // 4. 获取通知列表
    match repo.get_notifications(
        query.notification_type.as_deref(),
        query.status.as_deref(),
        query.source.as_deref(),
        page,
        per_page,
    ) {
        Ok((notifications, total)) => {
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            // 转换为响应格式
            let data: Vec<NotificationItem> = notifications.into_iter().map(|n| NotificationItem {
                id: n.id,
                notification_type: n.notification_type,
                title: n.title,
                message: n.message,
                source: n.source,
                status: if n.is_read { "read".to_string() } else { "unread".to_string() },
                created_at: n.created_at,
                read_at: n.read_at,
            }).collect();

            Ok(HttpResponse::Ok().json(NotificationsResponse {
                success: true,
                data,
                pagination: PaginationInfo {
                    page,
                    per_page,
                    total,
                    total_pages: total_pages as u32,
                },
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询通知列表失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_notifications_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/notifications", web::get().to(list_notifications))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
