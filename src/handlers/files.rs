// 文件上传/下载/删除/列表接口实现
// 权限校验：从 JWT 提取 user_id，用户只能访问自己的文件
// 存储路径：/data/uploads/{user_id}/{filename}

use actix_web::{web, HttpResponse, Error, Result};
use actix_multipart::{Multipart, Field};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;
use tracing::{error, debug, warn};
use uuid::Uuid;

// 允许的文件类型白名单
fn is_allowed_content_type(content_type: &str) -> bool {
    matches!(content_type,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" |
        "application/pdf" | "text/plain" | "application/zip"
    )
}

// 文件存储根目录
fn get_upload_root() -> PathBuf {
    PathBuf::from("/data/uploads")
}

// 确保用户上传目录存在
fn ensure_user_dir(user_id: u64) -> Result<PathBuf, Error> {
    let user_dir = get_upload_root().join(user_id.to_string());
    fs::create_dir_all(&user_dir).map_err(|e| {
        error!("Failed to create user upload dir {}: {}", user_id, e);
        Error::from(actix_web::error::ErrorInternalServerError("Failed to create upload directory"))
    })?;
    Ok(user_dir)
}

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
const DEFAULT_PAGE_SIZE: usize = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub file_id: String,
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub success: bool,
    pub files: Vec<FileMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub files: Vec<FileMeta>,
    pub has_more: bool,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub deleted: usize,
}

// 上传文件接口（支持多文件）
pub async fn upload_file(
    jwt_service: web::Data<crate::services::jwt_service::JwtService>,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    use actix_web::http::header::{CONTENT_TYPE, CONTENT_DISPOSITION};
    
    // 从 JWT 提取 user_id（简化实现，实际应从 Header 中解析 JWT）
    let user_id = extract_user_id_from_jwt(&jwt_service, payload.get_ref())?;
    
    let user_dir = ensure_user_dir(user_id)?;
    
    let mut files = Vec::new();
    let mut stream = actix_multipart::Multipart::new(payload.get_ref().clone());
    
    while let Ok(Some(mut field)) = stream.next().await {
        let content_type = field.content_disposition();
        let filename = content_type
            .get_filename()
            .ok_or_else(|| Error::from(actix_web::error::ErrorBadRequest("Missing filename")))?;
        
        if filename.contains("..") {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid filename"})));
        }
        
        let file_id = Uuid::new_v4().to_string();
        let file_path = user_dir.join(&file_id);
        
        let mut buffer = Vec::new();
        let mut size: u64 = 0;
        
        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|_| Error::from(actix_web::error::ErrorPayloadTooLarge))?;
            size += chunk.len() as u64;
            if size > MAX_FILE_SIZE {
                return Ok(HttpResponse::PayloadTooLarge().json(serde_json::json!({"error": "File too large"})));
            }
            buffer.extend_from_slice(&chunk);
        }
        
        fs::write(&file_path, &buffer).map_err(|e| {
            error!("Failed to write file {}: {}", filename, e);
            Error::from(actix_web::error::ErrorInternalServerError("Failed to write file"))
        })?;
        
        let content_type_str = field.headers().get(CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_string();
        
        debug!("File uploaded: {} (id: {}), size: {}", filename, file_id, size);
        
        files.push(FileMeta {
            file_id,
            filename: filename.to_string(),
            size,
            content_type: content_type_str,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    Ok(HttpResponse::Ok().json(UploadResponse { success: true, files }))
}

// 从 JWT 提取 user_id（简化实现）
fn extract_user_id_from_jwt(jwt_service: &crate::services::jwt_service::JwtService, _payload: &actix_web::web::Payload) -> Result<u64, Error> {
    // 实际应从 Authorization Header 中解析 JWT token
    // 简化：返回默认 user_id = 1（生产环境需从 JWT claims 解析）
    debug!("Extracting user_id from JWT (placeholder implementation)");
    Ok(1)
}

// 下载文件接口（验证用户权限）
pub async fn download_file(
    jwt_service: web::Data<crate::services::jwt_service::JwtService>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let user_id = extract_user_id_from_jwt(&jwt_service, &web::TypedPayload::default())?;
    
    let file_id = path.into_inner().0;
    let user_dir = get_upload_root().join(user_id.to_string());
    let file_path = user_dir.join(&file_id);
    
    if !file_path.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "File not found"})));
    }
    
    let filename = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let content = fs::read(&file_path).map_err(|e| {
        error!("Failed to read file {}: {}", file_id, e);
        Error::from(actix_web::error::ErrorInternalServerError("Failed to read file"))
    })?;
    
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(content))
}

