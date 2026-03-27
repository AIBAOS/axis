pub mod scanner;

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::models::media::{MediaFile, MediaMetadata};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
}

pub async fn list_media_files(
    query: web::Query<QueryParams>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    
    // 示例数据
    let files = vec![
        MediaFile {
            id: 1,
            file_path: "/media/videos/hero.mp4".to_string(),
            file_name: "hero.mp4".to_string(),
            file_type: "video".to_string(),
            size: 1073741824,
            metadata: MediaMetadata {
                resolution: "1920x1080".to_string(),
                duration: 7200,
                bitrate: 5000,
                codec: "h264".to_string(),
                size: Some(1073741824),
            },
            created_at: chrono::Utc::now().to_rfc3339(),
        },
    ];
    
    Ok(HttpResponse::Ok().json(files))
}

pub async fn get_media_file(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    
    Ok(HttpResponse::Ok().json(MediaFile {
        id,
        file_path: format!("/media/files/{}", id),
        file_name: format!("file_{}.mp4", id),
        file_type: "video".to_string(),
        size: 1073741824,
        metadata: MediaMetadata {
            resolution: "1920x1080".to_string(),
            duration: 7200,
            bitrate: 5000,
            codec: "h264".to_string(),
            size: Some(1073741824),
        },
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

pub async fn get_media_stats() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "total_files": 150,
        "videos": 85,
        "music": 45,
        "images": 20,
        "total_size": 5368709120u64
    })))
}
