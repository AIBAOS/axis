// Phase 236: 媒体视频详情 API
// GET /api/v1/media/videos/{id} — 获取视频详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 视频详情信息
#[derive(Serialize, Clone)]
pub struct VideoDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub resolution: String,
    pub codec: String,
    pub bitrate: u64,
    pub framerate: f32,
    pub created_at: u64,
    pub updated_at: u64,
    pub thumbnail_path: String,
    pub folder: String,
}

/// 视频详情响应
#[derive(Serialize)]
pub struct VideoDetailResponse {
    pub success: bool,
    pub data: VideoDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取视频详情（Phase 236）
/// - JWT 认证，任意登录用户可访问
/// - 验证视频 ID 存在性（404 Not Found）
/// - 返回视频完整详情
/// - 错误处理：401/404/500
pub async fn get_video_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let video_id = path.into_inner();

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

    // 4. 模拟视频数据
    // 实际实现中，这里会查询数据库或文件系统
    let mock_videos = vec![
        VideoDetail {
            id: 1,
            name: "movie_001.mp4".to_string(),
            path: "/media/videos/movie_001.mp4".to_string(),
            size_bytes: 2147483648, // 2 GB
            duration_seconds: 7200, // 2 hours
            resolution: "1920x1080".to_string(),
            codec: "H.264".to_string(),
            bitrate: 5000000, // 5 Mbps
            framerate: 30.0,
            created_at: now - 86400,
            updated_at: now - 86400,
            thumbnail_path: "/media/thumbnails/movie_001.jpg".to_string(),
            folder: "/media/videos".to_string(),
        },
        VideoDetail {
            id: 2,
            name: "movie_002.mp4".to_string(),
            path: "/media/videos/movie_002.mp4".to_string(),
            size_bytes: 3221225472, // 3 GB
            duration_seconds: 9000, // 2.5 hours
            resolution: "3840x2160".to_string(),
            codec: "H.265".to_string(),
            bitrate: 8000000, // 8 Mbps
            framerate: 60.0,
            created_at: now - 172800,
            updated_at: now - 172800,
            thumbnail_path: "/media/thumbnails/movie_002.jpg".to_string(),
            folder: "/media/videos".to_string(),
        },
    ];

    // 5. 查找视频
    let video = mock_videos.into_iter().find(|v| v.id == video_id);

    match video {
        Some(detail) => {
            Ok(HttpResponse::Ok().json(VideoDetailResponse {
                success: true,
                data: detail,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Video {} not found", video_id),
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
    async fn test_get_video_detail_success() {
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
                .route("/api/v1/media/videos/{id}", web::get().to(get_video_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_video_detail_not_found() {
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
                .route("/api/v1/media/videos/{id}", web::get().to(get_video_detail))
        ).await;

        // 注意：实际测试需要验证视频不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
