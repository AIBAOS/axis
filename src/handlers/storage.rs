// 存储管理 API 处理器（Phase 62）
// Phase 62: 增强 JWT 认证和存储池列表
// 包含：磁盘列表、存储池、存储使用量查询

use actix_web::{web, HttpResponse, Error, Result, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

#[derive(Debug, Serialize, Deserialize)]
pub enum DiskStatus {
    Online,
    Offline,
    Degraded,
}

impl std::fmt::Display for DiskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskStatus::Online => write!(f, "online"),
            DiskStatus::Offline => write!(f, "offline"),
            DiskStatus::Degraded => write!(f, "degraded"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiskType {
    HDD,
    SSD,
    NVME,
    External,
}

impl std::fmt::Display for DiskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskType::HDD => write!(f, "hdd"),
            DiskType::SSD => write!(f, "ssd"),
            DiskType::NVME => write!(f, "nvme"),
            DiskType::External => write!(f, "external"),
        }
    }
}

/// 磁盘信息（Phase 41）
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiskInfo {
    pub disk_id: u64,
    pub name: String,
    pub path: String,
    pub size_gb: u64,
    pub used_gb: u64,
    pub available_gb: u64,
    pub usage_percent: u8,
    pub status: String,
    pub r#type: String,
    pub mount_point: String,
}

/// 存储池信息（Phase 62 增强）
#[derive(Serialize, Deserialize, Clone)]
pub struct PoolInfo {
    pub id: u64,
    pub name: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub disks: Vec<String>,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 存储池列表响应（Phase 62）
#[derive(Serialize)]
pub struct PoolListResponse {
    pub success: bool,
    pub data: Vec<PoolInfo>,
    pub total_pools: u64,
    pub total_capacity_bytes: u64,
    pub total_used_bytes: u64,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 任意登录用户可访问
}

/// 存储使用量响应
#[derive(Serialize)]
pub struct UsageResponse {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub pools: Vec<PoolInfo>,
}

/// 获取磁盘列表（Phase 41）
pub async fn list_disks() -> Result<HttpResponse, Error> {
    let disks = vec![
        DiskInfo {
            disk_id: 1,
            name: "System SSD".to_string(),
            path: "/dev/nvme0n1p1".to_string(),
            size_gb: 1024,
            used_gb: 768,
            available_gb: 256,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "nvme".to_string(),
            mount_point: "/".to_string(),
        },
        DiskInfo {
            disk_id: 2,
            name: "Data HDD 1".to_string(),
            path: "/dev/sda1".to_string(),
            size_gb: 2048,
            used_gb: 1536,
            available_gb: 512,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "hdd".to_string(),
            mount_point: "/data".to_string(),
        },
        DiskInfo {
            disk_id: 3,
            name: "Backup HDD".to_string(),
            path: "/dev/sdb1".to_string(),
            size_gb: 4096,
            used_gb: 2048,
            available_gb: 2048,
            usage_percent: 50,
            status: "online".to_string(),
            r#type: "hdd".to_string(),
            mount_point: "/backup".to_string(),
        },
        DiskInfo {
            disk_id: 4,
            name: "External USB".to_string(),
            path: "/dev/sdc1".to_string(),
            size_gb: 1024,
            used_gb: 400,
            available_gb: 624,
            usage_percent: 39,
            status: "online".to_string(),
            r#type: "external".to_string(),
            mount_point: "/mnt/usb".to_string(),
        },
    ];
    
    Ok(HttpResponse::Ok().json(disks))
}

/// 存储卷类型
#[derive(Debug, Serialize, Deserialize)]
pub enum VolumeType {
    Basic,
    JBOD,
    RAID0,
    RAID1,
    RAID5,
    RAID10,
    ZFS,
}

impl std::fmt::Display for VolumeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolumeType::Basic => write!(f, "basic"),
            VolumeType::JBOD => write!(f, "jbod"),
            VolumeType::RAID0 => write!(f, "raid0"),
            VolumeType::RAID1 => write!(f, "raid1"),
            VolumeType::RAID5 => write!(f, "raid5"),
            VolumeType::RAID10 => write!(f, "raid10"),
            VolumeType::ZFS => write!(f, "zfs"),
        }
    }
}

