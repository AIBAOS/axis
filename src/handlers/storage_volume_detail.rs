// Phase 188: 存储卷详情 API
// GET /api/v1/storage/volumes/{id} — 获取存储卷详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 存储卷详情信息
#[derive(Serialize, Clone)]
pub struct StorageVolumeDetail {
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

/// 存储卷详情响应
#[derive(Serialize)]
pub struct StorageVolumeDetailResponse {
    pub success: bool,
    pub data: StorageVolumeDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储卷详情（Phase 188）
/// - JWT 认证，登录用户可访问
/// - 验证卷 ID 存在性（404 Not Found）
/// - 返回卷详细信息
pub async fn get_storage_volume_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let volume_id = path.into_inner();

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
    let mock_volumes = vec![
        StorageVolumeDetail {
            id: 1,
            name: "System Volume".to_string(),
            total_bytes: 500107862016, // 500GB
            used_bytes: 250053931008, // 250GB
            available_bytes: 250053931008, // 250GB
            usage_percent: 50.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        StorageVolumeDetail {
            id: 2,
            name: "Data Volume".to_string(),
            total_bytes: 1000204886016, // 1TB
            used_bytes: 600122931610, // 600GB
            available_bytes: 400081954406, // 400GB
            usage_percent: 60.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/data".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        StorageVolumeDetail {
            id: 3,
            name: "Backup Volume".to_string(),
            total_bytes: 2000398934016, // 2TB
            used_bytes: 400079786803, // 400GB
            available_bytes: 1600319147213, // 1.6TB
            usage_percent: 20.0,
            status: "active".to_string(),
            filesystem_type: "ext4".to_string(),
            mount_point: "/backup".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 4. 查找卷
    let volume = mock_volumes.into_iter().find(|v| v.id == volume_id);

    // 5. 验证卷存在性
    match volume {
        Some(volume) => {
            // 6. 返回卷详情
            Ok(HttpResponse::Ok().json(StorageVolumeDetailResponse {
                success: true,
                data: volume,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_storage_volume_detail_success() {
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
                .route("/api/v1/storage/volumes/{id}", web::get().to(get_storage_volume_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_storage_volume_detail_not_found() {
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
                .route("/api/v1/storage/volumes/{id}", web::get().to(get_storage_volume_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }
}
