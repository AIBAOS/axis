// Phase 119 - 文件上传 API
// POST /api/v1/files/upload — 上传文件

use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use serde::Serialize;
use std::io::Write as _;
use std::path::PathBuf;
use std::fs;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 文件上传响应
#[derive(Serialize)]
pub struct FileUploadResponse {
    pub success: bool,
    pub message: String,
    pub data: UploadedFileInfo,
}

/// 已上传文件信息
#[derive(Serialize)]
pub struct UploadedFileInfo {
    pub filename: String,
    pub path: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub uploaded_at: u64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

// 配置常量
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
const ALLOWED_EXTENSIONS: [&str; 10] = ["txt", "pdf", "doc", "docx", "xls", "xlsx", "jpg", "jpeg", "png", "gif"];

/// 上传文件（Phase 119）
/// - JWT 认证，登录用户可访问
/// - multipart/form-data 格式
/// - 验证文件类型和大小
/// - 保存到指定路径
/// - 处理文件名冲突
pub async fn upload_file(
    req: HttpRequest,
    mut payload: Multipart,
    _rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 获取当前用户信息
    let _user_id = claims.sub.parse().unwrap_or(0);
    let username = claims.sub.clone();

    // 3. 解析 multipart 表单数据
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut target_path: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let cd_opt = field.content_disposition();
        let field_name = cd_opt.and_then(|cd| cd.get_name().map(|s| s.to_string()));
        let file_fname = cd_opt.and_then(|cd| cd.get_filename().map(|s| s.to_string()));
        
        match field_name.as_deref() {
            Some("file") => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk?;
                    data.extend_from_slice(&chunk);
                }
                
                // 检查文件大小
                if data.len() as u64 > MAX_FILE_SIZE {
                    return Ok(HttpResponse::PayloadTooLarge().json(ErrorResponse {
                        success: false,
                        error: format!("File size exceeds limit of {} MB", MAX_FILE_SIZE / 1024 / 1024),
                        code: "FILE_TOO_LARGE".to_string(),
                    }));
                }
                
                file_data = Some(data);
                filename = file_fname;
                content_type = field.content_type().map(|ct| ct.to_string());
            }
            Some("path") => {
                let mut data = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk?;
                    data.extend_from_slice(&chunk);
                }
                if let Ok(path) = String::from_utf8(data) {
                    target_path = Some(path);
                }
            }
            _ => {}
        }
    }

    // 4. 验证必要参数
    let file_data = file_data.ok_or_else(|| actix_web::error::ErrorBadRequest("file is required"))?;
    let filename = filename.ok_or_else(|| actix_web::error::ErrorBadRequest("filename is required"))?;

    // 5. 验证文件类型
    let extension = filename.split('.').last().unwrap_or("").to_lowercase();
    if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("File type '{}' not supported. Allowed types: {}", extension, ALLOWED_EXTENSIONS.join(", ")),
            code: "UNSUPPORTED_FILE_TYPE".to_string(),
        }));
    }

    // 6. 确定保存路径（默认用户主目录）
    let upload_dir = target_path
        .unwrap_or_else(|| format!("/users/{}", username));
    
    // 7. 处理文件名冲突（自动重命名）
    let file_path = PathBuf::from(&upload_dir).join(&filename);
    let final_path = if file_path.exists() {
        // 文件已存在，自动重命名
        let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(&filename);
        let ext = filename.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
        let mut counter = 1;
        loop {
            let new_filename = if ext.is_empty() {
                format!("{}_{}", stem, counter)
            } else {
                format!("{}_{}.{}", stem, counter, ext)
            };
            let new_path = PathBuf::from(&upload_dir).join(&new_filename);
            if !new_path.exists() {
                break PathBuf::from(&upload_dir).join(new_filename);
            }
            counter += 1;
        }
    } else {
        file_path
    };

    // 8. 创建目录（如果不存在）
    if let Some(parent) = final_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create directory: {}", e)))?;
    }

    // 9. 保存文件
    let mut file = fs::File::create(&final_path)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create file: {}", e)))?;
    
    file.write_all(&file_data)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to write file: {}", e)))?;

    // 10. 返回上传结果
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
        .as_secs();

    let uploaded_file = UploadedFileInfo {
        filename: final_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        path: final_path.to_string_lossy().to_string(),
        size_bytes: file_data.len() as u64,
        mime_type: content_type.unwrap_or_else(|| "application/octet-stream".to_string()),
        uploaded_at: now,
    };

    Ok(HttpResponse::Ok().json(FileUploadResponse {
        success: true,
        message: "File uploaded successfully".to_string(),
        data: uploaded_file,
    }))
}