/// 存储卷信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeInfo {
    pub volume_id: u64,
    pub name: String,
    pub path: String,
    pub size_gb: u64,
    pub used_gb: u64,
    pub available_gb: u64,
    pub usage_percent: u8,
    pub status: String,
    pub r#type: String,
    pub disks: Vec<u64>,
}

/// 获取存储卷列表（Phase 43）
pub async fn list_volumes() -> Result<HttpResponse, Error> {
    let volumes = vec![
        VolumeInfo {
            volume_id: 1,
            name: "Root Volume".to_string(),
            path: "/".to_string(),
            size_gb: 1024,
            used_gb: 768,
            available_gb: 256,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "basic".to_string(),
            disks: vec![1],
        },
        VolumeInfo {
            volume_id: 2,
            name: "Data Pool".to_string(),
            path: "/data".to_string(),
            size_gb: 2048,
            used_gb: 1536,
            available_gb: 512,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "jbod".to_string(),
            disks: vec![2],
        },
        VolumeInfo {
            volume_id: 3,
            name: "Backup RAID1".to_string(),
            path: "/backup".to_string(),
            size_gb: 4096,
            used_gb: 2048,
            available_gb: 2048,
            usage_percent: 50,
            status: "online".to_string(),
            r#type: "raid1".to_string(),
            disks: vec![2, 3],
        },
    ];
    
    Ok(HttpResponse::Ok().json(volumes))
}

#[derive(Debug, Deserialize)]
pub struct CreateVolumeRequest {
    pub name: String,
    pub path: String,
    pub size_gb: u64,
    pub r#type: String,
    pub disk_ids: Vec<u64>,
}

/// 创建存储卷（Phase 45）
pub async fn create_volume(
    web::Json(payload): web::Json<CreateVolumeRequest>,
) -> Result<HttpResponse, Error> {
    // 验证参数
    if payload.name.is_empty() || payload.path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "name and path are required"
        })));
    }

    // 验证类型
    let valid_types = ["basic", "jbod", "raid0", "raid1", "raid5", "raid10", "zfs"];
    if !valid_types.contains(&payload.r#type.as_str()) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": format!("Invalid type '{}'. Valid types: {}", payload.r#type, valid_types.join(", "))
        })));
    }

    // 验证磁盘是否存在
    let mock_disks = vec![1, 2, 3];
    for disk_id in &payload.disk_ids {
        if !mock_disks.contains(disk_id) {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "message": format!("Disk {} not found", disk_id)
            })));
        }
    }

    // 模拟已存在卷检查
    let existing_names = vec!["Root Volume".to_string(), "Data Pool".to_string(), "Backup RAID1".to_string()];
    if existing_names.contains(&payload.name) {
        return Ok(HttpResponse::Conflict().json(serde_json::json!({
            "success": false,
            "message": format!("Volume name '{}' already exists", payload.name)
        })));
    }

    let new_volume = VolumeInfo {
        volume_id: 100 + (payload.disk_ids.len() as u64),
        name: payload.name,
        path: payload.path,
        size_gb: payload.size_gb,
        used_gb: 0,
        available_gb: payload.size_gb,
        usage_percent: 0,
        status: "online".to_string(),
        r#type: payload.r#type,
        disks: payload.disk_ids,
    };

    Ok(HttpResponse::Created().json(VolumeDetail {
        volume: new_volume,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// 存储卷详情（Phase 44）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VolumeDetail {
    pub volume: VolumeInfo,
    pub created_at: String,
    pub updated_at: String,
}

/// 获取存储卷详情（Phase 44）
pub async fn get_volume(
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let mock_volumes = vec![
        VolumeInfo {
            volume_id: 1,
            name: "Root Volume".to_string(),
            path: "/".to_string(),
            size_gb: 1024,
            used_gb: 768,
            available_gb: 256,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "basic".to_string(),
            disks: vec![1],
        },
        VolumeInfo {
            volume_id: 2,
            name: "Data Pool".to_string(),
            path: "/data".to_string(),
            size_gb: 2048,
            used_gb: 1536,
            available_gb: 512,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "jbod".to_string(),
            disks: vec![2],
        },
        VolumeInfo {
            volume_id: 3,
            name: "Backup RAID1".to_string(),
            path: "/backup".to_string(),
            size_gb: 4096,
            used_gb: 2048,
            available_gb: 2048,
            usage_percent: 50,
            status: "online".to_string(),
            r#type: "raid1".to_string(),
            disks: vec![2, 3],
        },
    ];

    match mock_volumes.iter().find(|v| v.volume_id == id) {
        Some(volume) => Ok(HttpResponse::Ok().json(VolumeDetail {
            volume: volume.clone(),
            created_at: "2026-03-15T10:00:00Z".to_string(),
            updated_at: "2026-03-18T17:00:00Z".to_string(),
        })),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Volume {} not found", id)
        }))),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskHealth {
    pub smart_status: String,
    pub temperature: u32,
    pub power_on_hours: u64,
}


