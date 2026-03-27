// Phase 75: 存储池列表 API
// GET /api/v1/storage/pools — 获取存储池列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 磁盘信息（简化）
#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub id: u64,
    pub name: String,
    pub size_bytes: u64,
}

/// 存储池信息
#[derive(Serialize, Clone)]
pub struct StoragePoolInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub disk_count: u32,
    pub disks: Vec<DiskInfo>,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PoolListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 存储池列表响应
#[derive(Serialize)]
pub struct StoragePoolsResponse {
    pub success: bool,
    pub data: Vec<StoragePoolInfo>,
    pub pagination: PaginationInfo,
}

/// 存储池列表（Phase 75）
/// - JWT 认证，任意登录用户可访问
/// - 返回字段：id/name/description/total_bytes/used_bytes/available_bytes/usage_percent/disk_count/disks/status/created_at/updated_at
/// - 支持分页：page, per_page
/// - 无数据返回空数组
pub async fn list_pools(
    req: HttpRequest,
    query: web::Query<PoolListQuery>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 任意登录用户可访问（无需 admin）

    // 3. 解析查询参数
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

    // 4. 模拟存储池数据（后续连接系统 API）
    let all_pools = vec![
        StoragePoolInfo {
            id: 1,
            name: "System Pool".to_string(),
            description: "System storage pool".to_string(),
            total_bytes: 500 * 1024 * 1024 * 1024,
            used_bytes: 250 * 1024 * 1024 * 1024,
            available_bytes: 250 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            disk_count: 1,
            disks: vec![
                DiskInfo {
                    id: 1,
                    name: "Disk 1".to_string(),
                    size_bytes: 500 * 1024 * 1024 * 1024,
                },
            ],
            status: "online".to_string(),
            created_at: 1710500000,
            updated_at: 1711400000,
        },
        StoragePoolInfo {
            id: 2,
            name: "Data Pool".to_string(),
            description: "Data storage pool".to_string(),
            total_bytes: 4000 * 1024 * 1024 * 1024,
            used_bytes: 1600 * 1024 * 1024 * 1024,
            available_bytes: 2400 * 1024 * 1024 * 1024,
            usage_percent: 40.0,
            disk_count: 2,
            disks: vec![
                DiskInfo {
                    id: 2,
                    name: "Disk 2".to_string(),
                    size_bytes: 2000 * 1024 * 1024 * 1024,
                },
                DiskInfo {
                    id: 3,
                    name: "Disk 3".to_string(),
                    size_bytes: 2000 * 1024 * 1024 * 1024,
                },
            ],
            status: "online".to_string(),
            created_at: 1710600000,
            updated_at: 1711500000,
        },
        StoragePoolInfo {
            id: 3,
            name: "Backup Pool".to_string(),
            description: "Backup storage pool".to_string(),
            total_bytes: 12000 * 1024 * 1024 * 1024,
            used_bytes: 3000 * 1024 * 1024 * 1024,
            available_bytes: 9000 * 1024 * 1024 * 1024,
            usage_percent: 25.0,
            disk_count: 4,
            disks: vec![
                DiskInfo {
                    id: 4,
                    name: "Disk 4".to_string(),
                    size_bytes: 3000 * 1024 * 1024 * 1024,
                },
                DiskInfo {
                    id: 5,
                    name: "Disk 5".to_string(),
                    size_bytes: 3000 * 1024 * 1024 * 1024,
                },
                DiskInfo {
                    id: 6,
                    name: "Disk 6".to_string(),
                    size_bytes: 3000 * 1024 * 1024 * 1024,
                },
                DiskInfo {
                    id: 7,
                    name: "Disk 7".to_string(),
                    size_bytes: 3000 * 1024 * 1024 * 1024,
                },
            ],
            status: "degraded".to_string(),
            created_at: 1710700000,
            updated_at: 1711600000,
        },
    ];

    let total = all_pools.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = (page - 1) as usize * per_page as usize;
    let end = start + per_page as usize;

    // 5. 分页
    let pools: Vec<StoragePoolInfo> = if start < all_pools.len() {
        all_pools[start..end.min(all_pools.len())].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(StoragePoolsResponse {
        success: true,
        data: pools,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
