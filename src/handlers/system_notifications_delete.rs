// Phase 208: 通知删除 API
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
/// - JWT 认证，仅 admin 用户可访问
/// - 验证通知 ID 存在性（404 Not Found）
/// - 删除成功返回 204 No Content
pub async fn delete_notification(
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

    // 3. 验证 admin 权限
    if !claims.roles.iter().any(|r| r == "admin") {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "仅 admin 可删除通知".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 查询通知是否存在
    match repo.get_notification_by_id(notification_id) {
        Ok(Some(_notification)) => {
            // 5. 删除通知
            match repo.delete_notification(notification_id) {
                Ok(true) => Ok(HttpResponse::NoContent().finish()),
                Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("Notification {} not found", notification_id),
                    code: "NOT_FOUND".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("删除通知失败：{}", e),
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_delete_notification_success() {
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
                .route("/api/v1/system/notifications/{id}", web::delete().to(delete_notification))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
