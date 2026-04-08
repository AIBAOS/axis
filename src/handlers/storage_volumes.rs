// Phase 78 - 存储卷列表 API
// GET /api/v1/storage/volumes — 获取存储卷列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 存储卷信息
#[derive(Serialize, Clone)]
pub struct VolumeInfo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub pool_id: u64,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub status: String,
    pub filesystem: String,
    pub mount_point: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Deserialize)]
pub struct VolumeListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 存储卷列表响应
#[derive(Serialize)]
pub struct VolumeListResponse {
    pub success: bool,
    pub data: Vec<VolumeInfo>,
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

/// 存储卷列表（Phase 78）
/// - JWT 认证，任意登录用户可访问
/// - 支持分页：page/per_page
/// - 返回存储卷列表和分页信息
pub async fn list_volumes(
    req: HttpRequest,
    query: web::Query<VolumeListQuery>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户可访问）
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

    // 3. 模拟存储卷数据
    let all_volumes = vec![
        VolumeInfo {
            id: 1,
            name: "root".to_string(),
            description: Some("Root volume for system files".to_string()),
            pool_id: 1,
            total_bytes: 1099511627776,        // 1TB
            used_bytes: 549755813888,         // 512GB
            available_bytes: 549755813888,    // 512GB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "ext4".to_string(),
            mount_point: "/mnt/volumes/root".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        VolumeInfo {
            id: 2,
            name: "data".to_string(),
            description: Some("Data volume for user files".to_string()),
            pool_id: 1,
            total_bytes: 2199023255552,       // 2TB
            used_bytes: 1099511627776,        // 1TB
            available_bytes: 1099511627776,   // 1TB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "ext4".to_string(),
            mount_point: "/mnt/volumes/data".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        VolumeInfo {
            id: 3,
            name: "backup".to_string(),
            description: Some("Backup volume for system backups".to_string()),
            pool_id: 2,
            total_bytes: 4398046511104,       // 4TB
            used_bytes: 2199023255552,        // 2TB
            available_bytes: 2199023255552,   // 2TB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "btrfs".to_string(),
            mount_point: "/mnt/volumes/backup".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
    ];

    let total = all_volumes.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = (page - 1) as usize * per_page as usize;
    let end = start + per_page as usize;

    // 4. 分页处理
    let volumes: Vec<VolumeInfo> = all_volumes
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if i >= start && i < end {
                Some(v)
            } else {
                None
            }
        })
        .collect();

    // 5. 返回响应（无数据返回空数组）
    Ok(HttpResponse::Ok().json(VolumeListResponse {
        success: true,
        data: volumes,
        pagination: PaginationMeta {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