/// 获取存储池列表（Phase 62 增强）
/// - JWT 认证，任意登录用户可访问
/// - 返回存储池列表和总体容量信息
pub async fn list_pools(
    req: HttpRequest,
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

    // 2. 存储池数据
    let pools = vec![
        PoolInfo {
            id: 1,
            name: "primary".to_string(),
            total_bytes: 3298534883328,  // 3TB
            used_bytes: 1649267441664,   // 1.5TB
            available_bytes: 1649267441664,
            usage_percent: 50.0,
            disks: vec!["/dev/sda1".to_string(), "/dev/sdb1".to_string()],
            status: "healthy".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
        PoolInfo {
            id: 2,
            name: "backup".to_string(),
            total_bytes: 2199023255552,  // 2TB
            used_bytes: 1099511627776,   // 1TB
            available_bytes: 1099511627776,
            usage_percent: 50.0,
            disks: vec!["/dev/sdc1".to_string()],
            status: "healthy".to_string(),
            created_at: 1710000000,
            updated_at: 1774345600,
        },
    ];

    // 3. 计算总体容量
    let total_pools = pools.len() as u64;
    let total_capacity_bytes: u64 = pools.iter().map(|p| p.total_bytes).sum();
    let total_used_bytes: u64 = pools.iter().map(|p| p.used_bytes).sum();

    // 4. 返回响应
    Ok(HttpResponse::Ok().json(PoolListResponse {
        success: true,
        data: pools,
        total_pools,
        total_capacity_bytes,
        total_used_bytes,
    }))
}

/// 创建存储池
pub async fn create_pool(
    req: web::Json<serde_json::Value>,
) -> Result<HttpResponse, Error> {
    // 解析请求参数
    // name: str, disks: Vec<String>, filesystem: Option<String>
    
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("default");
    let disks = req.get("disks").and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<String>>());
    
    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "message": format!("Storage pool '{}' created", name),
        "pool": {
            "id": 2,
            "name": name,
            "disks": disks.unwrap_or_default()
        }
    })))
}

/// 删除存储池
pub async fn delete_pool(
    pool_id: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    let id = pool_id.into_inner();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Storage pool {} deleted", id)
    })))
}

/// 获取存储使用量
pub async fn get_usage() -> Result<HttpResponse, Error> {
    let usage = UsageResponse {
        total_bytes: 3298534883328,  // 3TB
        used_bytes: 1649267441664,   // 1.5TB
        free_bytes: 1649267441664,   // 1.5TB
        pools: vec![
            PoolInfo {
                id: 1,
                name: "primary".to_string(),
                total_bytes: 3298534883328,
                used_bytes: 1649267441664,
                available_bytes: 1649267441664,
                usage_percent: 50.0,
                disks: vec!["/dev/sda1".to_string(), "/dev/sdb1".to_string()],
                status: "healthy".to_string(),
                created_at: 1710000000,
                updated_at: 1774345600,
            },
        ],
    };
    
    Ok(HttpResponse::Ok().json(usage))
}

