use actix_web::{HttpResponse, Result};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanStatus {
    pub status: String,
    pub progress: f32,
    pub files_scanned: u64,
    pub total_files: u64,
    pub last_scan: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanConfig {
    pub scan_paths: Vec<String>,
    pub exclude_paths: Vec<String>,
    pub file_types: Vec<String>,
}

pub async fn trigger_scan() -> Result<HttpResponse> {
    // 模拟触发扫描任务
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Scan started",
        "scan_id": 1
    })))
}

pub async fn get_scan_status() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ScanStatus {
        status: "idle".to_string(),
        progress: 0.0,
        files_scanned: 0,
        total_files: 0,
        last_scan: None,
    }))
}

pub async fn get_scan_config() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ScanConfig {
        scan_paths: vec!["/media/video".to_string(), "/media/music".to_string()],
        exclude_paths: vec!["/media/video/.thumbnails".to_string()],
        file_types: vec!["mp4".to_string(), "avi".to_string(), "mkv".to_string()],
    }))
}

pub async fn update_scan_config(
    Json(new_config): Json<ScanConfig>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(new_config))
}
