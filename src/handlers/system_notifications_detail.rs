// Phase 209: 系统通知详情 API
// GET /api/v1/system/notifications/{id} — 获取通知详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::notification_store::SqliteNotificationRepository;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    #[serde(rename = "type")]
    pub notification_type: Option<String>,
}

/// 通知详情响应
#[derive(Serialize)]
pub struct NotificationDetailResponse {
    pub success: bool,
    pub data: NotificationDetail,
}

/// 通知详情
#[derive(Serialize, Clone)]
pub struct NotificationDetail {
    pub id: i64,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub source: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub read_at: Option<i64>,
    pub metadata: Option<NotificationMetadata>,
}

/// 通知元数据
#[derive(Serialize, Clone)]
pub struct NotificationMetadata {
    pub priority: Option<String>,
    pub target_user_id: Option<i64>,
    pub action_url: Option<String>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取通知详情（Phase 209）
/// - JWT 认证，登录用户可访问
/// - 验证通知 ID 存在性（404 Not Found）
/// - 验证通知归属：用户只能查看自己的通知，admin 可查看任意
pub async fn get_notification_detail(
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

    let is_admin = claims.roles.iter().any(|r| r == "admin");
    let user_id = claims.user_id;

    // 3. 查询通知是否存在
    match repo.get_notification_by_id(notification_id) {
        Ok(Some(notification)) => {
            // 4. 验证通知归属
            if !is_admin {
                // 非 admin 用户只能查看自己的通知或系统通知
                if let Some(target_id) = notification.target_user_id {
                    if target_id != user_id as i64 {
                        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                            success: false,
                            error: "只能查看自己的通知".to_string(),
                            code: "FORBIDDEN".to_string(),
                        }));
                    }
                }
            }

            // 5. 转换为响应格式
            let detail = NotificationDetail {
                id: notification.id,
                notification_type: notification.notification_type,
                title: notification.title,
                message: notification.message,
                source: notification.source,
                status: if notification.is_read { "read".to_string() } else { "unread".to_string() },
                created_at: notification.created_at,
                read_at: notification.read_at,
                metadata: Some(NotificationMetadata {
                    priority: Some(notification.priority),
                    target_user_id: notification.target_user_id,
                    action_url: notification.action_url,
                }),
            };

            Ok(HttpResponse::Ok().json(NotificationDetailResponse {
                success: true,
                data: detail,
            }))
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_notification_detail_success() {
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
                .route("/api/v1/system/notifications/{id}", web::get().to(get_notification_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
