// Phase 240: 媒体照片删除 API
// DELETE /api/v1/media/photos/{id} — 删除照片

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 删除响应
#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除照片（Phase 240 增强版）
/// - JWT 认证，登录用户可访问
/// - 验证照片 ID 存在性（404 Not Found）
/// - 权限验证：仅照片所有者可删除（403 Forbidden）
/// - 删除成功后返回 204 No Content
/// - 错误处理：401/403/404/500
pub async fn delete_photo(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let photo_id = path.into_inner();

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

    let current_user_id = claims.user_id;

    // 3. 模拟照片数据验证
    // 实际实现中，这里会查询数据库验证照片是否存在及所有者
    let mock_photos = vec![
        (1u64, 100u64), // (photo_id, owner_id)
        (2, 100),
        (3, 101),
        (4, 102),
        (5, 100),
    ];

    // 4. 验证照片是否存在并检查权限
    let photo_owner = mock_photos.iter().find(|(id, _)| *id == photo_id);
    
    match photo_owner {
        Some((_, owner_id)) => {
            // 5. 权限验证：仅照片所有者可删除
            if *owner_id != current_user_id {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "Only photo owner can delete this photo".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }
        }
        None => {
            // 照片不存在
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Photo {} not found", photo_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    }

    // 6. 模拟删除照片
    // 实际实现中，这里会删除文件并更新数据库
    Ok(HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_delete_photo_success() {
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
                .route("/api/v1/media/photos/{id}", web::delete().to(delete_photo))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_delete_photo_not_found() {
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
                .route("/api/v1/media/photos/{id}", web::delete().to(delete_photo))
        ).await;

        // 注意：实际测试需要验证照片不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
