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
const MIN_FILE_SIZE: u64 = 1; // 最小 1 字节，防止空文件
const ALLOWED_EXTENSIONS: [&str; 10] = ["txt", "pdf", "doc", "docx", "xls", "xlsx", "jpg", "jpeg", "png", "gif"];

/// 路径安全验证（防止路径遍历攻击）
fn validate_path(path: &str) -> Result<(), ErrorResponse> {
    // 必须是绝对路径
    if !path.starts_with('/') {
        return Err(ErrorResponse {
            success: false,
            error: "Path must be absolute (start with /)".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    // 禁止路径遍历
    if path.contains("..") {
        return Err(ErrorResponse {
            success: false,
            error: "Path contains forbidden sequence '..'".to_string(),
            code: "PATH_TRAVERSAL".to_string(),
        });
    }
    // 禁止 null 字节
    if path.contains('\0') {
        return Err(ErrorResponse {
            success: false,
            error: "Path contains invalid null byte".to_string(),
            code: "INVALID_PATH".to_string(),
        });
    }
    Ok(())
}

/// 文件名安全验证（过滤危险字符）
fn validate_filename(filename: &str) -> Result<String, ErrorResponse> {
    // 禁止空文件名
    if filename.is_empty() {
        return Err(ErrorResponse {
            success: false,
            error: "Filename cannot be empty".to_string(),
            code: "INVALID_FILENAME".to_string(),
        });
    }
    // Bug #50 修复：限制文件名长度 (最大 255 字符)
    if filename.len() > 255 {
        return Err(ErrorResponse {
            success: false,
            error: "Filename cannot exceed 255 characters".to_string(),
            code: "INVALID_FILENAME".to_string(),
        });
    }
    // 禁止路径分隔符
    if filename.contains('/') || filename.contains('\\') {
        return Err(ErrorResponse {
            success: false,
            error: "Filename cannot contain path separators (/ or \\)".to_string(),
            code: "INVALID_FILENAME".to_string(),
        });
    }
    // 禁止路径遍历
    if filename.contains("..") {
        return Err(ErrorResponse {
            success: false,
            error: "Filename cannot contain '..'".to_string(),
            code: "PATH_TRAVERSAL".to_string(),
        });
    }
    // 禁止 null 字节
    if filename.contains('\0') {
        return Err(ErrorResponse {
            success: false,
            error: "Filename contains invalid null byte".to_string(),
            code: "INVALID_FILENAME".to_string(),
        });
    }
    // 禁止控制字符
    if filename.chars().any(|c| c.is_control()) {
        return Err(ErrorResponse {
            success: false,
            error: "Filename contains control characters".to_string(),
            code: "INVALID_FILENAME".to_string(),
        });
    }
    // 返回清理后的文件名（仅保留安全字符）
    let safe_filename = filename
        .chars()
        .filter(|c| !c.is_control() && *c != '/' && *c != '\\' && *c != '\0')
        .collect::<String>();
    Ok(safe_filename)
}

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
                
                // 检查文件大小（上限）
                if data.len() as u64 > MAX_FILE_SIZE {
                    return Ok(HttpResponse::PayloadTooLarge().json(ErrorResponse {
                        success: false,
                        error: format!("File size exceeds limit of {} MB", MAX_FILE_SIZE / 1024 / 1024),
                        code: "FILE_TOO_LARGE".to_string(),
                    }));
                }
                
                // 检查文件大小（下限 - Bug #18 修复）
                if (data.len() as u64) < MIN_FILE_SIZE {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: "Empty files are not allowed".to_string(),
                        code: "FILE_EMPTY".to_string(),
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
    let raw_filename = filename.ok_or_else(|| actix_web::error::ErrorBadRequest("filename is required"))?;

    // 4.1 验证文件名安全性（Bug #17 修复）
    let safe_filename = match validate_filename(&raw_filename) {
        Ok(name) => name,
        Err(e) => return Ok(HttpResponse::BadRequest().json(e)),
    };

    // 5. 验证文件类型
    let extension = safe_filename.split('.').last().unwrap_or("").to_lowercase();
    if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("File type '{}' not supported. Allowed types: {}", extension, ALLOWED_EXTENSIONS.join(", ")),
            code: "UNSUPPORTED_FILE_TYPE".to_string(),
        }));
    }

    // 6. 确定保存路径（默认用户主目录）
    let raw_upload_dir = target_path
        .unwrap_or_else(|| format!("/users/{}", username));
    
    // 6.1 验证路径安全性（Bug #16 修复）
    if let Err(e) = validate_path(&raw_upload_dir) {
        return Ok(HttpResponse::BadRequest().json(e));
    }
    let upload_dir = raw_upload_dir;
    
    // 7. 处理文件名冲突（自动重命名）
    let file_path = PathBuf::from(&upload_dir).join(&safe_filename);
    let final_path = if file_path.exists() {
        // 文件已存在，自动重命名
        let stem = safe_filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(&safe_filename);
        let ext = safe_filename.rsplit_once('.').map(|(_, e)| e).unwrap_or("");
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
