// Phase 64 - 存储池创建 API
// POST /api/v1/storage/pools — 创建新存储池

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 创建存储池请求
#[derive(Deserialize)]
pub struct CreateStoragePoolRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub disk_ids: Vec<u64>,
}

/// 存储池响应
#[derive(Serialize)]
pub struct StoragePoolResponse {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub status: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub disk_count: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 创建存储池响应
#[derive(Serialize)]
pub struct CreateStoragePoolResponse {
    pub success: bool,
    pub message: String,
    pub data: StoragePoolResponse,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 创建存储池（Phase 64）
/// - JWT 认证，任意登录用户可访问
/// - 创建新存储池
/// - 返回创建的存储池信息
pub async fn create_storage_pool(
    req: HttpRequest,
    payload: web::Json<CreateStoragePoolRequest>,
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

    // 3. 验证必要参数
    let name = &payload.name;
    let pool_type = &payload.pool_type;
    let disk_ids = &payload.disk_ids;

    if name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 验证存储池类型
    let valid_types = ["basic", "raid0", "raid1", "raid5", "raid6", "raid10"];
    if !valid_types.contains(&pool_type.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Invalid type. Valid types: {}", valid_types.join(", ")),
            code: "INVALID_TYPE".to_string(),
        }));
    }

    // 5. 验证磁盘 ID 列表
    if disk_ids.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "disk_ids must contain at least one disk".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 验证最小磁盘数量要求
    let min_disks = match pool_type.as_str() {
        "basic" => 1,
        "raid0" | "raid1" => 2,
        "raid5" => 3,
        "raid6" => 4,
        "raid10" => 4,
        _ => 1,
    };

    if disk_ids.len() < min_disks {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("{} requires at least {} disk(s)", pool_type, min_disks),
            code: "INSUFFICIENT_DISKS".to_string(),
        }));
    }

    // 6. 模拟磁盘存在性验证（后续可连接数据库）
    let mock_disk_ids = vec![1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for disk_id in disk_ids {
        if !mock_disk_ids.contains(disk_id) {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Disk {} not found", disk_id),
                code: "DISK_NOT_FOUND".to_string(),
            }));
        }
    }

    // 7. 模拟名称唯一性检查
    let existing_names = vec!["System Pool", "Data Pool", "Backup Pool", "Archive Pool"];
    if existing_names.contains(&name.as_str()) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage pool '{}' already exists", name),
            code: "NAME_CONFLICT".to_string(),
        }));
    }

    // 8. 模拟创建存储池（后续可连接数据库）
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
        .as_secs();

    // 模拟生成的存储池 ID
    let new_id = 100u64;

    // 模拟计算总容量（假设每个磁盘 2TB）
    let disk_capacity = 2000 * 1024 * 1024 * 1024u64; // 2TB
    let total_bytes = disk_capacity * disk_ids.len() as u64;

    // RAID1/RAID10 可用容量减半
    let usable_bytes = if pool_type == "raid1" || pool_type == "raid10" {
        total_bytes / 2
    } else if pool_type == "raid5" {
        // RAID5 损失一个磁盘容量
        disk_capacity * (disk_ids.len() as u64 - 1)
    } else if pool_type == "raid6" {
        // RAID6 损失两个磁盘容量
        disk_capacity * (disk_ids.len() as u64 - 2)
    } else {
        total_bytes
    };

    let pool = StoragePoolResponse {
        id: new_id,
        name: name.clone(),
        pool_type: pool_type.clone(),
        status: "online".to_string(),
        total_bytes: usable_bytes,
        used_bytes: 0,
        available_bytes: usable_bytes,
        usage_percent: 0.0,
        disk_count: disk_ids.len() as u32,
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(CreateStoragePoolResponse {
        success: true,
        message: "Storage pool created successfully".to_string(),
        data: pool,
    }))
}
