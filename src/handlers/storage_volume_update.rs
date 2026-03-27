// Phase 68 - 更新存储卷 API
// PUT /api/v1/storage/volumes/{id} — 更新存储卷

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 更新存储卷请求
#[derive(Debug, Deserialize)]
pub struct UpdateVolumeRequest {
    pub name: Option<String>,
    pub size_bytes: Option<u64>,
    pub filesystem_type: Option<String>,
}

/// 更新存储卷响应
#[derive(Debug, Serialize)]
pub struct UpdateVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<VolumeData>,
}

/// 存储卷数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeData {
    pub id: u64,
    pub name: String,
    pub pool_id: u64,
    pub pool_name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub mount_point: String,
    pub filesystem: String,
    pub status: String,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 验证文件系统类型
fn validate_filesystem_type(fs_type: &str) -> bool {
    let valid_types = ["ext4", "btrfs", "zfs"];
    valid_types.contains(&fs_type.to_lowercase().as_str())
}

/// 更新存储卷（Phase 68）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在
/// - 部分更新（仅传递需要更新的字段）
/// - 验证名称唯一性（排除自身）
/// - 验证新容量不小于已用容量
/// - 验证文件系统类型有效性
pub async fn update_volume(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
    req: web::Json<UpdateVolumeRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let volume_id = path.into_inner();

    // 2. 验证请求参数（至少一个字段）
    if req.name.is_none() && req.size_bytes.is_none() && req.filesystem_type.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one field (name/size_bytes/filesystem_type) must be provided".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 验证文件系统类型（如果提供）
    if let Some(ref fs_type) = req.filesystem_type {
        if !validate_filesystem_type(fs_type) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid filesystem_type '{}'. Valid types: ext4, btrfs, zfs", fs_type),
                code: "INVALID_FILESYSTEM_TYPE".to_string(),
            }));
        }
    }

    // 4. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        VolumeData {
            id: 1,
            name: "root".to_string(),
            pool_id: 1,
            pool_name: "primary".to_string(),
            size_bytes: 1099511627776,
            used_bytes: 549755813888,
            available_bytes: 549755813888,
            usage_percent: 50.0,
            mount_point: "/mnt/volumes/root".to_string(),
            filesystem: "ext4".to_string(),
            status: "online".to_string(),
            description: None,
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        VolumeData {
            id: 2,
            name: "data".to_string(),
            pool_id: 1,
            pool_name: "primary".to_string(),
            size_bytes: 2199023255552,
            used_bytes: 1099511627776,
            available_bytes: 1099511627776,
            usage_percent: 50.0,
            mount_point: "/mnt/volumes/data".to_string(),
            filesystem: "ext4".to_string(),
            status: "online".to_string(),
            description: None,
            created_at: 1710000000,
            updated_at: 1774345600,
        },
    ];

    let mut volume = mock_volumes.into_iter().find(|v| v.id == volume_id);

    // 5. 验证存储卷存在
    if volume.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", volume_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let mut volume = volume.unwrap();

    // 6. 检查名称是否已存在（如果更新名称，排除自身）
    if let Some(ref new_name) = req.name {
        let existing_names = vec![
            (1, "root".to_string()),
            (2, "data".to_string()),
            (3, "backup".to_string()),
        ];
        let name_exists = existing_names.iter().any(|(id, name)| *id != volume_id && name == new_name);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("Storage volume name '{}' already exists", new_name),
                code: "CONFLICT".to_string(),
            }));
        }
        volume.name = new_name.clone();
        volume.mount_point = format!("/mnt/volumes/{}", new_name);
    }

    // 7. 验证新容量（如果更新容量，不能小于已用容量）
    if let Some(new_size) = req.size_bytes {
        if new_size < volume.used_bytes {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("New size cannot be smaller than used space. Used: {} bytes, Requested: {} bytes", volume.used_bytes, new_size),
                code: "INVALID_SIZE".to_string(),
            }));
        }
        volume.size_bytes = new_size;
        volume.available_bytes = new_size - volume.used_bytes;
        volume.usage_percent = (volume.used_bytes as f64 / new_size as f64 * 100.0) as f32;
    }

    // 8. 更新文件系统类型（如果提供）
    if let Some(ref new_fs_type) = req.filesystem_type {
        volume.filesystem = new_fs_type.to_lowercase();
    }

    // 9. 更新时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();
    volume.updated_at = now;

    // 10. 返回更新结果
    Ok(HttpResponse::Ok().json(UpdateVolumeResponse {
        success: true,
        message: "Storage volume updated successfully".to_string(),
        data: Some(volume),
    }))
}
