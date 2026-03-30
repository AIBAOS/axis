// Phase 65: 存储池更新 API
// PUT /api/v1/storage/pools/{id} — 更新存储池

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::rbac::RbacRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 更新存储池请求
#[derive(Debug, Deserialize)]
pub struct UpdatePoolRequest {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub pool_type: Option<String>,
    pub status: Option<String>,
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

/// 更新存储池响应
#[derive(Serialize)]
pub struct UpdatePoolResponse {
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

/// 存储池更新（Phase 65）
/// - JWT 认证，仅 admin 角色可访问
/// - 可更新字段：name, type, status
/// - 验证：存储池 ID 存在（404）、名称唯一性（409）、type 变更检查（400）
/// - 更新成功返回 200 OK + 更新后的存储池信息
pub async fn update_pool(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdatePoolRequest>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let pool_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 仅 admin 角色可访问
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update storage pools".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 验证请求参数
    if let Some(ref name) = payload.name {
        if name.is_empty() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "name cannot be empty".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
        if name.len() > 100 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "name must be less than 100 characters".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    if let Some(ref pool_type) = payload.pool_type {
        if !is_valid_pool_type(pool_type) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid type. Valid values: basic, raid0, raid1, raid5, raid6, raid10".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    if let Some(ref status) = payload.status {
        let valid_statuses = ["online", "degraded", "offline"];
        if !valid_statuses.contains(&status.as_str()) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid status. Valid values: online, degraded, offline".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    // 4. 模拟数据：检查存储池是否存在
    let existing_pools = vec![
        (1, "System Pool", "basic", "online", false),
        (2, "Data Pool", "raid1", "online", true),  // has_volumes = true
        (3, "Backup Pool", "raid5", "degraded", false),
    ];

    let pool = existing_pools.iter().find(|(id, _, _, _, _)| *id == pool_id);

    let (_, current_name, current_type, current_status, has_volumes) = match pool {
        Some(p) => p,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage pool {} not found", pool_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    };

    // 5. 验证名称唯一性（排除自身）
    let new_name = payload.name.as_deref().unwrap_or(current_name);
    let all_pool_names = vec!["System Pool", "Data Pool", "Backup Pool"];
    
    if new_name != *current_name && all_pool_names.contains(&new_name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage pool '{}' already exists", new_name),
            code: "CONFLICT".to_string(),
        }));
    }

    // 6. 验证 type 变更时是否有卷在使用
    let new_type = payload.pool_type.as_deref().unwrap_or(current_type);
    if new_type != *current_type && *has_volumes {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Cannot change pool type while volumes are using this pool".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 7. 更新存储池（模拟）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let updated_pool = StoragePoolInfo {
        id: pool_id,
        name: new_name.to_string(),
        pool_type: new_type.to_string(),
        status: payload.status.as_deref().unwrap_or(current_status).to_string(),
        total_bytes: 4000 * 1024 * 1024 * 1024,
        used_bytes: 1600 * 1024 * 1024 * 1024,
        available_bytes: 2400 * 1024 * 1024 * 1024,
        usage_percent: 40.0,
        disk_count: 2,
        disks: vec![
            DiskInfo {
                id: 2,
                name: "Disk 2".to_string(),
                capacity_bytes: 2000 * 1024 * 1024 * 1024,
                status: "online".to_string(),
            },
            DiskInfo {
                id: 3,
                name: "Disk 3".to_string(),
                capacity_bytes: 2000 * 1024 * 1024 * 1024,
                status: "online".to_string(),
            },
        ],
        created_at: 1710600000,
        updated_at: now,
    };

    Ok(HttpResponse::Ok().json(UpdatePoolResponse {
        success: true,
        message: "Storage pool updated successfully".to_string(),
        data: updated_pool,
    }))
}
