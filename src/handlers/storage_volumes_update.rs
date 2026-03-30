// Phase 68 - 存储卷更新 API
// PUT /api/v1/storage/volumes/{id} — 更新存储卷信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 更新存储卷请求
#[derive(Deserialize)]
pub struct UpdateStorageVolumeRequest {
    pub name: Option<String>,
    pub size_bytes: Option<u64>,
    pub filesystem_type: Option<String>,
}

/// 存储卷响应
#[derive(Serialize)]
pub struct StorageVolumeResponse {
    pub id: u64,
    pub name: String,
    pub pool_id: u64,
    pub pool_name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub filesystem_type: String,
    pub mount_point: String,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 更新存储卷响应
#[derive(Serialize)]
pub struct UpdateStorageVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: StorageVolumeResponse,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 更新存储卷（Phase 68）
/// - JWT 认证，仅 admin 角色可访问
/// - 可更新字段：name, size_bytes
/// - filesystem_type 不可变更
/// - 验证存储卷存在性、名称唯一性、容量限制
pub async fn update_storage_volume(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateStorageVolumeRequest>,
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

    // 2. 权限校验 - 仅 admin 角色可更新存储卷
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let volume_id = path.into_inner();

    // 3. 验证文件系统类型不可变更
    if payload.filesystem_type.is_some() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "filesystem_type cannot be changed after volume creation".to_string(),
            code: "FILESYSTEM_IMMUTABLE".to_string(),
        }));
    }

    // 4. 模拟查找存储卷（后续可连接数据库）
    let mock_volumes = vec![
        serde_json::json!({
            "id": 1,
            "name": "System Volume",
            "pool_id": 1,
            "pool_name": "System Pool",
            "size_bytes": 250 * 1024 * 1024 * 1024,
            "used_bytes": 125 * 1024 * 1024 * 1024,
            "available_bytes": 125 * 1024 * 1024 * 1024,
            "filesystem_type": "ext4",
            "mount_point": "/mnt/system_volume",
            "status": "online",
            "created_at": 1710489600,
            "updated_at": 1711440000,
        }),
        serde_json::json!({
            "id": 2,
            "name": "Data Volume",
            "pool_id": 2,
            "pool_name": "Data Pool",
            "size_bytes": 1000 * 1024 * 1024 * 1024,
            "used_bytes": 500 * 1024 * 1024 * 1024,
            "available_bytes": 500 * 1024 * 1024 * 1024,
            "filesystem_type": "ext4",
            "mount_point": "/mnt/data_volume",
            "status": "online",
            "created_at": 1710489600,
            "updated_at": 1711440000,
        }),
        serde_json::json!({
            "id": 3,
            "name": "Backup Volume",
            "pool_id": 3,
            "pool_name": "Backup Pool",
            "size_bytes": 2000 * 1024 * 1024 * 1024,
            "used_bytes": 800 * 1024 * 1024 * 1024,
            "available_bytes": 1200 * 1024 * 1024 * 1024,
            "filesystem_type": "btrfs",
            "mount_point": "/mnt/backup_volume",
            "status": "online",
            "created_at": 1710489600,
            "updated_at": 1711440000,
        }),
    ];

    let mut volume = mock_volumes.into_iter().find(|p| p["id"] == volume_id);

    match volume {
        Some(ref mut v) => {
            // 5. 验证名称唯一性（排除自身）
            if let Some(ref new_name) = payload.name {
                let existing_names = vec!["System Volume", "Data Volume", "Backup Volume", "Archive Volume"];
                let current_name = v["name"].as_str().unwrap_or("unknown");
                if existing_names.contains(&new_name.as_str()) && new_name != current_name {
                    return Ok(HttpResponse::Conflict().json(ErrorResponse {
                        success: false,
                        error: format!("Storage volume '{}' already exists", new_name),
                        code: "NAME_CONFLICT".to_string(),
                    }));
                }
                v["name"] = serde_json::json!(new_name);
                // 更新挂载点
                v["mount_point"] = serde_json::json!(format!("/mnt/{}", new_name.to_lowercase().replace(" ", "_")));
            }

            // 6. 验证容量缩小限制
            if let Some(new_size) = payload.size_bytes {
                let used_bytes = v["used_bytes"].as_u64().unwrap_or(0);
                if new_size < used_bytes {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: format!("Cannot shrink volume below used space ({} bytes)", used_bytes),
                        code: "SIZE_TOO_SMALL".to_string(),
                    }));
                }
                v["size_bytes"] = serde_json::json!(new_size);
                v["available_bytes"] = serde_json::json!(new_size.saturating_sub(used_bytes));
            }

            // 7. 更新时间戳
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();
            v["updated_at"] = serde_json::json!(now);

            // 8. 返回更新后的存储卷信息
            let response = StorageVolumeResponse {
                id: v["id"].as_u64().unwrap_or(0),
                name: v["name"].as_str().unwrap_or("unknown").to_string(),
                pool_id: v["pool_id"].as_u64().unwrap_or(0),
                pool_name: v["pool_name"].as_str().unwrap_or("unknown").to_string(),
                size_bytes: v["size_bytes"].as_u64().unwrap_or(0),
                used_bytes: v["used_bytes"].as_u64().unwrap_or(0),
                available_bytes: v["available_bytes"].as_u64().unwrap_or(0),
                filesystem_type: v["filesystem_type"].as_str().unwrap_or("unknown").to_string(),
                mount_point: v["mount_point"].as_str().unwrap_or("/mnt/unknown").to_string(),
                status: v["status"].as_str().unwrap_or("unknown").to_string(),
                created_at: v["created_at"].as_u64().unwrap_or(now),
                updated_at: now,
            };

            Ok(HttpResponse::Ok().json(UpdateStorageVolumeResponse {
                success: true,
                message: "Storage volume updated successfully".to_string(),
                data: response,
            }))
        }
        None => {
            // 9. 存储卷不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
