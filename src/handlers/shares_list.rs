// Phase 90 - 共享文件夹列表 API
// GET /api/v1/shares — 获取共享文件夹列表

use actix_web::{web, HttpResponse, Error};
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
    pub read_only: bool,
    pub guest_access: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Deserialize)]
pub struct SharesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub protocol: Option<String>,
    pub volume_id: Option<u64>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

/// 共享文件夹列表响应
#[derive(Serialize)]
pub struct SharesListResponse {
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
    true // 任意登录用户可访问
}

/// 获取共享文件夹列表（Phase 90）
/// - JWT 认证，登录用户可访问
/// - 支持分页：page, per_page
/// - 支持筛选：protocol, volume_id
/// - 无数据返回空数组
pub async fn list_shares(
    req: actix_web::HttpRequest,
    query: web::Query<SharesQuery>,
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
    let per_page = query.per_page.unwrap_or(20).min(100); // 最大 100

    // 3. 解析筛选参数
    let protocol_filter = query.protocol.as_deref();
    let volume_id_filter = query.volume_id;

    // 4. 模拟共享文件夹数据
    let all_shares = vec![
        SharedFolderInfo {
            id: 1,
            name: "public".to_string(),
            path: "/mnt/volumes/data/public".to_string(),
            volume_id: 2,
            volume_name: "data".to_string(),
            description: Some("Public shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string()],
            read_only: false,
            guest_access: true,
            created_at: 1710000000,
            updated_at: 1710000000,
        },
        SharedFolderInfo {
            id: 2,
            name: "media".to_string(),
            path: "/mnt/volumes/data/media".to_string(),
            volume_id: 2,
            volume_name: "data".to_string(),
            description: Some("Media files shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string(), "afp".to_string()],
            read_only: false,
            guest_access: false,
            created_at: 1710000000,
            updated_at: 1710000000,
        },
        SharedFolderInfo {
            id: 3,
            name: "backup".to_string(),
            path: "/mnt/volumes/backup/shared".to_string(),
            volume_id: 3,
            volume_name: "backup".to_string(),
            description: Some("Backup shared folder".to_string()),
            protocols: vec!["smb".to_string()],
            read_only: true,
            guest_access: false,
            created_at: 1710000000,
            updated_at: 1710000000,
        },
    ];

    // 5. 应用筛选
    let filtered_shares: Vec<SharedFolderInfo> = all_shares
        .into_iter()
        .filter(|share| {
            // 协议筛选
            if let Some(protocol) = protocol_filter {
                if !share.protocols.iter().any(|p| p == protocol) {
                    return false;
                }
            }
            // 存储卷 ID 筛选
            if let Some(volume_id) = volume_id_filter {
                if share.volume_id != volume_id {
                    return false;
                }
            }
            true
        })
        .collect();

    // 6. 分页
    let total = filtered_shares.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = ((page - 1) * per_page) as usize;
    let end = (page * per_page) as usize;
    let paginated_shares: Vec<SharedFolderInfo> = filtered_shares
        .into_iter()
        .skip(start)
        .take(end - start)
        .collect();

    // 7. 返回响应
    Ok(HttpResponse::Ok().json(SharesListResponse {
        success: true,
        data: paginated_shares,
        pagination: PaginationMeta {
            total,
            page,
            per_page,
            total_pages,
        },
    }))
}
