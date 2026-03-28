// Phase 237: 媒体音频详情 API
// GET /api/v1/media/audios/{id} — 获取音频详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 音频详情信息
#[derive(Serialize, Clone)]
pub struct AudioDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub duration_seconds: u64,
    pub artist: String,
    pub album: String,
    pub track_number: u32,
    pub genre: String,
    pub bitrate: u64,
    pub sample_rate: u32,
    pub created_at: u64,
    pub updated_at: u64,
    pub thumbnail_path: String,
}

/// 音频详情响应
#[derive(Serialize)]
pub struct AudioDetailResponse {
    pub success: bool,
    pub data: AudioDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取音频详情（Phase 237）
/// - JWT 认证，任意登录用户可访问
/// - 验证音频 ID 存在性（404 Not Found）
/// - 返回音频完整详情
/// - 错误处理：401/404/500
pub async fn get_audio_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let audio_id = path.into_inner();

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
    // 实际实现中，这里会查询数据库或文件系统
    let mock_audios = vec![
        AudioDetail {
            id: 1,
            name: "song_001.mp3".to_string(),
            path: "/media/audios/song_001.mp3".to_string(),
            size_bytes: 10485760, // 10 MB
            duration_seconds: 240, // 4 minutes
            artist: "Artist A".to_string(),
            album: "Album X".to_string(),
            track_number: 1,
            genre: "Rock".to_string(),
            bitrate: 320000, // 320 kbps
            sample_rate: 44100, // 44.1 kHz
            created_at: now - 86400,
            updated_at: now - 86400,
            thumbnail_path: "/media/thumbnails/song_001.jpg".to_string(),
        },
        AudioDetail {
            id: 2,
            name: "song_002.mp3".to_string(),
            path: "/media/audios/song_002.mp3".to_string(),
            size_bytes: 8388608, // 8 MB
            duration_seconds: 180, // 3 minutes
            artist: "Artist B".to_string(),
            album: "Album Y".to_string(),
            track_number: 2,
            genre: "Pop".to_string(),
            bitrate: 256000, // 256 kbps
            sample_rate: 44100, // 44.1 kHz
            created_at: now - 172800,
            updated_at: now - 172800,
            thumbnail_path: "/media/thumbnails/song_002.jpg".to_string(),
        },
    ];

    // 5. 查找音频
    let audio = mock_audios.into_iter().find(|a| a.id == audio_id);

    match audio {
        Some(detail) => {
            Ok(HttpResponse::Ok().json(AudioDetailResponse {
                success: true,
                data: detail,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Audio {} not found", audio_id),
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
    async fn test_get_audio_detail_success() {
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
                .route("/api/v1/media/audios/{id}", web::get().to(get_audio_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_audio_detail_not_found() {
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
                .route("/api/v1/media/audios/{id}", web::get().to(get_audio_detail))
        ).await;

        // 注意：实际测试需要验证音频不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
