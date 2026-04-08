// Phase 90 - 共享文件夹列表 API
// GET /api/v1/shared-folders — 获取共享文件夹列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 共享文件夹信息
#[derive(Serialize, Clone)]
pub struct SharedFolderInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub is_public: bool,
    pub status: String,
    pub created_at: u64,
    pub created_by: String,
}

/// 分页查询参数
#[derive(Deserialize)]
pub struct SharedFolderListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub protocol: Option<String>,
    pub volume_id: Option<u64>,
    pub status: Option<String>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 共享文件夹列表响应
#[derive(Serialize)]
pub struct SharedFolderListResponse {
    pub success: bool,
    pub data: Vec<SharedFolderInfo>,
    pub pagination: PaginationMeta,
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

/// 共享文件夹列表（Phase 90）
/// - JWT 认证，登录用户可访问
/// - 支持分页：page/per_page
/// - 支持筛选：protocol/volume_id/status
/// - 返回共享文件夹列表和分页信息
pub async fn list_shared_folders(
    req: HttpRequest,
    query: web::Query<SharedFolderListQuery>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    // 2. 解析分页参数
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let per_page = query.per_page.unwrap_or(20).max(1).min(100);

    // 3. 解析筛选参数
    let protocol_filter = query.protocol.as_deref();
    let volume_id_filter = query.volume_id;
    let status_filter = query.status.as_deref();

    // 4. 模拟共享文件夹数据
    let all_folders = vec![
        SharedFolderInfo {
            id: 1,
            name: "public".to_string(),
            path: "/public".to_string(),
            volume_id: 1,
            volume_name: "data".to_string(),
            description: Some("Public shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string()],
            is_public: true,
            status: "active".to_string(),
            created_at: 1774259200,
            created_by: "admin".to_string(),
        },
        SharedFolderInfo {
            id: 2,
            name: "homes".to_string(),
            path: "/homes".to_string(),
            volume_id: 1,
            volume_name: "data".to_string(),
            description: Some("User home directories".to_string()),
            protocols: vec!["smb".to_string()],
            is_public: false,
            status: "active".to_string(),
            created_at: 1774345600,
            created_by: "admin".to_string(),
        },
        SharedFolderInfo {
            id: 3,
            name: "media".to_string(),
            path: "/media".to_string(),
            volume_id: 2,
            volume_name: "backup".to_string(),
            description: Some("Media files shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string(), "afp".to_string()],
            is_public: false,
            status: "active".to_string(),
            created_at: 1774432000,
            created_by: "admin".to_string(),
        },
        SharedFolderInfo {
            id: 4,
            name: "archive".to_string(),
            path: "/archive".to_string(),
            volume_id: 2,
            volume_name: "backup".to_string(),
            description: None,
            protocols: vec!["smb".to_string()],
            is_public: false,
            status: "inactive".to_string(),
            created_at: 1774518400,
            created_by: "admin".to_string(),
        },
    ];

    // 5. 应用筛选
    let mut filtered_folders = all_folders;

    // 协议筛选
    if let Some(protocol) = protocol_filter {
        filtered_folders.retain(|f| {
            f.protocols.iter().any(|p| p.to_lowercase() == protocol.to_lowercase())
        });
    }

    // 存储卷 ID 筛选
    if let Some(volume_id) = volume_id_filter {
        filtered_folders.retain(|f| f.volume_id == volume_id);
    }

    // 状态筛选
    if let Some(status) = status_filter {
        filtered_folders.retain(|f| f.status.to_lowercase() == status.to_lowercase());
    }

    let total = filtered_folders.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = (page - 1) as usize * per_page as usize;
    let end = start + per_page as usize;

    // 6. 分页处理
    let folders: Vec<SharedFolderInfo> = filtered_folders
        .into_iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if i >= start && i < end {
                Some(f)
            } else {
                None
            }
        })
        .collect();

    // 7. 返回响应（无数据返回空数组）
    Ok(HttpResponse::Ok().json(SharedFolderListResponse {
        success: true,
        data: folders,
        pagination: PaginationMeta {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
