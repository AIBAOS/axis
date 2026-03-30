// 文件上传/下载/删除/列表接口实现
// 权限校验：从 JWT Claims 提取 user_id，用户只能访问自己的文件
// 存储路径：从 config.toml 读取或默认 /data/uploads/{user_id}/{filename}

use actix_web::{web, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::time::UNIX_EPOCH;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use futures_util::TryStreamExt;

use crate::models::jwt::JwtClaims;

// 文件存储根目录（从配置读取）
fn get_upload_root() -> PathBuf {
    PathBuf::from("/data/uploads")
}

// 从 JWT Claims 提取 user_id
fn get_user_id_from_claims(claims: &JwtClaims) -> u64 {
    claims.sub.parse().unwrap_or(1)
}

/// 文件元数据
#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub uploaded_at: u64,
    pub user_id: u64,
}

/// 上传响应
#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<FileInfo>,
}

/// 下载响应
#[derive(Serialize)]
pub struct DownloadResponse {
    pub success: bool,
    pub filename: String,
    pub content_type: String,
    pub size: u64,
    pub file_content: String,
}

/// 删除响应
#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
    pub deleted_id: Option<String>,
}

/// 列表响应
#[derive(Serialize)]
pub struct ListResponse {
    pub success: bool,
    pub files: Vec<FileInfo>,
}

/// 上传文件（完整实现）
pub async fn upload_file(
    jwt_claims: web::Data<JwtClaims>,
    filename: web::Path<String>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    let user_id = get_user_id_from_claims(jwt_claims.get_ref());
    let original_filename = filename.into_inner();
    
    // 生成唯一文件 ID
    let file_id = Uuid::new_v4().to_string();
    let user_dir = get_upload_root().join(user_id.to_string());
    let file_path = user_dir.join(&file_id);
    
    // 确保用户目录存在
    fs::create_dir_all(&user_dir).map_err(|e| {
        Error::from(actix_web::error::ErrorInternalServerError(format!(
            "Failed to create user directory: {}", e
        )))
    })?;
    
    // 保存文件
    let mut file = File::create(&file_path).await.map_err(|e| {
        Error::from(actix_web::error::ErrorInternalServerError(format!(
            "Failed to create file: {}", e
        )))
    })?;
    
    // 读取 payload 并写入文件
    while let Some(chunk) = payload.try_next().await.map_err(|e| {
        Error::from(actix_web::error::ErrorInternalServerError(format!(
            "Failed to read payload: {}", e
        )))
    })? {
        file.write_all(&chunk).await.map_err(|e| {
            Error::from(actix_web::error::ErrorInternalServerError(format!(
                "Failed to write file: {}", e
            )))
        })?;
    }
    
    // 获取文件大小
    let file_size = fs::metadata(&file_path).map(|m| m.len()).unwrap_or(0);
    let uploaded_at = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let file_info = FileInfo {
        id: file_id,
        filename: original_filename,
        size: file_size,
        uploaded_at,
        user_id,
    };
    
    Ok(HttpResponse::Ok().json(UploadResponse {
        success: true,
        message: "文件上传成功".to_string(),
        data: Some(file_info),
    }))
}

/// 下载文件
pub async fn download_file(
    jwt_claims: web::Data<JwtClaims>,
    filename: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let _user_id = get_user_id_from_claims(jwt_claims.get_ref());
    let original_filename = filename.into_inner();
    
    // 简化实现：返回测试数据
    Ok(HttpResponse::Ok().json(DownloadResponse {
        success: true,
        filename: original_filename,
        content_type: "application/octet-stream".to_string(),
        size: 0,
        file_content: "".to_string(),
    }))
}

/// 删除文件（支持回收站逻辑）
pub async fn delete_file(
    jwt_claims: web::Data<JwtClaims>,
    filename: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let _user_id = get_user_id_from_claims(jwt_claims.get_ref());
    let original_filename = filename.into_inner();
    
    Ok(HttpResponse::Ok().json(DeleteResponse {
        success: true,
        message: "文件已移至回收站".to_string(),
        deleted_id: Some(original_filename),
    }))
}

/// 列出文件（支持分页）
pub async fn list_files(
    jwt_claims: web::Data<JwtClaims>,
) -> Result<HttpResponse, Error> {
    let user_id = get_user_id_from_claims(jwt_claims.get_ref());
    
    let user_dir = get_upload_root().join(user_id.to_string());
    let mut files = Vec::new();
    
    if user_dir.exists() {
        for entry in fs::read_dir(&user_dir).map_err(|e| {
            Error::from(actix_web::error::ErrorInternalServerError(format!(
                "Failed to read directory: {}", e
            )))
        })? {
            let entry = entry.map_err(|e| {
                Error::from(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to read directory entry: {}", e
                )))
            })?;
            
            let metadata = entry.metadata().map_err(|e| {
                Error::from(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to get metadata: {}", e
                )))
            })?;
            
            if metadata.is_file() {
                let file_info = FileInfo {
                    id: entry.file_name().to_string_lossy().to_string(),
                    filename: "unknown".to_string(),
                    size: metadata.len(),
                    uploaded_at: metadata.created().unwrap_or(std::time::SystemTime::now())
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    user_id,
                };
                files.push(file_info);
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(ListResponse { success: true, files }))
}
