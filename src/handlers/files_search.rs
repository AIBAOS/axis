// Phase 46 - 文件搜索 API
// 支持按名称/类型/时间筛选的文件搜索功能

use actix_web::{web, HttpResponse, Error};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::UNIX_EPOCH;

use crate::models::jwt::JwtClaims;

// 文件存储根目录
fn get_file_root() -> PathBuf {
    PathBuf::from("/data/files")
}

// 从 JWT Claims 提取 user_id
fn get_user_id_from_claims(claims: &JwtClaims) -> u64 {
    claims.sub.parse().unwrap_or(1)
}

// 获取用户文件目录
fn get_user_file_dir(user_id: u64) -> PathBuf {
    get_file_root().join(user_id.to_string())
}

/// 文件信息
#[derive(Debug, serde::Serialize, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub file_type: String, // "file", "folder", "image", "video", "document"
    pub size: u64,
    pub modified: u64,
}

/// 搜索查询参数
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// 搜索关键词（模糊匹配文件名）
    pub q: Option<String>,
    /// 搜索起始路径（默认用户根目录）
    pub path: Option<String>,
    /// 是否递归搜索（默认 false）
    #[serde(default)]
    pub recursive: bool,
    /// 文件类型筛选：file/folder/image/video/document
    #[serde(rename = "file_type")]
    pub file_type: Option<String>,
    /// 修改时间之后（Unix 时间戳）
    #[serde(rename = "modified_after")]
    pub modified_after: Option<u64>,
    /// 修改时间之前（Unix 时间戳）
    #[serde(rename = "modified_before")]
    pub modified_before: Option<u64>,
    /// 分页：每页数量（默认 50，最大 200）
    pub limit: Option<u32>,
    /// 分页：偏移量（默认 0）
    pub offset: Option<u32>,
}

/// 搜索结果响应
#[derive(Debug, serde::Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub data: Vec<FileInfo>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// 判断文件类型
fn detect_file_type(path: &Path, is_dir: bool) -> String {
    if is_dir {
        return "folder".to_string();
    }
    
    let extension = path.extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    
    match extension.as_str() {
        // 图片
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "ico" => "image".to_string(),
        // 视频
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v" => "video".to_string(),
        // 文档
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "rtf" => "document".to_string(),
        // 默认文件
        _ => "file".to_string(),
    }
}

