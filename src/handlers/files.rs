use actix_web::{web, HttpResponse, Error};
use futures_util::stream::StreamExt;
use std::path::PathBuf;
use serde::Serialize;
use std::fs;
use std::time::SystemTime;
use tracing::{error, debug};

// 初始化文件目录
pub fn ensure_upload_dir() -> Result<PathBuf, String> {
    let upload_path = PathBuf::from("uploads");
    fs::create_dir_all(&upload_path)
        .map_err(|e| format!("Failed to create uploads directory: {}", e))?;
    Ok(upload_path)
}

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub filename: String,
    pub path: String,
    pub size: u64,
    pub content_type: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub deleted: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub files: Vec<FileMeta>,
}

#[derive(Serialize)]
pub struct FileMeta {
    pub filename: String,
    pub size: u64,
    pub modified: u64,
}

// 上传文件接口
pub async fn upload_file(
    filename: web::Path<String>,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let filename_str = filename.into_inner();
    if filename_str.contains("..") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid filename"})));
    }
    
    let mut buffer = Vec::new();
    let mut size: u64 = 0;
    let mut stream = payload.into_inner();
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| Error::from(actix_web::error::PayloadError::Overflow))?;
        size += chunk.len() as u64;
        if size > MAX_FILE_SIZE {
            return Ok(HttpResponse::PayloadTooLarge().json(serde_json::json!({"error": "File too large"})));
        }
        buffer.extend_from_slice(&chunk);
    }
    
    let upload_path = ensure_upload_dir().map_err(|_| {
        error!("Failed to ensure upload dir");
        actix_web::error::ErrorInternalServerError("Failed to ensure upload dir")
    })?;
    
    let file_path = upload_path.join(&filename_str);
    fs::write(&file_path, &buffer).map_err(|e| {
        error!("Failed to write file: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to write file")
    })?;
    
    debug!("File uploaded: {}, size: {}", filename_str, size);
    
    Ok(HttpResponse::Ok().json(UploadResponse {
        success: true,
        filename: filename_str.clone(),
        path: format!("/uploads/{}", filename_str),
        size,
        content_type: "application/octet-stream".to_string(),
    }))
}

// 下载文件接口
pub async fn download_file(
    filename: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let filename_str = filename.into_inner();
    if filename_str.contains("..") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid filename"})));
    }
    
    let upload_path = ensure_upload_dir().map_err(|_| {
        error!("Failed to ensure upload dir");
        actix_web::error::ErrorInternalServerError("Failed to ensure upload dir")
    })?;
    
    let file_path = upload_path.join(&filename_str);
    if !file_path.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "File not found"})));
    }
    
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename_str))
        .body(fs::read(&file_path).map_err(|e| {
            error!("Failed to read file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to read file")
        })?))
}

// 删除文件接口
pub async fn delete_file(
    filename: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let filename_str = filename.into_inner();
    if filename_str.contains("..") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid filename"})));
    }
    
    let upload_path = ensure_upload_dir().map_err(|_| {
        error!("Failed to ensure upload dir");
        actix_web::error::ErrorInternalServerError("Failed to ensure upload dir")
    })?;
    
    let file_path = upload_path.join(&filename_str);
    if !file_path.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "File not found"})));
    }
    
    fs::remove_file(&file_path).map_err(|e| {
        error!("Failed to delete file: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to delete file")
    })?;
    
    Ok(HttpResponse::Ok().json(DeleteResponse { success: true, deleted: filename_str }))
}

// 列出文件接口
pub async fn list_files() -> Result<HttpResponse, Error> {
    let upload_path = ensure_upload_dir().map_err(|_| {
        error!("Failed to ensure upload dir");
        actix_web::error::ErrorInternalServerError("Failed to ensure upload dir")
    })?;
    
    let mut files = Vec::new();
    for entry in fs::read_dir(&upload_path).map_err(|_| {
        error!("Failed to read directory");
        actix_web::error::ErrorInternalServerError("Failed to read directory")
    })? {
        let entry = entry.map_err(|_| {
            error!("Failed to read entry");
            actix_web::error::ErrorInternalServerError("Failed to read entry")
        })?;
        
        if entry.path().is_file() {
            if let Some(name) = entry.path().file_name() {
                if let Some(name_str) = name.to_str() {
                    let metadata = fs::metadata(entry.path()).ok();
                    let modified = metadata.as_ref().and_then(|m| m.modified().ok())
                        .map(|t| t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()).unwrap_or(0);
                    files.push(FileMeta {
                        filename: name_str.to_string(),
                        size: metadata.map(|m| m.len()).unwrap_or(0),
                        modified,
                    });
                }
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(ListResponse { files }))
}
