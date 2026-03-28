// Phase 74 - 磁盘详情 API
// GET /api/v1/storage/disks/{id} — 获取单个磁盘详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 磁盘详情响应
#[derive(Serialize)]
pub struct DiskDetailResponse {
    pub success: bool,
    pub data: DiskInfo,
}

/// 磁盘信息
#[derive(Serialize)]
pub struct DiskInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub model: String,
    pub serial_number: String,
    pub disk_type: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub smart_status: String,
    pub speed_rpm: Option<u32>,
    pub health_status: String,
    pub in_storage_pool: bool,
    pub pool_name: Option<String>,
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

/// 获取磁盘详情（Phase 74）
/// - JWT 认证，任意登录用户可访问
/// - 返回单个磁盘的详细信息
/// - 磁盘不存在返回 404 Not Found
pub async fn get_storage_disk(
    req: HttpRequest,
    path: web::Path<u64>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
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

    let disk_id = path.into_inner();

    // 3. 模拟磁盘数据（后续可连接数据库）
    let mock_disks = vec![
        DiskInfo {
            id: 1,
            name: "Disk 1".to_string(),
            path: "/dev/sda".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC12345678".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4000 * 1024 * 1024 * 1024,
            used_bytes: 2000 * 1024 * 1024 * 1024,
            available_bytes: 2000 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            speed_rpm: Some(7200),
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("System Pool".to_string()),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 2,
            name: "Disk 2".to_string(),
            path: "/dev/sdb".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC87654321".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4000 * 1024 * 1024 * 1024,
            used_bytes: 2000 * 1024 * 1024 * 1024,
            available_bytes: 2000 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            speed_rpm: Some(7200),
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("Data Pool".to_string()),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 3,
            name: "Disk 3".to_string(),
            path: "/dev/sdc".to_string(),
            model: "Samsung 970 EVO 1TB".to_string(),
            serial_number: "S464NX0M123456".to_string(),
            disk_type: "nvme".to_string(),
            size_bytes: 1000 * 1024 * 1024 * 1024,
            used_bytes: 500 * 1024 * 1024 * 1024,
            available_bytes: 500 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            speed_rpm: None,
            health_status: "good".to_string(),
            in_storage_pool: false,
            pool_name: None,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 4,
            name: "Disk 4".to_string(),
            path: "/dev/sdd".to_string(),
            model: "Crucial MX500 2TB".to_string(),
            serial_number: "2038E5E71234".to_string(),
            disk_type: "ssd".to_string(),
            size_bytes: 2000 * 1024 * 1024 * 1024,
            used_bytes: 800 * 1024 * 1024 * 1024,
            available_bytes: 1200 * 1024 * 1024 * 1024,
            usage_percent: 40.0,
            smart_status: "healthy".to_string(),
            speed_rpm: None,
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("Backup Pool".to_string()),
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 5,
            name: "Disk 5".to_string(),
            path: "/dev/sde".to_string(),
            model: "WD Blue 2TB".to_string(),
            serial_number: "WD-WCC87654000".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 2000 * 1024 * 1024 * 1024,
            used_bytes: 1000 * 1024 * 1024 * 1024,
            available_bytes: 1000 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            smart_status: "warning".to_string(),
            speed_rpm: Some(5400),
            health_status: "warning".to_string(),
            in_storage_pool: false,
            pool_name: None,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 6,
            name: "Disk 6".to_string(),
            path: "/dev/sdf".to_string(),
            model: "Seagate IronWolf 8TB".to_string(),
            serial_number: "ZC123456".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 8000 * 1024 * 1024 * 1024,
            used_bytes: 4000 * 1024 * 1024 * 1024,
            available_bytes: 4000 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            speed_rpm: Some(7200),
            health_status: "good".to_string(),
            in_storage_pool: false,
            pool_name: None,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
    ];

    // 4. 查找磁盘
    let disk = mock_disks.into_iter().find(|d| d.id == disk_id);

    match disk {
        Some(d) => {
            Ok(HttpResponse::Ok().json(DiskDetailResponse {
                success: true,
                data: d,
            }))
        }
        None => {
            // 5. 磁盘不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Disk {} not found", disk_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