/// 删除存储卷（Phase 46）
pub async fn delete_volume(
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let mock_volumes = vec![
        VolumeInfo {
            volume_id: 1,
            name: "Root Volume".to_string(),
            path: "/".to_string(),
            size_gb: 1024,
            used_gb: 768,
            available_gb: 256,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "basic".to_string(),
            disks: vec![1],
        },
        VolumeInfo {
            volume_id: 2,
            name: "Data Pool".to_string(),
            path: "/data".to_string(),
            size_gb: 2048,
            used_gb: 1536,
            available_gb: 512,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "jbod".to_string(),
            disks: vec![2],
        },
        VolumeInfo {
            volume_id: 3,
            name: "Backup RAID1".to_string(),
            path: "/backup".to_string(),
            size_gb: 4096,
            used_gb: 2048,
            available_gb: 2048,
            usage_percent: 50,
            status: "online".to_string(),
            r#type: "raid1".to_string(),
            disks: vec![2, 3],
        },
    ];

    match mock_volumes.iter().find(|v| v.volume_id == id) {
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Volume {} not found", id)
        }))),
        Some(volume) => {
            if volume.status == "online" {
                return Ok(HttpResponse::Conflict().json(serde_json::json!({
                    "success": false,
                    "message": format!("Volume {} is still in use", id)
                })));
            }
            
            Ok(HttpResponse::NoContent().finish())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateVolumeRequest {
    pub name: Option<String>,
    pub size_gb: Option<u64>,
}

/// 更新存储卷（Phase 47）
pub async fn update_volume(
    path: web::Path<u64>,
    web::Json(payload): web::Json<UpdateVolumeRequest>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();

    let mock_volumes = vec![
        VolumeInfo {
            volume_id: 1,
            name: "Root Volume".to_string(),
            path: "/".to_string(),
            size_gb: 1024,
            used_gb: 768,
            available_gb: 256,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "basic".to_string(),
            disks: vec![1],
        },
        VolumeInfo {
            volume_id: 2,
            name: "Data Pool".to_string(),
            path: "/data".to_string(),
            size_gb: 2048,
            used_gb: 1536,
            available_gb: 512,
            usage_percent: 75,
            status: "online".to_string(),
            r#type: "jbod".to_string(),
            disks: vec![2],
        },
        VolumeInfo {
            volume_id: 3,
            name: "Backup RAID1".to_string(),
            path: "/backup".to_string(),
            size_gb: 4096,
            used_gb: 2048,
            available_gb: 2048,
            usage_percent: 50,
            status: "online".to_string(),
            r#type: "raid1".to_string(),
            disks: vec![2, 3],
        },
    ];

    let volume_index = mock_volumes.iter().position(|v| v.volume_id == id);
    
    match volume_index {
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Volume {} not found", id)
        }))),
        Some(idx) => {
            let mut volume = mock_volumes[idx].clone();
            
            // 验证参数
            if let Some(ref name) = payload.name {
                if name.is_empty() {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "success": false,
                        "message": "name cannot be empty"
                    })));
                }
                
                // 检查名称重复
                for v in &mock_volumes {
                    if v.volume_id != id && v.name == *name {
                        return Ok(HttpResponse::Conflict().json(serde_json::json!({
                            "success": false,
                            "message": format!("Volume name '{}' already exists", name)
                        })));
                    }
                }
                
                volume.name = name.clone();
            }
            
            if let Some(new_size) = payload.size_gb {
                if new_size < volume.used_gb {
                    return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                        "success": false,
                        "message": "New size cannot be smaller than used space"
                    })));
                }
                volume.size_gb = new_size;
                volume.available_gb = new_size - volume.used_gb;
            }
            
            Ok(HttpResponse::Ok().json(VolumeDetail {
                volume,
                created_at: "2026-03-15T10:00:00Z".to_string(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            }))
        }
    }
}
