// Phase 80 - 存储卷创建 API
// POST /api/v1/storage/volumes — 创建存储卷

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 创建存储卷请求
#[derive(Deserialize)]
pub struct CreateVolumeRequest {
    pub name: String,
    pub description: Option<String>,
    pub pool_id: u64,
    pub size_bytes: u64,
    pub filesystem: String,
}

/// 创建存储卷响应
#[derive(Serialize)]
pub struct CreateVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<VolumeData>,
}

/// 存储卷数据
#[derive(Serialize)]
pub struct VolumeData {
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

/// 验证文件系统类型
fn validate_filesystem(fs_type: &str) -> bool {
    let valid_types = ["ext4", "btrfs", "xfs", "zfs"];
    valid_types.contains(&fs_type.to_lowercase().as_str())
}

/// 创建存储卷（Phase 80）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证必要参数和存储池存在性
/// - 检查存储池容量是否足够
/// - 检查名称唯一性
/// - 返回创建的存储卷信息
pub async fn create_volume(
    jwt_claims: web::Data<JwtClaims>,
    req: web::Json<CreateVolumeRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 2. 验证必要参数
    if req.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // Bug #47 修复：添加卷名长度验证 (1-64 字符)
    if req.name.len() > 64 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name must be 64 characters or less".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // Bug #47 修复：验证卷名字符（只允许字母、数字、下划线、连字符）
    if !req.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name can only contain letters, numbers, underscores and hyphens".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if req.size_bytes == 0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "size_bytes must be greater than 0".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 验证文件系统类型
    if !validate_filesystem(&req.filesystem) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Invalid filesystem '{}'. Valid types: ext4, btrfs, xfs, zfs", req.filesystem),
            code: "INVALID_FILESYSTEM".to_string(),
        }));
    }

    // 4. 模拟存储池数据（验证存储池存在性）
    let mock_pools = vec![
        (1, "primary".to_string(), 8796093022208u64, 4398046511104u64), // id, name, total, used
        (2, "backup".to_string(), 4398046511104u64, 2199023255552u64),
    ];

    let pool = mock_pools.iter().find(|(id, _, _, _)| *id == req.pool_id);

    // 5. 验证存储池存在
    let pool = match pool {
        Some(p) => p,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage pool {} not found", req.pool_id),
                code: "POOL_NOT_FOUND".to_string(),
            }));
        }
    };

    let (_, _pool_name, _, pool_used) = pool;
    let pool_available = 8796093022208u64 - *pool_used; // 简化计算

    // 6. 检查存储池容量是否足够
    if req.size_bytes > pool_available {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Insufficient space in storage pool. Available: {} bytes, Requested: {} bytes", pool_available, req.size_bytes),
            code: "INSUFFICIENT_SPACE".to_string(),
        }));
    }

    // 7. 检查名称是否已存在（模拟）
    let existing_names = vec!["root".to_string(), "data".to_string(), "backup".to_string()];
    if existing_names.contains(&req.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Storage volume name '{}' already exists", req.name),
            code: "CONFLICT".to_string(),
        }));
    }

    // 8. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 9. 生成挂载点
    let mount_point = format!("/mnt/volumes/{}", req.name);

    // 10. 创建存储卷（模拟）
    let new_volume = VolumeData {
        id: 100 + req.pool_id,
        name: req.name.clone(),
        description: req.description.clone(),
        pool_id: req.pool_id,
        total_bytes: req.size_bytes,
        used_bytes: 0,
        available_bytes: req.size_bytes,
        usage_percent: 0.0,
        status: "online".to_string(),
        filesystem: req.filesystem.to_lowercase(),
        mount_point,
        created_at: now,
        updated_at: now,
    };

    // 11. 返回创建结果
    Ok(HttpResponse::Created().json(CreateVolumeResponse {
        success: true,
        message: "Storage volume created successfully".to_string(),
        data: Some(new_volume),
    }))
}
