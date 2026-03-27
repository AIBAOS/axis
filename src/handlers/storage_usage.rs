// Phase 77 - 存储使用统计 API
// GET /api/v1/storage/usage — 获取全局存储使用统计

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 存储使用统计响应
#[derive(Serialize)]
pub struct StorageUsageResponse {
    pub success: bool,
    pub data: StorageUsage,
}

/// 存储使用统计
#[derive(Serialize)]
pub struct StorageUsage {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub pool_count: u32,
    pub volume_count: u32,
    pub disk_count: u32,
    pub health_status: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储使用统计（Phase 77）
/// - JWT 认证，任意登录用户可访问
/// - 返回全局存储统计信息
pub async fn get_storage_usage(
    req: HttpRequest,
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

    // 3. 模拟存储池数据
    let mock_pools = vec![
        serde_json::json!({
            "id": 1,
            "name": "System Pool",
            "total_bytes": 500 * 1024 * 1024 * 1024,
            "used_bytes": 250 * 1024 * 1024 * 1024,
            "available_bytes": 250 * 1024 * 1024 * 1024,
            "disk_count": 1,
            "volume_count": 1,
            "health_status": "healthy",
        }),
        serde_json::json!({
            "id": 2,
            "name": "Data Pool",
            "total_bytes": 4000 * 1024 * 1024 * 1024,
            "used_bytes": 1600 * 1024 * 1024 * 1024,
            "available_bytes": 2400 * 1024 * 1024 * 1024,
            "disk_count": 2,
            "volume_count": 1,
            "health_status": "healthy",
        }),
        serde_json::json!({
            "id": 3,
            "name": "Backup Pool",
            "total_bytes": 12000 * 1024 * 1024 * 1024,
            "used_bytes": 4800 * 1024 * 1024 * 1024,
            "available_bytes": 7200 * 1024 * 1024 * 1024,
            "disk_count": 4,
            "volume_count": 1,
            "health_status": "healthy",
        }),
        serde_json::json!({
            "id": 4,
            "name": "Archive Pool",
            "total_bytes": 20000 * 1024 * 1024 * 1024,
            "used_bytes": 10000 * 1024 * 1024 * 1024,
            "available_bytes": 10000 * 1024 * 1024 * 1024,
            "disk_count": 6,
            "volume_count": 1,
            "health_status": "degraded",
        }),
    ];

    // 4. 计算总体统计
    let pool_count = mock_pools.len() as u32;
    let total_bytes: u64 = mock_pools.iter().map(|p| p["total_bytes"].as_u64().unwrap()).sum();
    let used_bytes: u64 = mock_pools.iter().map(|p| p["used_bytes"].as_u64().unwrap()).sum();
    let available_bytes: u64 = mock_pools.iter().map(|p| p["available_bytes"].as_u64().unwrap()).sum();
    let disk_count: u32 = mock_pools.iter().map(|p| p["disk_count"].as_u64().unwrap() as u32).sum();
    let volume_count: u32 = mock_pools.iter().map(|p| p["volume_count"].as_u64().unwrap() as u32).sum();

    // 5. 计算使用率
    let usage_percent = if total_bytes > 0 {
        (used_bytes as f64 / total_bytes as f64) * 100.0
    } else {
        0.0
    };

    // 6. 确定整体健康状态
    let has_critical = mock_pools.iter().any(|p| p["health_status"].as_str() == Some("critical"));
    let has_degraded = mock_pools.iter().any(|p| p["health_status"].as_str() == Some("degraded"));
    
    let health_status = if has_critical {
        "critical"
    } else if has_degraded {
        "degraded"
    } else {
        "healthy"
    }.to_string();

    Ok(HttpResponse::Ok().json(StorageUsageResponse {
        success: true,
        data: StorageUsage {
            total_bytes,
            used_bytes,
            available_bytes,
            usage_percent,
            pool_count,
            volume_count,
            disk_count,
            health_status,
        },
    }))
}
