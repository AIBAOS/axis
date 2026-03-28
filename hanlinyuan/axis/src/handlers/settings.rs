use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub key: String,
    // 兼容 Actix-web v4 的 Json 提取器
    pub value: serde_json::Value,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingRequest {
    pub value: serde_json::Value,
}

// 模拟 settings 存储，实际应从数据库读取
use once_cell::sync::Lazy;
use std::sync::Mutex;

static SETTINGS: Lazy<Mutex<serde_json::Map<String, serde_json::Value>>> = Lazy::new(|| {
    let mut settings = serde_json::Map::new();
    // 初始化默认设置
    settings.insert("network.host".to_string(), json!("0.0.0.0"));
    settings.insert("network.port".to_string(), json!(8080));
    settings.insert("storage.path".to_string(), json!("/data"));
    settings.insert("system.timezone".to_string(), json!("Asia/Shanghai"));
    settings.insert("user.prefer_theme".to_string(), json!("dark"));
    Mutex::new(settings)
});

fn get_settings_map() -> serde_json::Map<String, serde_json::Value> {
    let settings = SETTINGS.lock().unwrap();
    settings.clone()
}

pub async fn get_all_settings() -> Result<HttpResponse> {
    let settings = get_settings_map();
    Ok(HttpResponse::Ok().json(settings))
}

pub async fn get_setting(
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let key = path.into_inner();
    let settings = get_settings_map();
    
    match settings.get(&key) {
        Some(value) => Ok(HttpResponse::Ok().json(Setting {
            key,
            value: value.clone(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        })),
        None => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Setting '{}' not found", key)
        })))
    }
}

pub async fn update_setting(
    path: web::Path<String>,
    web::Json(payload): web::Json<UpdateSettingRequest>,
) -> Result<HttpResponse> {
    let key = path.into_inner();
    let mut settings = get_settings_map();
    
    settings.insert(key.clone(), payload.value.clone());
    
    let mut writable = SETTINGS.lock().unwrap();
    *writable = settings;
    
    Ok(HttpResponse::Ok().json(Setting {
        key,
        value: payload.value,
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}
