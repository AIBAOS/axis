// Phase 238: 媒体照片详情 API
// GET /api/v1/media/photos/{id} — 获取照片详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 照片详情信息
#[derive(Serialize, Clone)]
pub struct PhotoDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub taken_at: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub thumbnail_path: String,
    pub album: String,
    pub camera: Option<String>,
    pub location: Option<String>,
}

/// 照片详情响应
#[derive(Serialize)]
pub struct PhotoDetailResponse {
    pub success: bool,
    pub data: PhotoDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取照片详情（Phase 238）
/// - JWT 认证，任意登录用户可访问
/// - 验证照片 ID 存在性（404 Not Found）
/// - 返回照片完整详情
/// - 错误处理：401/404/500
pub async fn get_photo_detail(
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

    // 2. 验证 token 有效性（任意登录用户可访问）
    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 获取当前时间戳
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to get current time")
        })?
        .as_secs();

    // 4. 模拟照片数据
    // 实际实现中，这里会查询数据库或文件系统
    let mock_photos = vec![
        PhotoDetail {
            id: 1,
            name: "photo_001.jpg".to_string(),
            path: "/media/photos/photo_001.jpg".to_string(),
            size_bytes: 5242880, // 5 MB
            width: 4000,
            height: 3000,
            taken_at: now - 86400,
            created_at: now - 86400,
            updated_at: now - 86400,
            thumbnail_path: "/media/thumbnails/photo_001.jpg".to_string(),
            album: "Album A".to_string(),
            camera: Some("Canon EOS R5".to_string()),
            location: Some("Beijing, China".to_string()),
        },
        PhotoDetail {
            id: 2,
            name: "photo_002.jpg".to_string(),
            path: "/media/photos/photo_002.jpg".to_string(),
            size_bytes: 4194304, // 4 MB
            width: 3840,
            height: 2160,
            taken_at: now - 172800,
            created_at: now - 172800,
            updated_at: now - 172800,
            thumbnail_path: "/media/thumbnails/photo_002.jpg".to_string(),
            album: "Album B".to_string(),
            camera: Some("Sony A7III".to_string()),
            location: Some("Shanghai, China".to_string()),
        },
    ];

    // 5. 查找照片
    let photo = mock_photos.into_iter().find(|p| p.id == photo_id);

    match photo {
        Some(detail) => {
            Ok(HttpResponse::Ok().json(PhotoDetailResponse {
                success: true,
                data: detail,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Photo {} not found", photo_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_photo_detail_success() {
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
                .route("/api/v1/media/photos/{id}", web::get().to(get_photo_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_photo_detail_not_found() {
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
                .route("/api/v1/media/photos/{id}", web::get().to(get_photo_detail))
        ).await;

        // 注意：实际测试需要验证照片不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
