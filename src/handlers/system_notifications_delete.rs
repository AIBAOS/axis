// Phase 208: 系统通知删除 API
// DELETE /api/v1/system/notifications/{id} — 删除系统通知

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::notification_store::SqliteNotificationRepository;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除系统通知（Phase 208）
/// - JWT 认证，admin 角色可访问
/// - 验证通知 ID 存在性（404 Not Found）
/// - 验证是系统通知（target_user_id IS NULL）
/// - 删除成功返回 204 No Content
pub async fn delete_system_notification(
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
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete system notifications".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 查询通知是否存在且为系统通知
    match repo.get_notification_by_id(notification_id) {
        Ok(Some(notification)) => {
            // 5. 验证是系统通知（target_user_id IS NULL）
            if notification.target_user_id.is_some() {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("System notification {} not found", notification_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 6. 执行删除
            match repo.delete_notification(notification_id) {
                Ok(true) => Ok(HttpResponse::NoContent().finish()),
                Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("System notification {} not found", notification_id),
                    code: "NOT_FOUND".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("删除系统通知失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                })),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("System notification {} not found", notification_id),
            code: "NOT_FOUND".to_string(),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询通知失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}
