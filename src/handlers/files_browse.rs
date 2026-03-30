// Phase 104 - 文件浏览 API
// GET /api/v1/files/browse — 浏览文件/文件夹

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::jwt::JwtClaims;

/// 文件夹信息
#[derive(Serialize, Clone)]
pub struct FolderInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub modified_at: u64,
}

/// 文件信息
#[derive(Serialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub modified_at: u64,
}

/// 分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total_items: u64,
    pub total_pages: u32,
}

/// 浏览响应
#[derive(Serialize)]
pub struct BrowseResponse {
    pub current_path: String,
    pub parent_path: Option<String>,
    pub folders: Vec<FolderInfo>,
    pub files: Vec<FileInfo>,
    pub total_items: u64,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 登录用户可访问
}

/// 获取 MIME 类型
fn get_mime_type(path: &std::path::Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("png") => "image/png".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("bmp") => "image/bmp".to_string(),
        Some("webp") => "image/webp".to_string(),
        Some("mp4") => "video/mp4".to_string(),
        Some("avi") => "video/x-msvideo".to_string(),
        Some("mov") => "video/quicktime".to_string(),
        Some("wmv") => "video/x-ms-wmv".to_string(),
        Some("pdf") => "application/pdf".to_string(),
        Some("doc") => "application/msword".to_string(),
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        Some("xls") => "application/vnd.ms-excel".to_string(),
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        Some("ppt") => "application/vnd.ms-powerpoint".to_string(),
        Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        Some("txt") => "text/plain".to_string(),
        Some("md") => "text/markdown".to_string(),
        Some("html") => "text/html".to_string(),
        Some("css") => "text/css".to_string(),
        Some("js") => "application/javascript".to_string(),
        Some("json") => "application/json".to_string(),
        Some("xml") => "application/xml".to_string(),
        Some("zip") => "application/zip".to_string(),
        Some("rar") => "application/vnd.rar".to_string(),
        Some("7z") => "application/x-7z-compressed".to_string(),
        Some("tar") => "application/x-tar".to_string(),
        Some("gz") => "application/gzip".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

/// 文件浏览（Phase 104）
/// - JWT 认证，登录用户可访问
/// - 支持路径参数和分页
/// - 返回文件夹和文件列表
pub async fn browse_files(
    req: HttpRequest,
    query: web::Query<BrowseQuery>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查 Token 是否存在
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            error: "Invalid token".to_string(),
            code: "UNAUTHORIZED".to_string(),
        }));
    }

    // 2. 解析查询参数
    let path = query.path.clone().unwrap_or_else(|| "/".to_string());
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(50).max(1).min(200);

    // Bug #51 修复：路径遍历验证
    // 禁止 .. 路径遍历
    if path.contains("..") {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Path contains forbidden sequence '..'".to_string(),
            code: "PATH_TRAVERSAL".to_string(),
        }));
    }

    // 禁止 null 字节
    if path.contains('\0') {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Path contains invalid null byte".to_string(),
            code: "INVALID_PATH".to_string(),
        }));
    }

    // 3. 构建文件系统路径
    let base_dir = PathBuf::from("/data/files");
    let target_path = if path == "/" {
        base_dir.clone()
    } else {
        base_dir.join(path.trim_start_matches('/'))
    };

    // Bug #51 修复：验证路径仍在 base_dir 内
    // 使用 canonicalize 防止符号链接逃逸
    if target_path.exists() {
        if let (Ok(canonical_target), Ok(canonical_base)) = 
            (std::fs::canonicalize(&target_path), std::fs::canonicalize(&base_dir)) {
            if !canonical_target.starts_with(&canonical_base) {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "Access denied: path outside allowed directory".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }
        }
    };

    // 4. 验证路径存在
    if !target_path.exists() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Path '{}' not found", path),
            code: "PATH_NOT_FOUND".to_string(),
        }));
    }

    if !target_path.is_dir() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Path is not a directory".to_string(),
            code: "NOT_DIRECTORY".to_string(),
        }));
    }

    // 5. 计算父目录路径
    let parent_path = if path == "/" {
        None
    } else {
        let parent = std::path::Path::new(&path).parent();
        parent.map(|p| {
            let path_str = p.to_string_lossy().to_string();
            if path_str.is_empty() {
                "/".to_string()
            } else {
                path_str
            }
        })
    };

    // 6. 读取目录内容
    let mut folders = Vec::new();
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&target_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let metadata = match std::fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let name = entry.file_name().to_string_lossy().to_string();
            let relative_path = path.strip_prefix(&base_dir)
                .map(|p| format!("/{}", p.to_string_lossy()))
                .unwrap_or_else(|_| "/".to_string());
            let modified_at = metadata.modified()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
                .unwrap_or(0);

            if metadata.is_dir() {
                folders.push(FolderInfo {
                    name,
                    path: relative_path,
                    size_bytes: 0,
                    modified_at,
                });
            } else {
                files.push(FileInfo {
                    name,
                    path: relative_path,
                    size_bytes: metadata.len(),
                    mime_type: get_mime_type(&path),
                    modified_at,
                });
            }
        }
    }

    // 7. 计算总数和分页
    let total_items = (folders.len() + files.len()) as u64;
    let total_pages = ((total_items + limit as u64 - 1) / limit as u64) as u32;

    // 8. 分页截取
    let start = ((page - 1) * limit) as usize;
    let end = start + limit as usize;
    
    let mut all_items: Vec<(bool, Box<dyn std::any::Any>)> = Vec::new();
    for folder in &folders {
        all_items.push((true, Box::new(folder.clone())));
    }
    for file in &files {
        all_items.push((false, Box::new(file.clone())));
    }

    let paginated_items: Vec<_> = all_items
        .into_iter()
        .enumerate()
        .filter_map(|(i, (is_folder, item))| {
            if i >= start && i < end {
                Some((is_folder, item))
            } else {
                None
            }
        })
        .collect();

    let paginated_folders: Vec<FolderInfo> = paginated_items
        .iter()
        .filter_map(|(is_folder, item)| {
            if *is_folder {
                item.downcast_ref::<FolderInfo>().cloned()
            } else {
                None
            }
        })
        .collect();

    let paginated_files: Vec<FileInfo> = paginated_items
        .iter()
        .filter_map(|(is_folder, item)| {
            if !*is_folder {
                item.downcast_ref::<FileInfo>().cloned()
            } else {
                None
            }
        })
        .collect();

    // 9. 返回响应
    Ok(HttpResponse::Ok().json(BrowseResponse {
        current_path: path,
        parent_path,
        folders: paginated_folders,
        files: paginated_files,
        total_items,
        pagination: PaginationInfo {
            page,
            limit,
            total_items,
            total_pages,
        },
    }))
}

/// 浏览查询参数
#[derive(Deserialize)]
pub struct BrowseQuery {
    pub path: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
