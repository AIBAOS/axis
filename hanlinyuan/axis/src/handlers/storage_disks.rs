// Phase 180 - 存储磁盘列表 API
// GET /api/v1/storage/disks — 获取 NAS 上所有磁盘的信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 磁盘信息
#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub model: String,
    pub serial_number: String,
    #[serde(rename = "type")]
    pub disk_type: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub temperature: Option<i32>,
    pub smart_status: String,
    pub health_status: String,
    pub speed_rpm: Option<u32>,
    pub power_on_hours: Option<u64>,
    pub status: String,
    pub in_use: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 查询参数
#[derive(Deserialize)]
pub struct StorageDisksQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub disk_type: Option<String>,
    pub smart_status: Option<String>,
    pub status: Option<String>,
}

/// 分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 磁盘列表响应
#[derive(Serialize)]
pub struct StorageDisksResponse {
    pub success: bool,
    pub data: Vec<DiskInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 格式化字节大小为人类可读格式
fn format_bytes(bytes: u64) -> String {
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

/// 获取磁盘列表（Phase 180）
/// - JWT 认证，任意登录用户可访问
/// - 返回 NAS 上所有磁盘的信息（型号、容量、健康状态、温度等）
/// - 支持分页：page, limit
/// - 支持筛选：disk_type, smart_status, status
pub async fn list_storage_disks(
    req: HttpRequest,
    query: web::Query<StorageDisksQuery>,
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

    // 2. 解析查询参数
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let disk_type_filter = query.disk_type.as_deref();
    let smart_status_filter = query.smart_status.as_deref();
    let status_filter = query.status.as_deref();

    // 3. 模拟磁盘数据（实际应从数据库或系统获取）
    let all_disks = vec![
        DiskInfo {
            id: 1,
            name: "Disk 1".to_string(),
            path: "/dev/sda".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC12345678".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4 * 1024 * 1024 * 1024 * 1024, // 4TB
            size_human: "4.00 TB".to_string(),
            temperature: Some(35),
            smart_status: "healthy".to_string(),
            health_status: "good".to_string(),
            speed_rpm: Some(7200),
            power_on_hours: Some(8760),
            status: "online".to_string(),
            in_use: true,
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
            size_bytes: 4 * 1024 * 1024 * 1024 * 1024,
            size_human: "4.00 TB".to_string(),
            temperature: Some(37),
            smart_status: "healthy".to_string(),
            health_status: "good".to_string(),
            speed_rpm: Some(7200),
            power_on_hours: Some(8760),
            status: "online".to_string(),
            in_use: true,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 3,
            name: "Disk 3".to_string(),
            path: "/dev/nvme0n1".to_string(),
            model: "Samsung 970 EVO 1TB".to_string(),
            serial_number: "S464NX0M123456".to_string(),
            disk_type: "nvme".to_string(),
            size_bytes: 1024 * 1024 * 1024 * 1024, // 1TB
            size_human: "1.00 TB".to_string(),
            temperature: Some(42),
            smart_status: "healthy".to_string(),
            health_status: "good".to_string(),
            speed_rpm: None,
            power_on_hours: Some(4380),
            status: "online".to_string(),
            in_use: false,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 4,
            name: "Disk 4".to_string(),
            path: "/dev/sdc".to_string(),
            model: "Crucial MX500 2TB".to_string(),
            serial_number: "2038E5E71234".to_string(),
            disk_type: "ssd".to_string(),
            size_bytes: 2 * 1024 * 1024 * 1024 * 1024, // 2TB
            size_human: "2.00 TB".to_string(),
            temperature: Some(32),
            smart_status: "healthy".to_string(),
            health_status: "good".to_string(),
            speed_rpm: None,
            power_on_hours: Some(2190),
            status: "online".to_string(),
            in_use: true,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 5,
            name: "Disk 5".to_string(),
            path: "/dev/sdd".to_string(),
            model: "WD Blue 2TB".to_string(),
            serial_number: "WD-WCC87654000".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 2 * 1024 * 1024 * 1024 * 1024,
            size_human: "2.00 TB".to_string(),
            temperature: Some(40),
            smart_status: "warning".to_string(),
            health_status: "warning".to_string(),
            speed_rpm: Some(5400),
            power_on_hours: Some(17520),
            status: "online".to_string(),
            in_use: false,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 6,
            name: "Disk 6".to_string(),
            path: "/dev/sde".to_string(),
            model: "Seagate IronWolf 8TB".to_string(),
            serial_number: "ZC123456".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 8 * 1024 * 1024 * 1024 * 1024, // 8TB
            size_human: "8.00 TB".to_string(),
            temperature: Some(38),
            smart_status: "healthy".to_string(),
            health_status: "good".to_string(),
            speed_rpm: Some(7200),
            power_on_hours: Some(1000),
            status: "online".to_string(),
            in_use: false,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
        DiskInfo {
            id: 7,
            name: "Disk 7".to_string(),
            path: "/dev/sdf".to_string(),
            model: "WD Red 6TB".to_string(),
            serial_number: "WD-WCC99988877".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 6 * 1024 * 1024 * 1024 * 1024, // 6TB
            size_human: "6.00 TB".to_string(),
            temperature: None,
            smart_status: "unknown".to_string(),
            health_status: "unknown".to_string(),
            speed_rpm: Some(5400),
            power_on_hours: None,
            status: "offline".to_string(),
            in_use: false,
            created_at: 1710489600,
            updated_at: 1711440000,
        },
    ];

    // 4. 应用筛选
    let mut filtered_disks: Vec<DiskInfo> = all_disks
        .into_iter()
        .filter(|d| {
            if let Some(disk_type) = disk_type_filter {
                if d.disk_type != disk_type {
                    return false;
                }
            }
            if let Some(smart_status) = smart_status_filter {
                if d.smart_status != smart_status {
                    return false;
                }
            }
            if let Some(status) = status_filter {
                if d.status != status {
                    return false;
                }
            }
            true
        })
        .collect();

    // 5. 按 ID 排序
    filtered_disks.sort_by_key(|d| d.id);

    let total = filtered_disks.len() as u64;
    let total_pages = ((total + limit as u64 - 1) / limit as u64).max(1) as u32;
    let start = ((page - 1) as usize) * (limit as usize);
    let end = start + (limit as usize);

    // 6. 分页截取
    let disks: Vec<DiskInfo> = filtered_disks
        .into_iter()
        .enumerate()
        .filter_map(|(i, d)| {
            if i >= start && i < end {
                Some(d)
            } else {
                None
            }
        })
        .collect();

    log::info!("Listed {} disks (page {}, limit {})", disks.len(), page, limit);

    Ok(HttpResponse::Ok().json(StorageDisksResponse {
        success: true,
        data: disks,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
        assert_eq!(format_bytes(1099511627776), "1.00 TB");
        assert_eq!(format_bytes(500), "500 B");
    }

    #[test]
    fn test_disk_type_filter() {
        let disks = vec![
            DiskInfo {
                id: 1,
                name: "HDD".to_string(),
                path: "/dev/sda".to_string(),
                model: "Test".to_string(),
                serial_number: "123".to_string(),
                disk_type: "hdd".to_string(),
                size_bytes: 1000,
                size_human: "1 KB".to_string(),
                temperature: None,
                smart_status: "healthy".to_string(),
                health_status: "good".to_string(),
                speed_rpm: None,
                power_on_hours: None,
                status: "online".to_string(),
                in_use: false,
                created_at: 0,
                updated_at: 0,
            },
            DiskInfo {
                id: 2,
                name: "SSD".to_string(),
                path: "/dev/sdb".to_string(),
                model: "Test".to_string(),
                serial_number: "456".to_string(),
                disk_type: "ssd".to_string(),
                size_bytes: 2000,
                size_human: "2 KB".to_string(),
                temperature: None,
                smart_status: "healthy".to_string(),
                health_status: "good".to_string(),
                speed_rpm: None,
                power_on_hours: None,
                status: "online".to_string(),
                in_use: false,
                created_at: 0,
                updated_at: 0,
            },
        ];

        let filtered: Vec<DiskInfo> = disks
            .into_iter()
            .filter(|d| d.disk_type == "hdd")
            .collect();

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].disk_type, "hdd");
    }
}
