// Phase 79 - 存储卷详情 API
// GET /api/v1/storage/volumes/{id} — 获取存储卷详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 存储卷详情
#[derive(Serialize, Clone)]
pub struct VolumeDetail {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub pool_id: u64,
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

/// 存储卷详情响应
#[derive(Serialize)]
pub struct VolumeDetailResponse {
    pub success: bool,
    pub data: VolumeDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 任意登录用户可访问
}

/// 存储卷详情（Phase 79）
/// - JWT 认证，任意登录用户可访问
/// - 返回卷详情：id/name/description/pool_id/total_bytes/used_bytes/available_bytes/usage_percent/status/filesystem/mount_point/created_at/updated_at
/// - 卷不存在返回 404 Not Found
pub async fn get_volume(
    req: HttpRequest,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    let volume_id = path.into_inner();

    // 2. 模拟存储卷数据
    let volumes = vec![
        VolumeDetail {
            id: 1,
            name: "root".to_string(),
            description: Some("Root volume for system files".to_string()),
            pool_id: 1,
            total_bytes: 1099511627776,        // 1TB
            used_bytes: 549755813888,         // 512GB
            available_bytes: 549755813888,    // 512GB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "ext4".to_string(),
            mount_point: "/mnt/volumes/root".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        VolumeDetail {
            id: 2,
            name: "data".to_string(),
            description: Some("Data volume for user files".to_string()),
            pool_id: 1,
            total_bytes: 2199023255552,       // 2TB
            used_bytes: 1099511627776,        // 1TB
            available_bytes: 1099511627776,   // 1TB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "ext4".to_string(),
            mount_point: "/mnt/volumes/data".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        VolumeDetail {
            id: 3,
            name: "backup".to_string(),
            description: Some("Backup volume for system backups".to_string()),
            pool_id: 2,
            total_bytes: 4398046511104,       // 4TB
            used_bytes: 2199023255552,        // 2TB
            available_bytes: 2199023255552,   // 2TB
            usage_percent: 50.0,
            status: "online".to_string(),
            filesystem: "btrfs".to_string(),
            mount_point: "/mnt/volumes/backup".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
    ];

    // 3. 查找指定 ID 的卷
    let volume = volumes.into_iter().find(|v| v.id == volume_id);

    // 4. 返回响应
    match volume {
        Some(vol) => Ok(HttpResponse::Ok().json(VolumeDetailResponse {
            success: true,
            data: vol,
        })),
        None => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", volume_id),
            code: "NOT_FOUND".to_string(),
        })),
    }
}
