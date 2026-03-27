// Phase 181: 存储磁盘详情 API
// GET /api/v1/storage/disks/{id} — 获取存储磁盘详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 磁盘详情信息
#[derive(Serialize, Clone)]
pub struct DiskDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub model: String,
    pub serial_number: String,
    pub disk_type: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub temperature: u32,
    pub smart_status: String,
    pub health_status: String,
    pub speed_rpm: Option<u32>,
    pub power_on_hours: u64,
    pub status: String,
    pub in_use: bool,
    pub storage_pool_id: Option<u64>,
}

/// 磁盘详情响应
#[derive(Serialize)]
pub struct DiskDetailResponse {
    pub success: bool,
    pub data: DiskDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 格式化容量为人类可读格式
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 获取存储磁盘详情（Phase 181）
/// - JWT 认证，任意登录用户可访问
/// - 验证磁盘 ID 存在性（404 Not Found）
/// - 返回磁盘详细信息
pub async fn get_storage_disk_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let disk_id = path.into_inner();

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

    // 3. 模拟磁盘数据
    let mock_disks = vec![
        DiskDetail {
            id: 1,
            name: "System Disk".to_string(),
            path: "/dev/sda".to_string(),
            model: "Samsung SSD 860 EVO 500GB".to_string(),
            serial_number: "S3Z1NB0K123456".to_string(),
            disk_type: "ssd".to_string(),
            size_bytes: 500107862016, // 500GB
            size_human: "500.00 GB".to_string(),
            temperature: 35,
            smart_status: "passed".to_string(),
            health_status: "good".to_string(),
            speed_rpm: None, // SSD
            power_on_hours: 8760, // 1 year
            status: "online".to_string(),
            in_use: true,
            storage_pool_id: Some(1),
        },
        DiskDetail {
            id: 2,
            name: "Data Disk 1".to_string(),
            path: "/dev/sdb".to_string(),
            model: "Western Digital WD Blue 1TB".to_string(),
            serial_number: "WD-WCC4E1234567".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 1000204886016, // 1TB
            size_human: "1.00 TB".to_string(),
            temperature: 38,
            smart_status: "passed".to_string(),
            health_status: "good".to_string(),
            speed_rpm: Some(7200),
            power_on_hours: 17520, // 2 years
            status: "online".to_string(),
            in_use: true,
            storage_pool_id: Some(2),
        },
        DiskDetail {
            id: 3,
            name: "Data Disk 2".to_string(),
            path: "/dev/sdc".to_string(),
            model: "Seagate Barracuda 2TB".to_string(),
            serial_number: "ZDH1234567".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 2000398934016, // 2TB
            size_human: "2.00 TB".to_string(),
            temperature: 32,
            smart_status: "passed".to_string(),
            health_status: "good".to_string(),
            speed_rpm: Some(7200),
            power_on_hours: 4380, // 6 months
            status: "online".to_string(),
            in_use: false,
            storage_pool_id: None,
        },
    ];

    // 4. 查找磁盘
    let disk = mock_disks.into_iter().find(|d| d.id == disk_id);

    // 5. 验证磁盘存在性
    match disk {
        Some(disk) => {
            // 6. 返回磁盘详情
            Ok(HttpResponse::Ok().json(DiskDetailResponse {
                success: true,
                data: disk,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Disk {} not found", disk_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
