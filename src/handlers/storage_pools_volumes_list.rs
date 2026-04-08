// Phase 71 - 存储池卷列表 API
// GET /api/v1/storage/pools/{id}/volumes — 获取存储池下的所有卷

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

/// 存储池卷列表响应
#[derive(Serialize)]
pub struct StoragePoolVolumesResponse {
    pub success: bool,
    pub pool_id: u64,
    pub pool_name: String,
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

/// 获取存储池下的卷列表（Phase 71）
/// - JWT 认证，任意登录用户可访问
/// - 验证存储池存在性（404 Not Found）
/// - 支持分页：page, limit
/// - 返回该存储池下的所有卷
pub async fn list_pool_volumes(
    req: HttpRequest,
    path: web::Path<u64>,
    query: web::Query<StorageVolumesQuery>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
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

    let pool_id = path.into_inner();

    // 3. 解析查询参数
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let limit = query.limit.unwrap_or(20).max(1).min(100) // Bug #72 修复：防止空结果;

    // 4. 模拟存储池数据（验证存在性）
    let mock_pools = vec![
        serde_json::json!({"id": 1, "name": "System Pool"}),
        serde_json::json!({"id": 2, "name": "Data Pool"}),
        serde_json::json!({"id": 3, "name": "Backup Pool"}),
        serde_json::json!({"id": 4, "name": "Archive Pool"}),
    ];

    let pool = mock_pools.into_iter().find(|p| p["id"] == pool_id);

    match pool {
        Some(p) => {
            let pool_name = p["name"].as_str().unwrap_or("unknown").to_string();

            // 5. 模拟该存储池下的卷数据
            let all_volumes = vec![
                StorageVolumeInfo {
                    id: 1,
                    name: "System Volume".to_string(),
                    pool_id,
                    pool_name: pool_name.clone(),
                    size_bytes: 250 * 1024 * 1024 * 1024,
                    used_bytes: 125 * 1024 * 1024 * 1024,
                    available_bytes: 125 * 1024 * 1024 * 1024,
                    usage_percent: 50.0,
                    filesystem_type: "ext4".to_string(),
                    status: "online".to_string(),
                    mount_point: "/mnt/system_volume".to_string(),
                    created_at: 1710489600,
                    updated_at: 1711440000,
                },
                StorageVolumeInfo {
                    id: 5,
                    name: format!("{} Volume 2", pool_name),
                    pool_id,
                    pool_name: pool_name.clone(),
                    size_bytes: 500 * 1024 * 1024 * 1024,
                    used_bytes: 250 * 1024 * 1024 * 1024,
                    available_bytes: 250 * 1024 * 1024 * 1024,
                    usage_percent: 50.0,
                    filesystem_type: "ext4".to_string(),
                    status: "online".to_string(),
                    mount_point: format!("/mnt/{}_volume_2", pool_name.to_lowercase().replace(" ", "_")),
                    created_at: 1710489600,
                    updated_at: 1711440000,
                },
            ];

            let total = all_volumes.len() as u64;
            let total_pages = ((total + limit as u64 - 1) / limit as u64) as u32;
            let start = (page - 1) as usize * limit as usize;
            let end = start + limit as usize;

            // 6. 分页
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

            Ok(HttpResponse::Ok().json(StoragePoolVolumesResponse {
                success: true,
                pool_id,
                pool_name,
                data: volumes,
                pagination: PaginationMeta {
                    page,
                    limit,
                    total,
                    total_pages,
                },
            }))
        }
        None => {
            // 7. 存储池不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage pool {} not found", pool_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
