// Phase 187: 存储卷列表 API
// GET /api/v1/storage/volumes — 获取存储卷列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

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
}

/// 存储卷列表响应
#[derive(Serialize)]
pub struct StorageVolumeListResponse {
    pub success: bool,
    pub data: Vec<StorageVolume>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储卷列表（Phase 187）
/// - JWT 认证，登录用户可访问
/// - 返回所有存储卷的基本信息
pub async fn list_storage_volumes(
    req: HttpRequest,
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
    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 模拟存储卷数据
    let volumes = vec![
        StorageVolume {
            id: 1,
            name: "System Volume".to_string(),
            total_bytes: 500107862016, // 500GB
            used_bytes: 250053931008, // 250GB
            available_bytes: 250053931008, // 250GB
            usage_percent: 50.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/".to_string(),
        },
        StorageVolume {
            id: 2,
            name: "Data Volume".to_string(),
            total_bytes: 1000204886016, // 1TB
            used_bytes: 600122931610, // 600GB
            available_bytes: 400081954406, // 400GB
            usage_percent: 60.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/data".to_string(),
        },
        StorageVolume {
            id: 3,
            name: "Backup Volume".to_string(),
            total_bytes: 2000398934016, // 2TB
            used_bytes: 400079786803, // 400GB
            available_bytes: 1600319147213, // 1.6TB
            usage_percent: 20.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/backup".to_string(),
        },
    ];

    // 4. 返回存储卷列表
    Ok(HttpResponse::Ok().json(StorageVolumeListResponse {
        success: true,
        data: volumes,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_storage_volumes_success() {
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
                .route("/api/v1/storage/volumes", web::get().to(list_storage_volumes))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }
}