/// 递归搜索文件
fn search_recursive(
    dir: &Path,
    query: &Option<String>,
    file_type_filter: &Option<String>,
    modified_after: &Option<u64>,
    modified_before: &Option<u64>,
    results: &mut Vec<FileInfo>,
    base_dir: &Path,
    current_depth: usize,
    max_depth: usize,
) -> Result<(), Error> {
    // 深度限制
    if current_depth > max_depth {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir).map_err(|e| {
        log::warn!("Failed to read directory {}: {}", dir.display(), e);
        actix_web::error::ErrorInternalServerError(format!("Read directory error: {}", e))
    })?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        
        let metadata = match fs::metadata(&path) {
            Ok(md) => md,
            Err(e) => {
                log::warn!("Failed to get metadata for {}: {}", path.display(), e);
                continue;
            }
        };
        
        let is_dir = metadata.is_dir();
        
        // 获取相对路径
        let relative_path = path.strip_prefix(base_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        
        // 获取文件名
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        
        // 关键词匹配（模糊匹配）
        if let Some(q) = query {
            if !q.is_empty() && !name.to_lowercase().contains(&q.to_lowercase()) {
                // 如果是目录且递归搜索，继续深入
                if is_dir {
                    let _ = search_recursive(
                        &path,
                        query,
                        file_type_filter,
                        modified_after,
                        modified_before,
                        results,
                        base_dir,
                        current_depth + 1,
                        max_depth,
                    );
                }
                continue;
            }
        }
        
        // 文件类型筛选
        let detected_type = detect_file_type(&path, is_dir);
        if let Some(ref filter_type) = file_type_filter {
            if filter_type != &detected_type {
                // 类型不匹配，跳过
                if is_dir {
                    let _ = search_recursive(
                        &path,
                        query,
                        file_type_filter,
                        modified_after,
                        modified_before,
                        results,
                        base_dir,
                        current_depth + 1,
                        max_depth,
                    );
                }
                continue;
            }
        }
        
        // 修改时间筛选
        let modified = metadata.modified()
            .unwrap_or(std::time::SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        if let Some(after) = modified_after {
            if modified < *after {
                continue;
            }
        }
        
        if let Some(before) = modified_before {
            if modified > *before {
                continue;
            }
        }
        
        // 添加到结果
        results.push(FileInfo {
            name: name.clone(),
            path: if relative_path.is_empty() { "/".to_string() } else { relative_path },
            file_type: detected_type,
            size: metadata.len(),
            modified,
        });
        
        // 如果是目录且递归搜索，继续深入
        if is_dir {
            let _ = search_recursive(
                &path,
                query,
                file_type_filter,
                modified_after,
                modified_before,
                results,
                base_dir,
                current_depth + 1,
                max_depth,
            );
        }
    }
    
    Ok(())
}

/// 文件搜索接口
pub async fn search_files(
    jwt_claims: web::Data<JwtClaims>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, Error> {
    let user_id = get_user_id_from_claims(jwt_claims.get_ref());
    let base_dir = get_user_file_dir(user_id);
    
    // 确保用户目录存在
    if !base_dir.exists() {
        return Ok(HttpResponse::Ok().json(SearchResponse {
            success: true,
            data: vec![],
            total: 0,
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
        }));
    }
    
    // 确定搜索起始路径
    let start_path = if let Some(ref path) = query.path {
        let clean_path = path.trim_start_matches('/');
        // 安全检查：防止目录遍历
        if clean_path.contains("..") {
            return Err(actix_web::error::ErrorBadRequest("Path traversal not allowed"));
        }
        base_dir.join(clean_path)
    } else {
        base_dir.clone()
    };
    
    // 验证起始路径存在且为目录
    if !start_path.exists() {
        return Ok(HttpResponse::Ok().json(SearchResponse {
            success: true,
            data: vec![],
            total: 0,
            limit: query.limit.unwrap_or(50),
            offset: query.offset.unwrap_or(0),
        }));
    }
    
    if !start_path.is_dir() {
        return Err(actix_web::error::ErrorBadRequest("Search path must be a directory"));
    }
    
    // 安全检查：确保路径在用户目录内
    let canonical_base = base_dir.canonicalize().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Base dir error: {}", e))
    })?;
    
    if start_path.exists() {
        let canonical_start = start_path.canonicalize().map_err(|e| {
            actix_web::error::ErrorBadRequest(format!("Invalid path: {}", e))
        })?;
        if !canonical_start.starts_with(&canonical_base) {
            return Err(actix_web::error::ErrorBadRequest("Path traversal detected"));
        }
    }
    
    // 解析分页参数
    let limit = query.limit.unwrap_or(50).min(200); // 最大 200
    let offset = query.offset.unwrap_or(0);
    
    // 执行搜索
    let mut results: Vec<FileInfo> = Vec::new();
    let max_depth = if query.recursive { 10 } else { 0 };
    
    search_recursive(
        &start_path,
        &query.q,
        &query.file_type,
        &query.modified_after,
        &query.modified_before,
        &mut results,
        &base_dir,
        0,
        max_depth,
    )?;
    
    let total = results.len() as u32;
    
    // 分页截取
    let start = offset as usize;
    let end = (offset + limit) as usize;
    let paginated_results = if start < results.len() {
        results.into_iter()
            .skip(start)
            .take(end - start)
            .collect()
    } else {
        vec![]
    };
    
    Ok(HttpResponse::Ok().json(SearchResponse {
        success: true,
        data: paginated_results,
        total,
        limit,
        offset,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_file_type() {
        // 测试文件夹
        assert_eq!(detect_file_type(Path::new("/test"), true), "folder");
        
        // 测试图片
        assert_eq!(detect_file_type(Path::new("/test.jpg"), false), "image");
        assert_eq!(detect_file_type(Path::new("/test.PNG"), false), "image");
        
        // 测试视频
        assert_eq!(detect_file_type(Path::new("/test.mp4"), false), "video");
        
        // 测试文档
        assert_eq!(detect_file_type(Path::new("/test.pdf"), false), "document");
        assert_eq!(detect_file_type(Path::new("/test.md"), false), "document");
        
        // 测试普通文件
        assert_eq!(detect_file_type(Path::new("/test.zip"), false), "file");
    }
}
