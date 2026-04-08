use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 下载状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadStatus::Pending => write!(f, "pending"),
            DownloadStatus::Downloading => write!(f, "downloading"),
            DownloadStatus::Completed => write!(f, "completed"),
            DownloadStatus::Failed => write!(f, "failed"),
            DownloadStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// 下载任务信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownloadTask {
    pub id: u64,
    pub url: String,
    pub filename: String,
    pub save_path: String,
    pub status: String,
    pub progress: u8,
    pub total_bytes: u64,
    pub downloaded_bytes: u64,
    pub speed_bps: u64,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub error_message: Option<String>,
}

/// 分页参数
#[derive(Debug, Deserialize)]
pub struct DownloadQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

impl Default for DownloadQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            status: None,
            sort: Some("created_at".to_string()),
            order: Some("desc".to_string()),
        }
    }
}

/// 下载列表响应
#[derive(Debug, Serialize)]
pub struct DownloadListResponse {
    pub success: bool,
    pub data: Vec<DownloadTask>,
    pub pagination: DownloadPagination,
}

#[derive(Debug, Serialize)]
pub struct DownloadPagination {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
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

/// 获取下载任务列表
/// 需要登录用户访问
pub async fn get_downloads(
    http_req: HttpRequest,
    query: web::Query<DownloadQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let limit = query.per_page.unwrap_or(20).max(1).min(100) as u64; // 最大 100
    let status_filter = query.status.as_deref();
    let _sort = query.sort.as_deref().unwrap_or("created_at");
    let _order = query.order.as_deref().unwrap_or("desc");

    // 模拟数据
    let mut all_tasks = vec![
        DownloadTask {
            id: 1,
            url: "https://example.com/file1.iso".to_string(),
            filename: "file1.iso".to_string(),
            save_path: "/downloads/file1.iso".to_string(),
            status: "downloading".to_string(),
            progress: 65,
            total_bytes: 100_000_000,
            downloaded_bytes: 65_000_000,
            speed_bps: 10_000_000,
            created_at: "2026-03-18T20:00:00Z".to_string(),
            started_at: Some("2026-03-18T20:05:00Z".to_string()),
            completed_at: None,
            error_message: None,
        },
        DownloadTask {
            id: 2,
            url: "https://example.com/file2.zip".to_string(),
            filename: "file2.zip".to_string(),
            save_path: "/downloads/file2.zip".to_string(),
            status: "completed".to_string(),
            progress: 100,
            total_bytes: 50_000_000,
            downloaded_bytes: 50_000_000,
            speed_bps: 0,
            created_at: "2026-03-18T18:00:00Z".to_string(),
            started_at: Some("2026-03-18T18:02:00Z".to_string()),
            completed_at: Some("2026-03-18T19:30:00Z".to_string()),
            error_message: None,
        },
        DownloadTask {
            id: 3,
            url: "https://example.com/file3.tar.gz".to_string(),
            filename: "file3.tar.gz".to_string(),
            save_path: "/downloads/file3.tar.gz".to_string(),
            status: "failed".to_string(),
            progress: 45,
            total_bytes: 75_000_000,
            downloaded_bytes: 33_750_000,
            speed_bps: 0,
            created_at: "2026-03-18T17:00:00Z".to_string(),
            started_at: Some("2026-03-18T17:05:00Z".to_string()),
            completed_at: None,
            error_message: Some("Connection timeout".to_string()),
        },
        DownloadTask {
            id: 4,
            url: "https://example.com/file4.dmg".to_string(),
            filename: "file4.dmg".to_string(),
            save_path: "/downloads/file4.dmg".to_string(),
            status: "pending".to_string(),
            progress: 0,
            total_bytes: 200_000_000,
            downloaded_bytes: 0,
            speed_bps: 0,
            created_at: "2026-03-18T16:00:00Z".to_string(),
            started_at: None,
            completed_at: None,
            error_message: None,
        },
        DownloadTask {
            id: 5,
            url: "https://example.com/file5.pkg".to_string(),
            filename: "file5.pkg".to_string(),
            save_path: "/downloads/file5.pkg".to_string(),
            status: "cancelled".to_string(),
            progress: 30,
            total_bytes: 150_000_000,
            downloaded_bytes: 45_000_000,
            speed_bps: 0,
            created_at: "2026-03-18T15:00:00Z".to_string(),
            started_at: Some("2026-03-18T15:10:00Z".to_string()),
            completed_at: None,
            error_message: Some("User cancelled".to_string()),
        },
    ];

    // 默认按 created_at 倒序排序
    let mut sort = None;
    let mut order = None;
    if let Some(ref s) = query.sort {
        sort = Some(s.as_str());
    }
    if let Some(ref o) = query.order {
        order = Some(o.as_str());
    }
    let sort = sort.unwrap_or("created_at");
    let order = order.unwrap_or("desc");

    // 状态过滤（支持 all）
    if status_filter != Some("all") && status_filter.is_some() {
        all_tasks.retain(|t| t.status == status_filter.unwrap_or("all"));
    }

    // 排序
    match sort {
        "created_at" => {
            if order == "asc" {
                all_tasks.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            } else {
                all_tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
        }
        "progress" => {
            if order == "asc" {
                all_tasks.sort_by(|a, b| a.progress.cmp(&b.progress));
            } else {
                all_tasks.sort_by(|a, b| b.progress.cmp(&a.progress));
            }
        }
        "speed_bps" => {
            if order == "asc" {
                all_tasks.sort_by(|a, b| a.speed_bps.cmp(&b.speed_bps));
            } else {
                all_tasks.sort_by(|a, b| b.speed_bps.cmp(&a.speed_bps));
            }
        }
        _ => {}
    }

    let total = all_tasks.len() as u64;
    let start = (page - 1) * limit;
    let end = start + limit;

    let paginated_tasks: Vec<DownloadTask> = all_tasks
        .into_iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let idx = i as u64;
            if idx >= start && idx < end {
                Some(t)
            } else {
                None
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(DownloadListResponse {
        success: true,
        data: paginated_tasks,
        pagination: DownloadPagination {
            page,
            limit,
            total,
            total_pages: (total + limit - 1) / limit,
        },
    }))
}

/// 获取单个下载任务
/// 需要登录用户访问
pub async fn get_download(
    http_req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let id = path.into_inner();

    let mock_downloads = vec![
        DownloadTask {
            id: 1,
            url: "https://example.com/file1.iso".to_string(),
            filename: "file1.iso".to_string(),
            save_path: "/downloads/file1.iso".to_string(),
            status: "downloading".to_string(),
            progress: 65,
            total_bytes: 100_000_000,
            downloaded_bytes: 65_000_000,
            speed_bps: 10_000_000,
            created_at: "2026-03-18T20:00:00Z".to_string(),
            started_at: Some("2026-03-18T20:05:00Z".to_string()),
            completed_at: None,
            error_message: None,
        },
        DownloadTask {
            id: 2,
            url: "https://example.com/file2.zip".to_string(),
            filename: "file2.zip".to_string(),
            save_path: "/downloads/file2.zip".to_string(),
            status: "completed".to_string(),
            progress: 100,
            total_bytes: 50_000_000,
            downloaded_bytes: 50_000_000,
            speed_bps: 12_000_000,
            created_at: "2026-03-18T18:00:00Z".to_string(),
            started_at: Some("2026-03-18T18:02:00Z".to_string()),
            completed_at: Some("2026-03-18T18:10:00Z".to_string()),
            error_message: None,
        },
    ];

    match mock_downloads.iter().find(|d| d.id == id) {
        Some(download) => Ok(HttpResponse::Ok().json(download)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Download task {} not found", id)
        }))),
    }
}

