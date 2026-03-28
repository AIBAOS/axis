// Phase 43 文件重命名/移动 API
// 支持重命名和移动文件/文件夹

use actix_web::{web, HttpResponse, Error};
use serde::Deserialize;
use std::path::PathBuf;
use std::fs;

/// 重命名/移动请求体
#[derive(Debug, Deserialize)]
pub struct RenameRequest {
    pub source_path: String,
    pub target_path: String,
}

/// PUT /api/v1/files/rename — 重命名/移动文件或文件夹
pub async fn rename_file(
    payload: web::Json<RenameRequest>,
) -> Result<HttpResponse, Error> {
    let root_dir = PathBuf::from("/data/uploads");

    // 解析源路径
    let source = if payload.source_path.is_empty() || payload.source_path == "/" {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "source_path cannot be empty or root"
        })));
    } else {
        root_dir.join(&payload.source_path.trim_start_matches('/'))
    };

    // 解析目标路径
    let target = if payload.target_path.is_empty() || payload.target_path == "/" {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "target_path cannot be empty or root"
        })));
    } else {
        root_dir.join(&payload.target_path.trim_start_matches('/'))
    };

    // 安全校验：确保根目录存在
    if !root_dir.exists() {
        fs::create_dir_all(&root_dir).map_err(|e| {
            log::error!("Failed to create root directory: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to initialize storage")
        })?;
    }

    let canonical_root = fs::canonicalize(&root_dir).map_err(|e| {
        log::error!("Failed to canonicalize root: {}", e);
        actix_web::error::ErrorInternalServerError("Storage error")
    })?;

    // 检查源路径是否存在
    if !source.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Source path not found: {}", payload.source_path)
        })));
    }

    // 获取规范路径进行安全检查
    let canonical_source = match fs::canonicalize(&source) {
        Ok(p) => p,
        Err(e) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "message": format!("Cannot resolve source path: {}", e)
            })));
        }
    };

    // 安全校验：确保源路径在根目录内
    if !canonical_source.starts_with(&canonical_root) {
        log::warn!("Attempted path traversal attack on source: {:?}", canonical_source);
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "Access denied: source path outside allowed directory"
        })));
    }

    // 检查目标路径是否已存在
    if target.exists() {
        return Ok(HttpResponse::Conflict().json(serde_json::json!({
            "success": false,
            "message": format!("Target path already exists: {}", payload.target_path)
        })));
    }

    // 安全校验：确保目标路径在根目录内
    // 需要检查目标路径的父目录
    if let Some(target_parent) = target.parent() {
        // 创建父目录（如果不存在）
        if !target_parent.exists() {
            fs::create_dir_all(target_parent).map_err(|e| {
                log::error!("Failed to create target parent directory: {}", e);
                actix_web::error::ErrorInternalServerError("Failed to create target directory")
            })?;
        }

        // 验证父目录在根目录内
        let canonical_target_parent = fs::canonicalize(target_parent).map_err(|e| {
            log::error!("Failed to canonicalize target parent: {}", e);
            actix_web::error::ErrorInternalServerError("Storage error")
        })?;

        if !canonical_target_parent.starts_with(&canonical_root) {
            log::warn!("Attempted path traversal attack on target: {:?}", canonical_target_parent);
            return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                "success": false,
                "message": "Access denied: target path outside allowed directory"
            })));
        }
    }

    // 执行重命名/移动操作
    fs::rename(&canonical_source, &target).map_err(|e| {
        log::error!("Failed to rename {} to {}: {}", canonical_source.display(), target.display(), e);
        actix_web::error::ErrorInternalServerError(format!("Failed to rename: {}", e))
    })?;

    log::info!("File renamed/moved: {:?} -> {:?}", canonical_source, target);

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "File renamed successfully",
        "data": {
            "source_path": payload.source_path,
            "target_path": payload.target_path
        }
    })))
}

/// PUT /api/v1/files/move — 移动文件（别名，复用 rename 逻辑）
pub async fn move_file(
    payload: web::Json<RenameRequest>,
) -> Result<HttpResponse, Error> {
    rename_file(payload).await
}