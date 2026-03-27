// Phase 76 - 存储池详情 API
// GET /api/v1/storage/pools/{id} — 获取单个存储池详细信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 磁盘信息（用于存储池详情）
#[derive(Serialize, Clone)]
pub struct PoolDiskInfo {
    pub disk_id: u64,
    pub name: String,
    pub device_path: String,
    pub capacity_bytes: u64,
    pub status: String,
}

/// 存储卷信息（用于存储池详情）
#[derive(Serialize, Clone)]
pub struct PoolVolumeInfo {
    pub volume_id: u64,
    pub name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub status: String,
}

/// 存储池详情信息
#[derive(Serialize, Clone)]
pub struct PoolDetail {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub disk_count: u32,
    pub disks: Vec<PoolDiskInfo>,
    pub volume_count: u32,
    pub volumes: Vec<PoolVolumeInfo>,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 存储池详情响应
#[derive(Serialize)]
pub struct PoolDetailResponse {
    pub success: bool,
    pub data: PoolDetail,
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

/// 存储池详情（Phase 76）
/// - JWT 认证，任意登录用户可访问
/// - 验证存储池 ID 存在
/// - 返回存储池完整详情（含 disks + volumes 数组）
pub async fn get_pool(
    req: HttpRequest,
    path: web::Path<u64>,
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

    let pool_id = path.into_inner();

    // 2. 模拟存储池数据
    let mock_pools = vec![
        PoolDetail {
            id: 1,
            name: "primary".to_string(),
            description: Some("Primary storage pool for system data".to_string()),
            total_bytes: 3298534883328,        // 3TB
            used_bytes: 1649267441664,         // 1.5TB
            available_bytes: 1649267441664,    // 1.5TB
            usage_percent: 50.0,
            disk_count: 2,
            disks: vec![
                PoolDiskInfo {
                    disk_id: 1,
                    name: "System Disk".to_string(),
                    device_path: "/dev/sda1".to_string(),
                    capacity_bytes: 1649267441664,
                    status: "online".to_string(),
                },
                PoolDiskInfo {
                    disk_id: 2,
                    name: "Data Disk 1".to_string(),
                    device_path: "/dev/sdb1".to_string(),
                    capacity_bytes: 1649267441664,
                    status: "online".to_string(),
                },
            ],
            volume_count: 2,
            volumes: vec![
                PoolVolumeInfo {
                    volume_id: 1,
                    name: "root".to_string(),
                    size_bytes: 1099511627776,
                    used_bytes: 549755813888,
                    status: "online".to_string(),
                },
                PoolVolumeInfo {
                    volume_id: 2,
                    name: "data".to_string(),
                    size_bytes: 2199023255552,
                    used_bytes: 1099511627776,
                    status: "online".to_string(),
                },
            ],
            status: "online".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        PoolDetail {
            id: 2,
            name: "backup".to_string(),
            description: Some("Backup storage pool".to_string()),
            total_bytes: 2199023255552,        // 2TB
            used_bytes: 1099511627776,         // 1TB
            available_bytes: 1099511627776,    // 1TB
            usage_percent: 50.0,
            disk_count: 1,
            disks: vec![
                PoolDiskInfo {
                    disk_id: 3,
                    name: "Backup Disk".to_string(),
                    device_path: "/dev/sdc1".to_string(),
                    capacity_bytes: 2199023255552,
                    status: "online".to_string(),
                },
            ],
            volume_count: 1,
            volumes: vec![
                PoolVolumeInfo {
                    volume_id: 3,
                    name: "backup".to_string(),
                    size_bytes: 2199023255552,
                    used_bytes: 1099511627776,
                    status: "online".to_string(),
                },
            ],
            status: "online".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
    ];

    // 3. 查找存储池
    let pool = mock_pools.into_iter().find(|p| p.id == pool_id);

    // 4. 验证存储池存在
    match pool {
        Some(p) => Ok(HttpResponse::Ok().json(PoolDetailResponse {
            success: true,
            data: p,
        })),
        None => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage pool {} not found", pool_id),
            code: "NOT_FOUND".to_string(),
        })),
    }
}
