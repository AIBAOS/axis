use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::services::jwt_service::JwtService;

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
    let settings = SETTINGS.lock().expect("SETTINGS lock poisoned");
    settings.clone()
}

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// GET /api/v1/settings — 获取所有设置
/// 需要登录用户访问
pub async fn get_all_settings(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let token_val = token.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?; let _claims = jwt_service.validate_token(&token_val)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let settings = get_settings_map();
    Ok(HttpResponse::Ok().json(settings))
}

/// GET /api/v1/settings/{key} — 获取单个设置
/// 需要登录用户访问
pub async fn get_setting(
    req: HttpRequest,
    path: web::Path<String>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let token_val = token.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?; let _claims = jwt_service.validate_token(&token_val)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

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
    req: HttpRequest,
    path: web::Path<String>,
    web::Json(payload): web::Json<UpdateSettingRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let claims = jwt_service.validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // 仅管理员可修改设置
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only admin users can modify settings"
        })));
    }

    let key = path.into_inner();
    let mut settings = get_settings_map();
    
    settings.insert(key.clone(), payload.value.clone());
    
    let mut writable = SETTINGS.lock().expect("SETTINGS lock poisoned");
    *writable = settings;
    
    Ok(HttpResponse::Ok().json(Setting {
        key,
        value: payload.value,
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}
