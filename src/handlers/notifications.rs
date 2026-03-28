// 通知管理处理器（SQLite 持久化版）
// 包含：列表、详情、创建、标记已读、删除

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::notification_store::SqliteNotificationRepository;

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    #[serde(rename = "type")]
    pub notification_type: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    pub title: String,
    pub message: String,
    #[serde(rename = "type", default = "default_type")]
    pub notification_type: String,
    #[serde(default = "default_priority")]
    pub priority: String,
    pub target_user_id: Option<i64>,
    pub action_url: Option<String>,
}

fn default_type() -> String { "info".to_string() }
fn default_priority() -> String { "normal".to_string() }

/// GET /api/v1/notifications — 通知列表（分页 + 筛选）
pub async fn get_notifications(
    query: web::Query<NotificationQuery>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);

    match repo.get_notifications(
        query.notification_type.as_deref(),
        query.status.as_deref(),
        None,
        page,
        page_size,
    ) {
        Ok((notifications, total)) => {
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "notifications": notifications,
                "total": total,
                "page": page,
                "page_size": page_size,
                "has_more": ((page - 1) * page_size + notifications.len() as u32) < total as u32
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("查询通知失败: {}", e)
            })))
        }
    }
}

/// GET /api/v1/notifications/{id} — 通知详情
pub async fn get_notification(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_notification_by_id(id) {
        Ok(Some(notification)) => Ok(HttpResponse::Ok().json(notification)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Notification '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询通知失败: {}", e)
        }))),
    }
}

/// POST /api/v1/notifications — 创建通知
pub async fn create_notification(
    payload: web::Json<CreateNotificationRequest>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    match repo.create_notification(
        &payload.title,
        &payload.message,
        &payload.notification_type,
        &payload.priority,
        None,
        payload.target_user_id,
        payload.action_url.as_deref(),
    ) {
        Ok(notification) => Ok(HttpResponse::Created().json(notification)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("创建通知失败: {}", e)
        }))),
    }
}

/// PUT /api/v1/notifications/{id}/read — 标记已读
pub async fn mark_as_read(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.mark_as_read(id) {
        Ok(true) => {
            match repo.get_notification_by_id(id) {
                Ok(Some(notification)) => Ok(HttpResponse::Ok().json(notification)),
                _ => Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": "已标记为已读"
                }))),
            }
        }
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Notification '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("标记已读失败: {}", e)
        }))),
    }
}

/// DELETE /api/v1/notifications/{id} — 删除通知
pub async fn delete_notification(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.delete_notification(id) {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("Notification '{}' deleted", id)
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Notification '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("删除通知失败: {}", e)
        }))),
    }
}

/// DELETE /api/v1/notifications/read — 删除所有已读通知
pub async fn delete_read_notifications(
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse> {
    match repo.delete_read_notifications() {
        Ok(deleted) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "deleted_count": deleted
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("删除已读通知失败: {}", e)
        }))),
    }
}
