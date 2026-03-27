// Phase 65 - 存储池更新 API
// PUT /api/v1/storage/pools/{id} — 更新存储池信息

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 更新存储池请求
#[derive(Debug, Deserialize)]
pub struct UpdatePoolRequest {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<String>,
}

/// 更新存储池响应
#[derive(Debug, Serialize)]
pub struct UpdatePoolResponse {
    pub id: u64,
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub disk_count: u32,
    pub updated_at: u64,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub code: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 验证存储池类型
fn validate_pool_type(pool_type: &str) -> bool {
    let valid_types = ["basic", "raid0", "raid1", "raid5", "raid6", "raid10"];
    valid_types.contains(&pool_type)
}

/// 验证存储池状态
fn validate_pool_status(status: &str) -> bool {
    let valid_status = ["online", "degraded", "offline"];
    valid_status.contains(&status)
}

/// 更新存储池（Phase 65）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储池 ID 存在
/// - 返回更新后的存储池信息
pub async fn update_pool(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
    req: web::Json<UpdatePoolRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            message: "仅管理员可更新存储池信息".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let pool_id = path.into_inner();

    // 2. 模拟现有存储池数据（实际应从数据库读取）
    let mock_pools = vec![
        (1, "primary", "raid1", "online", 3298534883328u64, 1649267441664u64, 1649267441664u64, 50.0, 2),
        (2, "backup", "basic", "online", 2199023255552, 1099511627776, 1099511627776, 50.0, 1),
        (3, "media", "raid5", "online", 8796093022208, 4398046511104, 4398046511104, 50.0, 4),
    ];

    // 3. 查找存储池是否存在
    let existing_pool = mock_pools.iter().find(|(id, _, _, _, _, _, _, _, _)| *id == pool_id);

    if existing_pool.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            message: format!("存储池 {} 不存在", pool_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let (_, name, pool_type, status, total_bytes, used_bytes, available_bytes, usage_percent, disk_count) = existing_pool.unwrap();

    // 4. 准备更新字段
    let new_name = req.name.clone().unwrap_or_else(|| name.to_string());
    let new_type = req.r#type.clone().unwrap_or_else(|| pool_type.to_string());
    let new_status = req.status.clone().unwrap_or_else(|| status.to_string());

    // 5. 验证更新字段
    if !new_name.is_empty() && new_name.len() > 100 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            message: "存储池名称不能超过 100 个字符".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    if let Some(ref pool_type) = req.r#type {
        if !validate_pool_type(pool_type) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                message: "无效的存储池类型。有效类型：basic, raid0, raid1, raid5, raid6, raid10".to_string(),
                code: "INVALID_TYPE".to_string(),
            }));
        }
    }

    if let Some(ref status) = req.status {
        if !validate_pool_status(status) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                message: "无效的存储池状态。有效状态：online, degraded, offline".to_string(),
                code: "INVALID_STATUS".to_string(),
            }));
        }
    }

    // 6. 获取当前时间
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 7. 返回更新后的存储池信息
    let response = UpdatePoolResponse {
        id: pool_id,
        name: new_name,
        r#type: new_type,
        status: new_status,
        total_bytes: *total_bytes,
        used_bytes: *used_bytes,
        available_bytes: *available_bytes,
        usage_percent: *usage_percent,
        disk_count: *disk_count,
        updated_at: now,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "存储池信息更新成功",
        "data": response
    })))
}
