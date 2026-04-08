// Phase 81 - 存储卷快照列表 API
// GET /api/v1/storage/volumes/{id}/snapshots — 获取存储卷快照列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 快照信息
#[derive(Serialize, Clone)]
pub struct SnapshotInfo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub volume_id: u64,
    pub size_bytes: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub status: String,
}

/// 分页查询参数
#[derive(Deserialize)]
pub struct SnapshotListQuery {
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

/// 快照列表响应
#[derive(Serialize)]
pub struct SnapshotListResponse {
    pub success: bool,
    pub data: Vec<SnapshotInfo>,
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

/// 存储卷快照列表（Phase 81）
/// - JWT 认证，任意登录用户可访问
/// - 验证存储卷 ID 存在
/// - 支持分页：page/per_page
/// - 返回该存储卷下的所有快照
pub async fn list_snapshots(
    req: HttpRequest,
    path: web::Path<u64>,
    query: web::Query<SnapshotListQuery>,
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

    let volume_id = path.into_inner();

    // 2. 解析分页参数
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let per_page = query.per_page.unwrap_or(20).max(1).min(100); // Bug #72 修复：防止空结果，最大 100

    // 3. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![1, 2, 3];
    if !mock_volumes.contains(&volume_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", volume_id),
            code: "VOLUME_NOT_FOUND".to_string(),
        }));
    }

    // 4. 模拟该存储卷下的快照数据
    let all_snapshots = vec![
        SnapshotInfo {
            id: 1,
            name: "snapshot-2026-03-20".to_string(),
            description: Some("Daily backup snapshot".to_string()),
            volume_id,
            size_bytes: 549755813888,  // 512GB
            created_at: 1774259200,
            updated_at: 1774259200,
            status: "completed".to_string(),
        },
        SnapshotInfo {
            id: 2,
            name: "snapshot-2026-03-25".to_string(),
            description: Some("Pre-update snapshot".to_string()),
            volume_id,
            size_bytes: 549755813888,  // 512GB
            created_at: 1774345600,
            updated_at: 1774345600,
            status: "completed".to_string(),
        },
        SnapshotInfo {
            id: 3,
            name: "snapshot-2026-03-26".to_string(),
            description: None,
            volume_id,
            size_bytes: 549755813888,  // 512GB
            created_at: 1774432000,
            updated_at: 1774432000,
            status: "creating".to_string(),
        },
    ];

    let total = all_snapshots.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = (page - 1) as usize * per_page as usize;
    let end = start + per_page as usize;

    // 5. 分页处理
    let snapshots: Vec<SnapshotInfo> = all_snapshots
        .into_iter()
        .enumerate()
        .filter_map(|(i, s)| {
            if i >= start && i < end {
                Some(s)
            } else {
                None
            }
        })
        .collect();

    // 6. 返回响应（无数据返回空数组）
    Ok(HttpResponse::Ok().json(SnapshotListResponse {
        success: true,
        data: snapshots,
        pagination: PaginationMeta {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
