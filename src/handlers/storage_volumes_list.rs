// Phase 70 - 存储卷列表 API
// GET /api/v1/storage/volumes — 获取存储卷列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 存储卷信息
#[derive(Serialize)]
pub struct StorageVolumeInfo {
    pub id: u64,
    pub name: String,
    pub pool_id: u64,
    pub pool_name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub filesystem_type: String,
    pub status: String,
    pub mount_point: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Deserialize)]
pub struct StorageVolumesQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 存储卷列表响应
#[derive(Serialize)]
pub struct StorageVolumesResponse {
    pub success: bool,
    pub data: Vec<StorageVolumeInfo>,
    pub pagination: PaginationMeta,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储卷列表（Phase 70）
/// - JWT 认证，任意登录用户可访问
/// - 支持分页：page, limit
/// - 返回存储卷列表及分页信息
pub async fn list_storage_volumes(
    req: HttpRequest,
    query: web::Query<StorageVolumesQuery>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户）
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
    // 已通过 JWT 验证，说明是登录用户

    // 3. 解析查询参数
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);

    // 4. 模拟存储卷数据（后续可连接数据库）
    let all_volumes = vec![
        StorageVolumeInfo {
            id: 1,
            name: "System Volume".to_string(),
            pool_id: 1,
            pool_name: "System Pool".to_string(),
            size_bytes: 250 * 1024 * 1024 * 1024, // 250GB
            used_bytes: 125 * 1024 * 1024 * 1024,  // 125GB
            available_bytes: 125 * 1024 * 1024 * 1024, // 125GB
            usage_percent: 50.0,
            filesystem_type: "ext4".to_string(),
            status: "online".to_string(),
            mount_point: "/mnt/system_volume".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        StorageVolumeInfo {
            id: 2,
            name: "Data Volume".to_string(),
            pool_id: 2,
            pool_name: "Data Pool".to_string(),
            size_bytes: 1000 * 1024 * 1024 * 1024, // 1TB
            used_bytes: 500 * 1024 * 1024 * 1024,  // 500GB
            available_bytes: 500 * 1024 * 1024 * 1024, // 500GB
            usage_percent: 50.0,
            filesystem_type: "ext4".to_string(),
            status: "online".to_string(),
            mount_point: "/mnt/data_volume".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        StorageVolumeInfo {
            id: 3,
            name: "Backup Volume".to_string(),
            pool_id: 3,
            pool_name: "Backup Pool".to_string(),
            size_bytes: 2000 * 1024 * 1024 * 1024, // 2TB
            used_bytes: 800 * 1024 * 1024 * 1024,  // 800GB
            available_bytes: 1200 * 1024 * 1024 * 1024, // 1.2TB
            usage_percent: 40.0,
            filesystem_type: "btrfs".to_string(),
            status: "online".to_string(),
            mount_point: "/mnt/backup_volume".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        StorageVolumeInfo {
            id: 4,
            name: "Archive Volume".to_string(),
            pool_id: 4,
            pool_name: "Archive Pool".to_string(),
            size_bytes: 5000 * 1024 * 1024 * 1024, // 5TB
            used_bytes: 2500 * 1024 * 1024 * 1024, // 2.5TB
            available_bytes: 2500 * 1024 * 1024 * 1024, // 2.5TB
            usage_percent: 50.0,
            filesystem_type: "zfs".to_string(),
            status: "online".to_string(),
            mount_point: "/mnt/archive_volume".to_string(),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
    ];

    let total = all_volumes.len() as u64;
    let total_pages = ((total + limit as u64 - 1) / limit as u64) as u32;
    let start = (page - 1) as usize * limit as usize;
    let end = start + limit as usize;

    // 5. 分页
    let volumes: Vec<StorageVolumeInfo> = all_volumes
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

    Ok(HttpResponse::Ok().json(StorageVolumesResponse {
        success: true,
        data: volumes,
        pagination: PaginationMeta {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}
