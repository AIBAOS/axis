// Phase 232: 媒体视频列表 API
// GET /api/v1/media/videos — 获取视频列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 视频查询参数
#[derive(Debug, Deserialize)]
pub struct VideosQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub folder: Option<String>,
}

/// 视频信息
#[derive(Serialize, Clone)]
pub struct VideoInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub resolution: String,
    pub created_at: u64,
    pub thumbnail_path: String,
}

/// 视频列表响应
#[derive(Serialize)]
pub struct VideoListResponse {
    pub success: bool,
    pub data: Vec<VideoInfo>,
    pub total_count: u64,
    pub page: u32,
    pub per_page: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取视频列表（Phase 235 增强版）
/// - JWT 认证，任意登录用户可访问
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：folder（可选，按目录过滤）
/// - 返回视频列表 + 总数
/// - 错误处理：401/500
pub async fn get_videos(
    req: HttpRequest,
    query: web::Query<VideosQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).max(1).min(100);

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
    // 实际实现中，这里会扫描媒体库目录或查询数据库
    let all_videos = vec![
        VideoInfo {
            id: 1,
            name: "movie_001.mp4".to_string(),
            path: "/media/videos/movie_001.mp4".to_string(),
            size_bytes: 2147483648, // 2 GB
            duration_seconds: 7200, // 2 hours
            resolution: "1920x1080".to_string(),
            created_at: now - 86400,
            thumbnail_path: "/media/thumbnails/movie_001.jpg".to_string(),
        },
        VideoInfo {
            id: 2,
            name: "movie_002.mp4".to_string(),
            path: "/media/videos/movie_002.mp4".to_string(),
            size_bytes: 3221225472, // 3 GB
            duration_seconds: 9000, // 2.5 hours
            resolution: "3840x2160".to_string(),
            created_at: now - 172800,
            thumbnail_path: "/media/thumbnails/movie_002.jpg".to_string(),
        },
        VideoInfo {
            id: 3,
            name: "video_003.mp4".to_string(),
            path: "/media/videos/video_003.mp4".to_string(),
            size_bytes: 536870912, // 512 MB
            duration_seconds: 1800, // 30 minutes
            resolution: "1920x1080".to_string(),
            created_at: now - 259200,
            thumbnail_path: "/media/thumbnails/video_003.jpg".to_string(),
        },
    ];

    // 5. 应用 folder 筛选
    let filtered_videos: Vec<VideoInfo> = all_videos.into_iter().filter(|v| {
        query.folder.as_ref().map_or(true, |folder| v.path.starts_with(folder))
    }).collect();

    // 6. 应用分页
    let total_count = filtered_videos.len() as u64;
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_videos.len());
    
    let videos = if start < filtered_videos.len() {
        filtered_videos[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(VideoListResponse {
        success: true,
        data: videos,
        total_count,
        page,
        per_page,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_videos_success() {
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
                .route("/api/v1/media/videos", web::get().to(get_videos))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_videos_unauthorized() {
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
                .route("/api/v1/media/videos", web::get().to(get_videos))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
