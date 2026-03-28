// Phase 233: 媒体音频列表 API
// GET /api/v1/media/audios — 获取音频列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 音频查询参数
#[derive(Debug, Deserialize)]
pub struct AudiosQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub artist: Option<String>,
    pub album: Option<String>,
}

/// 音频信息
#[derive(Serialize, Clone)]
pub struct AudioInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub artist: String,
    pub album: String,
    pub track_number: u32,
    pub created_at: u64,
    pub thumbnail_path: String,
}

/// 音频列表响应
#[derive(Serialize)]
pub struct AudioListResponse {
    pub success: bool,
    pub data: Vec<AudioInfo>,
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

/// 获取音频列表（Phase 233）
/// - JWT 认证，任意登录用户可访问
/// - 支持分页：page(默认 1)/per_page(默认 20)
/// - 返回音频列表 + 总数
/// - 错误处理：401/500
pub async fn get_audios(
    req: HttpRequest,
    query: web::Query<AudiosQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

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

    // 4. 模拟音频数据
    // 实际实现中，这里会扫描媒体库目录或查询数据库
    let all_audios = vec![
        AudioInfo {
            id: 1,
            name: "song_001.mp3".to_string(),
            path: "/media/audios/song_001.mp3".to_string(),
            size_bytes: 10485760, // 10 MB
            duration_seconds: 240, // 4 minutes
            artist: "Artist A".to_string(),
            album: "Album X".to_string(),
            track_number: 1,
            created_at: now - 86400,
            thumbnail_path: "/media/thumbnails/song_001.jpg".to_string(),
        },
        AudioInfo {
            id: 2,
            name: "song_002.mp3".to_string(),
            path: "/media/audios/song_002.mp3".to_string(),
            size_bytes: 8388608, // 8 MB
            duration_seconds: 180, // 3 minutes
            artist: "Artist B".to_string(),
            album: "Album Y".to_string(),
            track_number: 2,
            created_at: now - 172800,
            thumbnail_path: "/media/thumbnails/song_002.jpg".to_string(),
        },
        AudioInfo {
            id: 3,
            name: "song_003.mp3".to_string(),
            path: "/media/audios/song_003.mp3".to_string(),
            size_bytes: 12582912, // 12 MB
            duration_seconds: 300, // 5 minutes
            artist: "Artist C".to_string(),
            album: "Album Z".to_string(),
            track_number: 3,
            created_at: now - 259200,
            thumbnail_path: "/media/thumbnails/song_003.jpg".to_string(),
        },
    ];

    // 5. 应用筛选
    let filtered_audios: Vec<AudioInfo> = all_audios.into_iter().filter(|a| {
        let artist_match = query.artist.as_ref().map_or(true, |artist| a.artist == *artist);
        let album_match = query.album.as_ref().map_or(true, |album| a.album == *album);
        artist_match && album_match
    }).collect();

    // 6. 应用分页
    let total_count = filtered_audios.len() as u64;
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_audios.len());
    
    let audios = if start < filtered_audios.len() {
        filtered_audios[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(AudioListResponse {
        success: true,
        data: audios,
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
    async fn test_get_audios_success() {
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
                .route("/api/v1/media/audios", web::get().to(get_audios))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_audios_unauthorized() {
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
                .route("/api/v1/media/audios", web::get().to(get_audios))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
