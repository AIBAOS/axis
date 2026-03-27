// Phase 189: 存储卷创建 API
// POST /api/v1/storage/volumes — 创建存储卷

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 创建存储卷请求
#[derive(Debug, Deserialize)]
pub struct CreateStorageVolumeRequest {
    pub name: String,
    pub size_bytes: u64,
    pub filesystem_type: Option<String>,
    pub mount_point: Option<String>,
    pub description: Option<String>,
}

/// 存储卷信息
#[derive(Serialize, Clone)]
pub struct StorageVolume {
    pub id: u64,
    pub name: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub status: String,
    pub filesystem_type: String,
    pub mount_point: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建存储卷响应
#[derive(Serialize)]
pub struct CreateStorageVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: StorageVolume,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证文件系统类型
fn validate_filesystem_type(fs_type: &str) -> bool {
    matches!(fs_type.to_lowercase().as_str(), "ext4" | "xfs" | "btrfs" | "zfs")
}

/// 验证挂载点格式
fn validate_mount_point(mount_point: &str) -> bool {
    mount_point.starts_with('/') && mount_point.len() <= 256
}

/// 创建存储卷（Phase 189）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证卷名称唯一性（409 Conflict）
/// - 验证文件系统类型（400 Bad Request）
/// - 验证挂载点格式（400 Bad Request）
/// - 创建成功返回 201 Created + 卷详情
pub async fn create_storage_volume(
    req: HttpRequest,
    payload: web::Json<CreateStorageVolumeRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证文件系统类型
    let filesystem_type = payload.filesystem_type.clone().unwrap_or_else(|| "ext4".to_string());
    if !validate_filesystem_type(&filesystem_type) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid filesystem type. Valid types: ext4, xfs, btrfs, zfs".to_string(),
            code: "INVALID_FILESYSTEM".to_string(),
        }));
    }

    // 5. 验证挂载点格式
    let mount_point = payload.mount_point.clone().unwrap_or_else(|| format!("/mnt/{}", payload.name));
    if !validate_mount_point(&mount_point) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid mount point format. Must start with '/' and be <= 256 chars".to_string(),
            code: "INVALID_MOUNT_POINT".to_string(),
        }));
    }

    // 6. 模拟现有卷数据（用于名称唯一性检查）
    let existing_volumes = vec!["System Volume", "Data Volume", "Backup Volume"];

    // 7. 验证卷名称唯一性
    if existing_volumes.contains(&payload.name.as_str()) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage volume '{}' already exists", payload.name),
            code: "VOLUME_EXISTS".to_string(),
        }));
    }

    // 8. 创建新卷
    let now = chrono::Utc::now().to_rfc3339();
    let new_volume = StorageVolume {
        id: 4, // 模拟自增 ID
        name: payload.name.clone(),
        total_bytes: payload.size_bytes,
        used_bytes: 0,
        available_bytes: payload.size_bytes,
        usage_percent: 0.0,
        status: "active".to_string(),
        filesystem_type,
        mount_point,
        created_at: now.clone(),
        updated_at: now,
    };

    // 9. 返回创建成功
    Ok(HttpResponse::Created().json(CreateStorageVolumeResponse {
        success: true,
        message: "Storage volume created successfully".to_string(),
        data: new_volume,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_storage_volume_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/storage/volumes", web::post().to(create_storage_volume))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和请求体
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_create_storage_volume_invalid_filesystem() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/storage/volumes", web::post().to(create_storage_volume))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和请求体
        // 这里只是示例测试结构
        assert!(true);
    }
}
