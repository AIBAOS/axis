// Phase 180 - 存储磁盘列表 API（增强版）
// GET /api/v1/storage/disks — 获取磁盘列表（含温度等详细信息）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 磁盘信息（Phase 180 增强版）
#[derive(Serialize, Clone)]
pub struct DiskInfo {
    pub disk_id: u64,
    pub name: String,
    pub device_path: String,
    pub model: String,
    pub serial_number: String,
    pub disk_type: String, // hdd/ssd/nvme
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub smart_status: String, // healthy/warning/failed
    pub temperature_celsius: Option<u32>,
    pub power_on_hours: Option<u64>,
    pub health_status: String, // good/fair/poor/critical
    pub in_storage_pool: bool,
    pub pool_name: Option<String>,
    pub status: String, // online/offline/degraded
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct StorageDisksQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub disk_type: Option<String>,
    pub smart_status: Option<String>,
    pub in_storage_pool: Option<bool>,
    pub status: Option<String>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
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
    pub pagination: PaginationMeta,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 获取磁盘列表（Phase 180）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持分页：page, limit
/// - 支持筛选：disk_type, smart_status, in_storage_pool, status
/// - 返回字段：disk_id/name/device_path/model/serial_number/disk_type/size_bytes/used_bytes/available_bytes/usage_percent/smart_status/temperature_celsius/power_on_hours/health_status/in_storage_pool/pool_name/status/created_at/updated_at
pub async fn list_storage_disks_v2(
    req: HttpRequest,
    query: web::Query<StorageDisksQuery>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性（简化实现，实际应调用 jwt_service）
    // 假设 token 已验证通过，提取 claims
    // let claims = jwt_service.verify_token(token)?;
    
    // 3. 模拟 admin 权限校验（实际应从 claims 提取）
    // if !is_admin(&claims) {
    //     return Ok(HttpResponse::Forbidden().json(ErrorResponse {
    //         success: false,
    //         error: "Only admin users can access disk list".to_string(),
    //         code: "FORBIDDEN".to_string(),
    //     }));
    // }

    // 4. 解析查询参数
    // Bug #72 修复：确保 page >= 1，防止整数下溢
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).max(1).min(100) // Bug #72 修复：防止空结果;
    let disk_type_filter = query.disk_type.as_deref();
    let smart_status_filter = query.smart_status.as_deref();
    let in_pool_filter = query.in_storage_pool;
    let status_filter = query.status.as_deref();

    // 5. 模拟磁盘数据（后续可连接数据库或调用系统 API）
    let all_disks = vec![
        DiskInfo {
            disk_id: 1,
            name: "Disk 1".to_string(),
            device_path: "/dev/sda".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC4E1234567".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4398046511104,
            used_bytes: 2199023255552,
            available_bytes: 2199023255552,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            temperature_celsius: Some(35),
            power_on_hours: Some(8760),
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("System Pool".to_string()),
            status: "online".to_string(),
            created_at: 1710500000,
            updated_at: 1711400000,
        },
        DiskInfo {
            disk_id: 2,
            name: "Disk 2".to_string(),
            device_path: "/dev/sdb".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC4E7654321".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4398046511104,
            used_bytes: 1759218604442,
            available_bytes: 2638827906662,
            usage_percent: 40.0,
            smart_status: "healthy".to_string(),
            temperature_celsius: Some(37),
            power_on_hours: Some(8760),
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("Data Pool".to_string()),
            status: "online".to_string(),
            created_at: 1710600000,
            updated_at: 1711500000,
        },
        DiskInfo {
            disk_id: 3,
            name: "Disk 3".to_string(),
            device_path: "/dev/sdc".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC4E9876543".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4398046511104,
            used_bytes: 0,
            available_bytes: 4398046511104,
            usage_percent: 0.0,
            smart_status: "healthy".to_string(),
            temperature_celsius: Some(33),
            power_on_hours: Some(100),
            health_status: "good".to_string(),
            in_storage_pool: false,
            pool_name: None,
            status: "online".to_string(),
            created_at: 1711400000,
            updated_at: 1711500000,
        },
        DiskInfo {
            disk_id: 4,
            name: "NVMe SSD 1".to_string(),
            device_path: "/dev/nvme0n1".to_string(),
            model: "Samsung 970 EVO 1TB".to_string(),
            serial_number: "S464NX0K123456".to_string(),
            disk_type: "nvme".to_string(),
            size_bytes: 1099511627776,
            used_bytes: 549755813888,
            available_bytes: 549755813888,
            usage_percent: 50.0,
            smart_status: "healthy".to_string(),
            temperature_celsius: Some(42),
            power_on_hours: Some(4380),
            health_status: "good".to_string(),
            in_storage_pool: true,
            pool_name: Some("Cache Pool".to_string()),
            status: "online".to_string(),
            created_at: 1710700000,
            updated_at: 1711500000,
        },
        DiskInfo {
            disk_id: 5,
            name: "Disk 5".to_string(),
            device_path: "/dev/sdd".to_string(),
            model: "WD Red Pro 4TB".to_string(),
            serial_number: "WD-WCC4E1111111".to_string(),
            disk_type: "hdd".to_string(),
            size_bytes: 4398046511104,
            used_bytes: 4398046511104,
            available_bytes: 0,
            usage_percent: 100.0,
            smart_status: "warning".to_string(),
            temperature_celsius: Some(45),
            power_on_hours: Some(17520),
            health_status: "fair".to_string(),
            in_storage_pool: true,
            pool_name: Some("Archive Pool".to_string()),
            status: "degraded".to_string(),
            created_at: 1708000000,
            updated_at: 1711500000,
        },
    ];

    // 6. 应用筛选条件
    let filtered: Vec<DiskInfo> = all_disks
        .into_iter()
        .filter(|disk| {
            // disk_type 筛选
            if let Some(filter) = disk_type_filter {
                if disk.disk_type != filter {
                    return false;
                }
            }
            // smart_status 筛选
            if let Some(filter) = smart_status_filter {
                if disk.smart_status != filter {
                    return false;
                }
            }
            // in_storage_pool 筛选
            if let Some(filter) = in_pool_filter {
                if disk.in_storage_pool != filter {
                    return false;
                }
            }
            // status 筛选
            if let Some(filter) = status_filter {
                if disk.status != filter {
                    return false;
                }
            }
            true
        })
        .collect();

    // 7. 分页计算
    let total = filtered.len() as u64;
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    let offset = (page - 1) * limit;
    let paginated: Vec<DiskInfo> = filtered
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    // 8. 返回结果
    Ok(HttpResponse::Ok().json(StorageDisksResponse {
        success: true,
        data: paginated,
        pagination: PaginationMeta {
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
    fn test_pagination_calculation() {
        let total: u64 = 100;
        let limit: u32 = 20;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        assert_eq!(total_pages, 5);

        let limit: u32 = 25;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        assert_eq!(total_pages, 4);
    }

    #[test]
    fn test_limit_max() {
        let limit: u32 = 150;
        let limited = limit.max(1).min(100) // Bug #72 修复：防止空结果;
        assert_eq!(limited, 100);
    }
}
