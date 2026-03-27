// Phase 80: 存储卷创建 API
// POST /api/v1/storage/volumes — 创建存储卷

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::rbac::RbacRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 创建存储卷请求
#[derive(Debug, Deserialize)]
pub struct CreateVolumeRequest {
    pub name: String,
    pub description: Option<String>,
    pub pool_id: u64,
    pub size_bytes: u64,
    pub filesystem: Option<String>,
    pub mount_point: Option<String>,
}

/// 存储卷信息
#[derive(Serialize, Clone)]
pub struct VolumeInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub pool_id: u64,
    pub pool_name: String,
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

/// 创建存储卷响应
#[derive(Serialize)]
pub struct CreateVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: VolumeInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证文件系统类型
fn is_valid_filesystem(fs: &str) -> bool {
    let valid = ["ext4", "xfs", "btrfs", "zfs"];
    valid.contains(&fs)
}

/// 存储卷创建（Phase 80）
/// - JWT 认证，仅 admin 角色可访问
/// - 请求体：name/description/pool_id/size_bytes/filesystem/mount_point
/// - 验证：存储池存在性（404）、名称唯一性（409）、容量检查（400）
/// - 创建成功返回 201 Created
pub async fn create_volume(
    req: HttpRequest,
    payload: web::Json<CreateVolumeRequest>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
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
            error: "Only admin users can create storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

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

    if payload.size_bytes == 0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "size_bytes must be greater than 0".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 验证文件系统类型
    let filesystem = payload.filesystem.as_deref().unwrap_or("ext4");
    if !is_valid_filesystem(filesystem) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid filesystem. Valid values: ext4, xfs, btrfs, zfs".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟数据：验证存储池是否存在
    let pools = vec![
        (1, "System Pool", 500 * 1024 * 1024 * 1024, 250 * 1024 * 1024 * 1024),
        (2, "Data Pool", 4000 * 1024 * 1024 * 1024, 1600 * 1024 * 1024 * 1024),
        (3, "Backup Pool", 12000 * 1024 * 1024 * 1024, 3000 * 1024 * 1024 * 1024),
    ];

    let pool = pools.iter().find(|(id, _, _, _)| *id == payload.pool_id);

    let (_, pool_name, _, pool_available) = match pool {
        Some(p) => p,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage pool {} not found", payload.pool_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    };

    // 5. 验证容量是否超过池可用空间
    if payload.size_bytes > *pool_available {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Requested size exceeds pool available space ({} bytes)", pool_available),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 6. 验证名称唯一性
    let existing_volumes = vec!["System Volume", "Data Volume 1", "Data Volume 2", "Backup Volume"];
    if existing_volumes.contains(&payload.name.as_str()) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage volume '{}' already exists", payload.name),
            code: "CONFLICT".to_string(),
        }));
    }

    // 7. 创建存储卷（模拟）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let volume = VolumeInfo {
        id: 100, // 模拟 ID
        name: payload.name.clone(),
        description: payload.description.clone().unwrap_or_else(|| "".to_string()),
        pool_id: payload.pool_id,
        pool_name: pool_name.to_string(),
        total_bytes: payload.size_bytes,
        used_bytes: 0,
        available_bytes: payload.size_bytes,
        usage_percent: 0.0,
        status: "active".to_string(),
        filesystem: filesystem.to_string(),
        mount_point: payload.mount_point.clone().unwrap_or_else(|| format!("/mnt/{}", payload.name)),
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(CreateVolumeResponse {
        success: true,
        message: "Storage volume created successfully".to_string(),
        data: volume,
    }))
}