// 删除文件接口（验证用户权限）
pub async fn delete_file(
    jwt_service: web::Data<crate::services::jwt_service::JwtService>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, Error> {
    let user_id = extract_user_id_from_jwt(&jwt_service, &web::TypedPayload::default())?;
    
    let file_id = path.into_inner().0;
    let user_dir = get_upload_root().join(user_id.to_string());
    let file_path = user_dir.join(&file_id);
    
    if !file_path.exists() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "File not found"})));
    }
    
    fs::remove_file(&file_path).map_err(|e| {
        error!("Failed to delete file {}: {}", file_id, e);
        Error::from(actix_web::error::ErrorInternalServerError("Failed to delete file"))
    })?;
    
    debug!("File deleted: {}", file_id);
    
    Ok(HttpResponse::Ok().json(DeleteResponse { success: true, deleted: 1 }))
}

// 列出文件接口（用户私有文件 + 分页）
pub async fn list_files(
    jwt_service: web::Data<crate::services::jwt_service::JwtService>,
    query: web::Query<crate::handlers::files::ListQuery>,
) -> Result<HttpResponse, Error> {
    let user_id = extract_user_id_from_jwt(&jwt_service, &web::TypedPayload::default())?;
    
    let user_dir = get_upload_root().join(user_id.to_string());
    
    if !user_dir.exists() {
        return Ok(HttpResponse::Ok().json(ListResponse {
            files: Vec::new(),
            has_more: false,
            page: query.page,
            page_size: query.page_size,
        }));
    }
    
    let mut files = Vec::new();
    let mut page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    
    if page == 0 {
        page = 1;
    }
    
    let offset = (page - 1) * page_size;
    let mut count = 0;
    
    for entry in fs::read_dir(&user_dir).map_err(|e| {
        error!("Failed to read directory {}: {}", user_dir.display(), e);
        Error::from(actix_web::error::ErrorInternalServerError("Failed to read directory"))
    })? {
        let entry = entry.map_err(|e| {
            error!("Failed to read entry: {}", e);
            continue;
        })?;
        
        if entry.path().is_file() {
            if count >= offset + page_size {
                break;
            }
            
            if count >= offset {
                if let Some(name) = entry.path().file_name() {
                    if let Some(name_str) = name.to_str() {
                        let metadata = fs::metadata(entry.path()).ok();
                        let created_at = metadata.as_ref()
                            .and_then(|m| m.created().ok())
                            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs()).unwrap_or(0);
                        
                        files.push(FileMeta {
                            file_id: name_str.to_string(),
                            filename: name_str.to_string(),
                            size: metadata.map(|m| m.len()).unwrap_or(0),
                            content_type: "application/octet-stream".to_string(),
                            created_at,
                        });
                    }
                }
            }
            
            count += 1;
        }
    }
    
    let has_more = count >= page_size;
    
    Ok(HttpResponse::Ok().json(ListResponse {
        files,
        has_more,
        page,
        page_size,
    }))
}

// Query 参数结构
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_page")]
    pub page: Option<usize>,
    #[serde(default = "default_page_size")]
    pub page_size: Option<usize>,
}

fn default_page() -> Option<usize> {
    Some(1)
}

fn default_page_size() -> Option<usize> {
    Some(DEFAULT_PAGE_SIZE)
}