/// 创建下载任务
/// 需要登录用户访问
pub async fn create_download(
    http_req: HttpRequest,
    payload: web::Json<CreateDownloadRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let url = &payload.url;
    
    // URL 格式校验
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Invalid URL format. URL must start with http:// or https://"
        })));
    }
    
    // 重复任务检测：检查模拟数据中是否存在相同 URL 的下载任务
    let mock_downloads = vec![
        DownloadTask {
            id: 1,
            url: "https://example.com/file1.iso".to_string(),
            filename: "file1.iso".to_string(),
            save_path: "/downloads/file1.iso".to_string(),
            status: "downloading".to_string(),
            progress: 65,
            total_bytes: 100_000_000,
            downloaded_bytes: 65_000_000,
            speed_bps: 10_000_000,
            created_at: "2026-03-18T20:00:00Z".to_string(),
            started_at: Some("2026-03-18T20:05:00Z".to_string()),
            completed_at: None,
            error_message: None,
        },
        DownloadTask {
            id: 2,
            url: "https://example.com/file2.zip".to_string(),
            filename: "file2.zip".to_string(),
            save_path: "/downloads/file2.zip".to_string(),
            status: "completed".to_string(),
            progress: 100,
            total_bytes: 50_000_000,
            downloaded_bytes: 50_000_000,
            speed_bps: 12_000_000,
            created_at: "2026-03-18T18:00:00Z".to_string(),
            started_at: Some("2026-03-18T18:02:00Z".to_string()),
            completed_at: Some("2026-03-18T18:10:00Z".to_string()),
            error_message: None,
        },
    ];
    
    if mock_downloads.iter().any(|d| d.url == *url) {
        return Ok(HttpResponse::Conflict().json(serde_json::json!({
            "success": false,
            "message": format!("Download task for URL '{}' already exists", url)
        })));
    }
    
    let save_path = payload.save_path.clone().unwrap_or_else(|| "/downloads".to_string());
    let filename = payload.filename.clone().unwrap_or_else(|| url.split('/').last().unwrap_or("downloaded_file").to_string());

    Ok(HttpResponse::Created().json(DownloadTask {
        id: 3,
        url: url.clone(),
        filename,
        save_path,
        status: "pending".to_string(),
        progress: 0,
        total_bytes: 0,
        downloaded_bytes: 0,
        speed_bps: 0,
        created_at: chrono::Utc::now().to_rfc3339(),
        started_at: None,
        completed_at: None,
        error_message: None,
    }))
}

/// 取消下载任务
/// 需要登录用户访问
pub async fn cancel_download(
    http_req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let id = path.into_inner();
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Download {} cancelled", id),
        "id": id
    })))
}

/// 获取下载统计
/// 需要登录用户访问
pub async fn get_download_stats(
    http_req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "total": 5,
        "pending": 1,
        "downloading": 1,
        "completed": 1,
        "failed": 1,
        "cancelled": 1,
        "total_size_bytes": 575_000_000,
        "total_downloaded_bytes": 143_750_000
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateDownloadRequest {
    pub url: String,
    pub filename: Option<String>,
    pub save_path: Option<String>,
}
