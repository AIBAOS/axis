// Phase 66 - 删除存储池 API
// DELETE /api/v1/storage/pools/{id} — 删除存储池

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 删除存储池响应
#[derive(Debug, Serialize)]
pub struct DeletePoolResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeletedPoolData>,
}

/// 已删除的存储池数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeletedPoolData {
    pub id: u64,
    pub name: String,
    pub deleted_at: u64,
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
    claims.is_admin()
}

/// 删除存储池（Phase 66）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储池 ID 存在
/// - 检查是否有卷在使用该池
/// - 删除成功返回 200 OK + 删除信息
pub async fn delete_pool(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete storage pools".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let pool_id = path.into_inner();

    // 2. 模拟存储池数据（验证存在性）
    let mock_pools = vec![
        (1, "primary".to_string(), false), // id, name, has_volumes
        (2, "backup".to_string(), false),
        (3, "media".to_string(), true),    // 有卷在使用
    ];

    let pool = mock_pools.iter().find(|(id, _, _)| *id == pool_id);

    // 3. 验证存储池存在
    if pool.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage pool {} not found", pool_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let (_, pool_name, has_volumes) = pool.expect("Pool should exist");

    // 4. 检查是否有卷在使用该池
    if *has_volumes {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Cannot delete storage pool '{}': volumes are using this pool", pool_name),
            code: "POOL_IN_USE".to_string(),
        }));
    }

    // 5. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 6. 执行删除（模拟）
    // 实际应调用数据库删除操作

    // 7. 返回删除结果
    Ok(HttpResponse::Ok().json(DeletePoolResponse {
        success: true,
        message: "Storage pool deleted successfully".to_string(),
        data: Some(DeletedPoolData {
            id: pool_id,
            name: pool_name.clone(),
            deleted_at: now,
        }),
    }))
}
