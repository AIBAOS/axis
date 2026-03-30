// 系统更新/固件管理处理器
// 包含：检查更新、下载、安装、取消等操作

use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

use crate::services::jwt_service::JwtService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInfo {
    pub version: String,
    pub release_date: String,
    pub changelog: Vec<String>,
    pub download_url: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateCheckResult {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub update_info: Option<UpdateInfo>,
    pub release_notes: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateDownloadStatus {
    pub status: String,
    pub progress: u8,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub eta_seconds: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInstallResult {
    pub success: bool,
    pub message: String,
    pub reboot_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateStatus {
    pub current_status: &'static str,
    pub progress: u8,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub last_check: Option<&'static str>,
    pub last_install: Option<&'static str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallRequest {
    pub version: Option<String>,
}

// 模拟更新状态存储
static UPDATE_STATUS: Mutex<UpdateStatus> = Mutex::new(UpdateStatus {
    current_status: "idle",
    progress: 0,
    downloaded_bytes: 0,
    total_bytes: 0,
    last_check: Some("2026-03-19T01:00:00Z"),
    last_install: None,
});

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// GET /api/v1/system/updates/check — 检查更新
/// 需要登录用户访问
pub async fn check_update(
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

    let _claims = jwt_service.validate_token(&token.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    status.last_check = Some("2026-03-19T11:55:00Z");
    
    // 模拟检查结果（实际应从服务器获取）
    let has_update = true;
    let _latest_version = "v0.2.0";
    let _current_version = "v0.1.0";
    
    let update_info = if has_update {
        Some(UpdateInfo {
            version: "v0.2.0".to_string(),
            release_date: now,
            changelog: vec![
                "新增系统更新管理 API".to_string(),
                "修复通知管理模块时间戳问题".to_string(),
                "优化文件上传性能".to_string(),
            ],
            download_url: "https://github.com/AIBAOS/axis/releases/download/v0.2.0/axis-v0.2.0.deb".to_string(),
            size: 15_728_640, // 15MB
        })
    } else {
        None
    };
    
    Ok(HttpResponse::Ok().json(UpdateCheckResult {
        has_update,
        current_version: "v0.1.0".to_string(),
        latest_version: "v0.2.0".to_string(),
        update_info,
        release_notes: if has_update {
            "新版本 v0.2.0 包含多项改进和新功能，推荐升级。"
        } else {
            "当前已是最新版本。"
        }.to_string(),
    }))
}

/// GET /api/v1/system/updates/info — 获取更新信息
/// 需要登录用户访问
pub async fn get_update_info(
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

    let _claims = jwt_service.validate_token(&token.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let _status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    
    Ok(HttpResponse::Ok().json(json!({
        "current_version": "v0.1.0",
        "latest_version": "v0.2.0",
        "release_date": "2026-03-18T12:00:00Z",
        "download_url": "https://github.com/AIBAOS/axis/releases/download/v0.2.0/axis-v0.2.0.deb",
        "size": 15728640,
        "changelog": [
            "新增系统更新管理 API",
            "修复通知管理模块时间戳问题",
            "优化文件上传性能"
        ]
    })))
}

/// POST /api/v1/system/updates/download — 下载更新
/// 仅管理员可执行
pub async fn download_update(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
    Json(_payload): web::Json<DownloadRequest>,
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

    // 仅管理员可下载更新
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only admin users can download system updates"
        })));
    }

    let mut status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    
    if status.current_status != "idle" && status.current_status != "downloaded" {
        return Ok(HttpResponse::Conflict().json(json!({
            "success": false,
            "message": format!("Cannot download: current status is {}", status.current_status)
        })));
    }
    
    status.current_status = "downloading";
    status.progress = 0;
    status.total_bytes = 15_728_640; // 15MB
    
    // 模拟下载过程（实际应异步执行）
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Download started",
        "status": "downloading",
        "progress": 0
    })))
}

/// POST /api/v1/system/updates/install — 安装更新
/// 仅管理员可执行
pub async fn install_update(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
    Json(_payload): web::Json<InstallRequest>,
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

    // 仅管理员可安装更新
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only admin users can install system updates"
        })));
    }

    let mut status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    
    if status.current_status != "downloaded" && status.progress < 100 {
        return Ok(HttpResponse::Conflict().json(json!({
            "success": false,
            "message": "Download not completed"
        })));
    }
    
    status.current_status = "installing";
    
    // 模拟安装过程
    Ok(HttpResponse::Ok().json(UpdateInstallResult {
        success: true,
        message: "Update installed successfully".to_string(),
        reboot_required: true,
    }))
}

/// GET /api/v1/system/updates/status — 获取更新状态
/// 需要登录用户访问
pub async fn get_update_status(
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

    let _claims = jwt_service.validate_token(&token.ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing token"))?)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    
    Ok(HttpResponse::Ok().json(status.clone()))
}

/// POST /api/v1/system/updates/cancel — 取消更新
/// 仅管理员可执行
pub async fn cancel_update(
    req: HttpRequest,
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

    // 仅管理员可取消更新
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only admin users can cancel system updates"
        })));
    }

    let mut status = UPDATE_STATUS.lock().expect("UPDATE_STATUS lock poisoned");
    
    if status.current_status == "idle" || status.current_status == "downloaded" {
        return Ok(HttpResponse::Conflict().json(json!({
            "success": false,
            "message": "No update in progress"
        })));
    }
    
    let was_downloading = status.current_status == "downloading";
    status.current_status = "cancelled";
    status.progress = if was_downloading { 0 } else { status.progress };
    
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Update cancelled",
        "previous_status": status.current_status
    })))
}
