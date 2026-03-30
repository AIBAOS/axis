// Phase 38 文件列表 API
// 返回指定目录下的文件/文件夹列表

use actix_web::{web, HttpRequest, HttpResponse, Error};
use serde::Deserialize;
use std::path::PathBuf;
use std::fs;
use std::time::UNIX_EPOCH;

use crate::services::jwt_service::JwtService;

/// 文件信息
#[derive(serde::Serialize)]
pub struct FileInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file" or "dir"
    pub size: u64,
    pub modified: u64,
}

/// 文件列表查询参数
#[derive(Clone, Deserialize)]
pub struct FilesQuery {
    pub path: Option<String>,
}

/// JWT 认证辅助函数
fn validate_auth(req: &HttpRequest, jwt_service: &web::Data<JwtService>) -> Result<crate::models::jwt::JwtClaims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Err(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    jwt_service.validate_token(&token.expect("Token should exist"))
        .map_err(|_| HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Invalid token"
        })))
}

/// 获取文件列表
/// 需要登录用户访问
pub async fn list_files(
    http_req: HttpRequest,
    query: web::Query<FilesQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    // 获取文件根目录
    let root_dir = PathBuf::from("/data/uploads");

    // 解析路径参数（默认为根目录）
    let base_path = query.path.clone().unwrap_or_else(|| "/".to_string());
    let dir_path = if base_path == "/" {
        root_dir.clone()
    } else {
        root_dir.join(&base_path.trim_start_matches('/'))
    };

    // 确保目录存在
    if !dir_path.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Directory not found: {}", dir_path.display())
        })));
    }

    // 读取目录内容
    let mut files = Vec::new();

    for entry in fs::read_dir(&dir_path).map_err(|e| {
        log::error!("Failed to read directory {}: {}", dir_path.display(), e);
        Error::from(actix_web::error::ErrorInternalServerError(format!(
            "Failed to read directory: {}", e
        )))
    })? {
        let entry = entry.map_err(|e| {
            log::error!("Failed to read directory entry: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError(format!(
                "Failed to read entry: {}", e
            )))
        })?;

        let metadata = entry.metadata().map_err(|e| {
            log::error!("Failed to get metadata: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError(format!(
                "Failed to get metadata: {}", e
            )))
        })?;

        let file_info = FileInfo {
            name: entry.file_name().to_string_lossy().to_string(),
            file_type: if metadata.is_dir() {
                "dir".to_string()
            } else {
                "file".to_string()
            },
            size: metadata.len(),
            modified: metadata.modified().map_err(|e| {
                log::error!("Failed to get modified time: {}", e);
                Error::from(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to get modified time: {}", e
                )))
            })?.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        };

        files.push(file_info);
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": files,
        "path": query.path.clone().unwrap_or_else(|| "/".to_string())
    })))
}
