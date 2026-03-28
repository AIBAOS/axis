// Phase 180: 存储磁盘列表 API
// GET /api/v1/storage/disks — 获取存储磁盘列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 磁盘信息
#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub id: u64,
    pub device: String,
    pub model: String,
    pub serial: String,
    pub size: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
    pub status: String,
    pub health: String,
    pub temperature: u32,
    pub created_at: String,
}

/// 磁盘列表响应
#[derive(Serialize)]
pub struct DiskListResponse {
    pub success: bool,
    pub data: Vec<DiskInfo>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储磁盘列表（Phase 180）
/// - JWT 认证，admin 角色可访问
/// - 返回所有存储磁盘信息列表
pub async fn list_storage_disks(
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
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view storage disks".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟磁盘数据
    let disks = vec![
        DiskInfo {
            id: 1,
            device: "/dev/sda".to_string(),
            model: "Samsung SSD 860 EVO 500GB".to_string(),
            serial: "S3Z1NB0K123456".to_string(),
            size: 500107862016, // 500GB
            used: 250053931008, // 250GB
            available: 250053931008, // 250GB
            usage_percent: 50.0,
            status: "online".to_string(),
            health: "good".to_string(),
            temperature: 35,
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        DiskInfo {
            id: 2,
            device: "/dev/sdb".to_string(),
            model: "Western Digital WD Blue 1TB".to_string(),
            serial: "WD-WCC4E1234567".to_string(),
            size: 1000204886016, // 1TB
            used: 600122931610, // 600GB
            available: 400081954406, // 400GB
            usage_percent: 60.0,
            status: "online".to_string(),
            health: "good".to_string(),
            temperature: 38,
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        DiskInfo {
            id: 3,
            device: "/dev/sdc".to_string(),
            model: "Seagate Barracuda 2TB".to_string(),
            serial: "ZDH1234567".to_string(),
            size: 2000398934016, // 2TB
            used: 400079786803, // 400GB
            available: 1600319147213, // 1.6TB
            usage_percent: 20.0,
            status: "online".to_string(),
            health: "good".to_string(),
            temperature: 32,
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 返回磁盘列表
    Ok(HttpResponse::Ok().json(DiskListResponse {
        success: true,
        data: disks,
    }))
}
