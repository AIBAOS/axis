// Phase 64: 存储池创建 API
// POST /api/v1/storage/pools — 创建存储池

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 创建存储池请求
#[derive(Debug, Deserialize)]
pub struct CreatePoolRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub disk_ids: Vec<u64>,
}

/// 磁盘信息
#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub id: u64,
    pub name: String,
    pub capacity_bytes: u64,
    pub status: String,
}

/// 存储池信息
#[derive(Serialize, Clone)]
pub struct StoragePoolInfo {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub status: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub disk_count: u32,
    pub disks: Vec<DiskInfo>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 创建存储池响应
#[derive(Serialize)]
pub struct CreatePoolResponse {
    pub success: bool,
    pub message: String,
    pub data: StoragePoolInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证 RAID 类型
fn is_valid_pool_type(pool_type: &str) -> bool {
    let valid_types = ["basic", "raid0", "raid1", "raid5", "raid6", "raid10"];
    valid_types.contains(&pool_type)
}

/// 验证 RAID 类型所需的最小磁盘数
fn get_min_disk_count(pool_type: &str) -> u32 {
    match pool_type {
        "basic" => 1,
        "raid0" | "raid1" => 2,
        "raid5" => 3,
        "raid6" => 4,
        "raid10" => 4,
        _ => 1,
    }
}

/// 创建存储池（Phase 64）
/// - JWT 认证，任意登录用户可访问
/// - 请求体：name, type, disk_ids
/// - 响应：201 Created + 存储池完整信息
/// - 错误处理：400 (参数缺失/无效), 409 (名称已存在), 404 (磁盘不存在)
pub async fn create_pool(
    req: HttpRequest,
    payload: web::Json<CreatePoolRequest>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
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

    // 3. 验证请求参数
    if payload.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if payload.name.len() > 100 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name must be less than 100 characters".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if !is_valid_pool_type(&payload.pool_type) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Invalid type. Valid values: basic, raid0, raid1, raid5, raid6, raid10"),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if payload.disk_ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "disk_ids is required and must not be empty".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 验证磁盘数量是否满足 RAID 类型要求
    let min_disks = get_min_disk_count(&payload.pool_type);
    if (payload.disk_ids.len() as u32) < min_disks {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("{} requires at least {} disk(s)", payload.pool_type, min_disks),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟数据：检查名称唯一性
    let existing_pools = vec!["System Pool", "Data Pool", "Backup Pool"];
    if existing_pools.contains(&payload.name.as_str()) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage pool '{}' already exists", payload.name),
            code: "CONFLICT".to_string(),
        }));
    }

    // 5. 模拟数据：验证磁盘 ID 是否存在
    let valid_disk_ids = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for disk_id in &payload.disk_ids {
        if !valid_disk_ids.contains(disk_id) {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Disk {} not found", disk_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    }

    // 6. 创建存储池（模拟）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // 模拟磁盘信息
    let disks: Vec<DiskInfo> = payload.disk_ids.iter().map(|&id| {
        DiskInfo {
            id,
            name: format!("Disk {}", id),
            capacity_bytes: 2000 * 1024 * 1024 * 1024, // 2TB per disk
            status: "online".to_string(),
        }
    }).collect();

    // 计算总容量（简化计算，实际应考虑 RAID 开销）
    let total_bytes = disks.iter().map(|d| d.capacity_bytes).sum();
    let used_bytes = 0;
    let available_bytes = total_bytes;
    let usage_percent = 0.0;

    let pool = StoragePoolInfo {
        id: 100, // 模拟 ID
        name: payload.name.clone(),
        pool_type: payload.pool_type.clone(),
        status: "online".to_string(),
        total_bytes,
        used_bytes,
        available_bytes,
        usage_percent,
        disk_count: payload.disk_ids.len() as u32,
        disks,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(CreatePoolResponse {
        success: true,
        message: "Storage pool created successfully".to_string(),
        data: pool,
    }))
}
