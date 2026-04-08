// Phase 199/200: 系统通知标记已读 API
// PUT /api/v1/system/notifications/{id}/read — 标记系统通知为已读
// POST /api/v1/system/notifications/{id}/mark-read — 标记通知为已读（支持个人通知）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::notification_store::SqliteNotificationRepository;

/// 标记已读响应
#[derive(Serialize)]
pub struct MarkAsReadResponse {
    pub success: bool,
    pub message: String,
    pub notification: Option<NotificationSummary>,
}

/// 通知摘要
#[derive(Serialize)]
pub struct NotificationSummary {
    pub id: i64,
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub is_read: bool,
    pub read_at: Option<i64>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 标记系统通知为已读（Phase 199）
/// - JWT 认证，登录用户可访问
/// - 验证通知 ID 存在性（404 Not Found）
/// - 仅允许标记 target_user_id IS NULL 的系统通知
/// - 标记成功返回 200 OK + 通知摘要
pub async fn mark_system_notification_as_read(
    req: HttpRequest,
    path: web::Path<i64>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse, Error> {
    let notification_id = path.into_inner();

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

    // 3. 查询通知是否存在
    match repo.get_notification_by_id(notification_id) {
        Ok(Some(notification)) => {
            // 4. 验证是系统通知（target_user_id IS NULL）
            if notification.target_user_id.is_some() {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "只能标记系统通知为已读".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 5. 如果已经是已读状态，返回 409 Conflict
            if notification.is_read {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: "通知已是已读状态".to_string(),
                    code: "CONFLICT".to_string(),
                }));
            }

            // 6. 标记为已读
            match repo.mark_as_read(notification_id) {
                Ok(true) => {
                    // 7. 获取更新后的通知信息
                    match repo.get_notification_by_id(notification_id) {
                        Ok(Some(updated)) => {
                            let title = updated.title.clone();
                            let summary = NotificationSummary {
                                id: updated.id,
                                title: updated.title,
                                message: updated.message,
                                notification_type: updated.notification_type,
                                is_read: updated.is_read,
                                read_at: updated.read_at,
                            };
                            Ok(HttpResponse::Ok().json(MarkAsReadResponse {
                                success: true,
                                message: format!("通知 '{}' 已标记为已读", title),
                                notification: Some(summary),
                            }))
                        }
                        _ => Ok(HttpResponse::Ok().json(MarkAsReadResponse {
                            success: true,
                            message: "通知已标记为已读".to_string(),
                            notification: None,
                        })),
                    }
                }
                Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("Notification {} not found", notification_id),
                    code: "NOT_FOUND".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("标记已读失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                })),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Notification {} not found", notification_id),
            code: "NOT_FOUND".to_string(),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询通知失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

/// 标记通知为已读（Phase 200）
/// - JWT 认证，登录用户可访问
/// - 验证通知 ID 存在性（404 Not Found）
/// - 验证通知归属：只能标记自己的通知，admin 可标记任意
/// - 已读通知重复标记返回 409 Conflict
/// - 更新 is_read = true, read_at = 当前时间
/// - 返回 200 OK + 通知详情
pub async fn mark_notification_as_read(
    req: HttpRequest,
    path: web::Path<i64>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteNotificationRepository>>,
) -> Result<HttpResponse, Error> {
    let notification_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并获取用户信息
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    let user_id = claims.user_id;

    // 3. 查询通知是否存在
    match repo.get_notification_by_id(notification_id) {
        Ok(Some(notification)) => {
            // 4. 验证通知归属
            if !is_admin {
                // 非 admin 用户只能标记自己的通知或系统通知
                if let Some(target_id) = notification.target_user_id {
                    if target_id != user_id as i64 {
                        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                            success: false,
                            error: "只能标记自己的通知为已读".to_string(),
                            code: "FORBIDDEN".to_string(),
                        }));
                    }
                }
            }

            // 5. 如果已经是已读状态，返回 409 Conflict
            if notification.is_read {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: "通知已是已读状态".to_string(),
                    code: "CONFLICT".to_string(),
                }));
            }

            // 6. 标记为已读
            match repo.mark_as_read(notification_id) {
                Ok(true) => {
                    // 7. 获取更新后的通知信息
                    match repo.get_notification_by_id(notification_id) {
                        Ok(Some(updated)) => {
                            let title = updated.title.clone();
                            let summary = NotificationSummary {
                                id: updated.id,
                                title: updated.title,
                                message: updated.message,
                                notification_type: updated.notification_type,
                                is_read: updated.is_read,
                                read_at: updated.read_at,
                            };
                            Ok(HttpResponse::Ok().json(MarkAsReadResponse {
                                success: true,
                                message: format!("通知 '{}' 已标记为已读", title),
                                notification: Some(summary),
                            }))
                        }
                        _ => Ok(HttpResponse::Ok().json(MarkAsReadResponse {
                            success: true,
                            message: "通知已标记为已读".to_string(),
                            notification: None,
                        })),
                    }
                }
                Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("Notification {} not found", notification_id),
                    code: "NOT_FOUND".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("标记已读失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                })),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Notification {} not found", notification_id),
            code: "NOT_FOUND".to_string(),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询通知失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}
