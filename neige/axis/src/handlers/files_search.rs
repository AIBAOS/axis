//! 文件搜索 Handler - Phase 46
//! 支持按名称/类型/时间筛选，递归搜索深度限制

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use crate::database::file_store::FileRepository;
use crate::middleware::jwt_auth::JwtClaims;

/// 搜索请求参数
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// 搜索关键词（模糊匹配文件名）
    pub q: Option<String>,
    /// 搜索路径（默认为用户根目录）
    pub path: Option<String>,
    /// 是否递归搜索（默认 false）
    pub recursive: Option<bool>,
    /// 文件类型筛选
    pub file_type: Option<FileType>,
    /// 修改时间之后（Unix 时间戳）
    pub modified_after: Option<i64>,
    /// 修改时间之前（Unix 时间戳）
    pub modified_before: Option<i64>,
    /// 分页限制（默认 50，最大 100）
    pub limit: Option<u32>,
    /// 分页偏移
    pub offset: Option<u32>,
}

/// 文件类型枚举
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
    Folder,
    Image,
    Video,
    Document,
}

impl FileType {
    /// 根据扩展名判断文件类型
    pub fn from_extension(ext: &str) -> Option<FileType> {
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" | "svg" => Some(FileType::Image),
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => Some(FileType::Video),
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" => {
                Some(FileType::Document)
            }
            _ => Some(FileType::File),
        }
    }
}

/// 搜索结果项
#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub size: u64,
    pub modified_at: i64,
    pub created_at: i64,
}

/// 搜索响应
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub message: String,
    pub data: SearchData,
}

#[derive(Debug, Serialize)]
pub struct SearchData {
    pub items: Vec<SearchResultItem>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// 最大递归深度
const MAX_RECURSIVE_DEPTH: u32 = 10;

/// 文件搜索处理器
/// 
/// GET /api/v1/files/search
/// 
/// 参数：
/// - q: 搜索关键词（模糊匹配文件名）
/// - path: 搜索路径（可选）
/// - recursive: 是否递归搜索（可选，默认 false）
/// - file_type: 文件类型筛选（可选）
/// - modified_after: 修改时间之后（可选）
/// - modified_before: 修改时间之前（可选）
/// - limit: 分页限制（可选，默认 50，最大 100）
/// - offset: 分页偏移（可选，默认 0）
pub async fn search_files(
    claims: JwtClaims,
    file_repo: web::Data<FileRepository>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, Error> {
    let user_id = claims.user_id;
    
    // 参数验证
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let recursive = query.recursive.unwrap_or(false);
    
    // 路径安全校验（防止目录遍历攻击）
    let search_path = if let Some(ref path) = query.path {
        if !is_safe_path(path) {
            return Ok(HttpResponse::BadRequest().json(SearchResponse {
                success: false,
                message: "无效的路径：检测到目录遍历攻击".to_string(),
                data: SearchData {
                    items: vec![],
                    total: 0,
                    limit,
                    offset,
                },
            }));
        }
        sanitize_path(path)
    } else {
        format!("/users/{}", user_id)
    };
    
    // 执行搜索
    let (items, total) = file_repo
        .search_files(
            &user_id,
            &search_path,
            recursive,
            query.q.as_deref(),
            query.file_type.as_ref(),
            query.modified_after,
            query.modified_before,
            limit,
            offset,
        )
        .await
        .map_err(|e| {
            log::error!("文件搜索失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    Ok(HttpResponse::Ok().json(SearchResponse {
        success: true,
        message: "搜索成功".to_string(),
        data: SearchData {
            items,
            total,
            limit,
            offset,
        },
    }))
}

/// 路径安全校验（防止目录遍历攻击）
fn is_safe_path(path: &str) -> bool {
    // 检查是否包含目录遍历模式
    if path.contains("..") {
        return false;
    }
    
    // 检查是否是绝对路径
    if path.starts_with('/') && !path.starts_with("/users/") && !path.starts_with("/data/") {
        return false;
    }
    
    true
}

/// 路径规范化
fn sanitize_path(path: &str) -> String {
    PathBuf::from(path)
        .components()
        .filter(|c| c.as_os_str() != "..")
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string()
}

/// 递归搜索文件系统（备用方案，当数据库搜索不可用时）
pub async fn search_filesystem(
    base_path: &Path,
    query: &str,
    file_type: Option<&FileType>,
    recursive: bool,
    depth: u32,
) -> Result<Vec<SearchResultItem>, Error> {
    if depth > MAX_RECURSIVE_DEPTH {
        return Ok(vec![]);
    }
    
    let mut results = Vec::new();
    
    match fs::read_dir(base_path).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                let metadata = fs::metadata(&path).await.ok();
                
                if let Some(meta) = metadata {
                    let name = entry.file_name().to_string_lossy().to_string();
                    
                    // 模糊匹配文件名
                    if !query.is_empty() && !name.to_lowercase().contains(&query.to_lowercase()) {
                        continue;
                    }
                    
                    // 文件类型筛选
                    if let Some(ref ft) = file_type {
                        let actual_type = if meta.is_dir() {
                            FileType::Folder
                        } else {
                            path.extension()
                                .and_then(|e| FileType::from_extension(e.to_string_lossy().as_ref()))
                                .unwrap_or(FileType::File)
                        };
                        
                        if actual_type != *ft {
                            continue;
                        }
                    }
                    
                    let modified_at = meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    
                    results.push(SearchResultItem {
                        id: path.to_string_lossy().to_string(),
                        name: name.clone(),
                        path: path.to_string_lossy().to_string(),
                        file_type: if meta.is_dir() {
                            "folder".to_string()
                        } else {
                            path.extension()
                                .map(|e| e.to_string_lossy().to_string())
                                .unwrap_or_else(|| "file".to_string())
                        },
                        size: meta.len(),
                        modified_at,
                        created_at: modified_at,
                    });
                    
                    // 递归搜索子目录
                    if recursive && meta.is_dir() {
                        let mut sub_results = search_filesystem(
                            &path,
                            query,
                            file_type,
                            recursive,
                            depth + 1,
                        )
                        .await?;
                        results.append(&mut sub_results);
                    }
                }
            }
        }
        Err(e) => {
            log::warn!("读取目录失败 {}: {}", base_path.display(), e);
        }
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_safe_path() {
        assert!(is_safe_path("documents"));
        assert!(is_safe_path("/users/123"));
        assert!(is_safe_path("/data/uploads"));
        assert!(!is_safe_path("../etc/passwd"));
        assert!(!is_safe_path("/etc/passwd"));
        assert!(!is_safe_path("foo/../../../bar"));
    }
    
    #[test]
    fn test_sanitize_path() {
        assert_eq!(sanitize_path("foo/../bar"), "bar");
        assert_eq!(sanitize_path("/users/123"), "/users/123");
        assert_eq!(sanitize_path("a/b/c"), "a/b/c");
    }
    
    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(FileType::from_extension("jpg"), Some(FileType::Image));
        assert_eq!(FileType::from_extension("png"), Some(FileType::Image));
        assert_eq!(FileType::from_extension("mp4"), Some(FileType::Video));
        assert_eq!(FileType::from_extension("pdf"), Some(FileType::Document));
        assert_eq!(FileType::from_extension("txt"), Some(FileType::Document));
        assert_eq!(FileType::from_extension("rs"), Some(FileType::File));
    }
}
