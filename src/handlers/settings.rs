// Settings handlers
use actix_web::{web, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub settings: crate::models::settings::SystemSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfoResponse {
    pub storage: crate::models::settings::StorageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub max_file_size_mb: Option<u64>,
    pub max_upload_workers: Option<u32>,
    pub enable_quota: Option<bool>,
    pub default_quota_mb: Option<u64>,
}

// GET /api/settings
pub async fn get_settings() -> Result<HttpResponse, Error> {
    let settings = crate::models::settings::SystemSettings::default();
    Ok(HttpResponse::Ok().json(SettingsResponse { settings }))
}

// PUT /api/settings
pub async fn update_settings(
    req: web::Json<UpdateSettingsRequest>,
) -> Result<HttpResponse, Error> {
    let mut settings = crate::models::settings::SystemSettings::default();
    
    if let Some(v) = req.max_file_size_mb {
        settings.max_file_size_mb = v;
    }
    if let Some(v) = req.max_upload_workers {
        settings.max_upload_workers = v;
    }
    if let Some(v) = req.enable_quota {
        settings.enable_quota = v;
    }
    if let Some(v) = req.default_quota_mb {
        settings.default_quota_mb = v;
    }
    
    Ok(HttpResponse::Ok().json(SettingsResponse { settings }))
}

// GET /api/settings/storage
pub async fn get_storage() -> Result<HttpResponse, Error> {
    let storage = crate::models::settings::StorageInfo {
        total: 1024 * 1024,
        used: 0,
        available: 1024 * 1024,
    };
    Ok(HttpResponse::Ok().json(StorageInfoResponse { storage }))
}
