// Phase 69 - 删除存储卷 API
// DELETE /api/v1/storage/volumes/{id} — 删除存储卷

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 删除存储卷响应
#[derive(Debug, Serialize)]
pub struct DeleteVolumeResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeletedVolumeData>,
}

/// 已删除的存储卷数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeletedVolumeData {
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

/// 删除存储卷（Phase 69）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在
/// - 检查是否有数据/服务在使用该卷
/// - 删除成功返回 200 OK + 删除信息
pub async fn delete_volume(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let volume_id = path.into_inner();

    // 2. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        (1, "root".to_string(), false), // id, name, has_data
        (2, "data".to_string(), false),
        (3, "backup".to_string(), true), // 有数据在使用
    ];

    let volume = mock_volumes.iter().find(|(id, _, _)| *id == volume_id);

    // 3. 验证存储卷存在
    if volume.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", volume_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let (_, volume_name, has_data) = volume.expect("Volume should exist");

    // 4. 检查是否有数据/服务在使用该卷
    if *has_data {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Cannot delete storage volume '{}': data or services are using this volume", volume_name),
            code: "VOLUME_IN_USE".to_string(),
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
    Ok(HttpResponse::Ok().json(DeleteVolumeResponse {
        success: true,
        message: "Storage volume deleted successfully".to_string(),
        data: Some(DeletedVolumeData {
            id: volume_id,
            name: volume_name.clone(),
            deleted_at: now,
        }),
    }))
}
