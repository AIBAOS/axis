// Phase 85 - 删除存储卷快照 API
// DELETE /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 删除存储卷快照

use actix_web::{web, HttpResponse, Error};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 删除快照响应
#[derive(Serialize)]
pub struct DeleteSnapshotResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeletedSnapshotData>,
}

/// 已删除的快照数据
#[derive(Serialize)]
pub struct DeletedSnapshotData {
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

/// 删除存储卷快照（Phase 85）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在
/// - 验证快照 ID 存在
/// - 检查快照保护状态
/// - 返回删除结果
pub async fn delete_snapshot(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<(u64, u64)>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete snapshots".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let (volume_id, snapshot_id) = path.into_inner();

    // 2. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        (1, "root".to_string()),
        (2, "data".to_string()),
        (3, "backup".to_string()),
    ];

    let volume = mock_volumes.iter().find(|(id, _)| *id == volume_id);

    // 3. 验证存储卷存在
    if volume.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", volume_id),
            code: "VOLUME_NOT_FOUND".to_string(),
        }));
    }

    // 4. 模拟该存储卷下的快照数据
    let mock_snapshots = vec![
        (1, "snapshot-2026-03-20".to_string(), false),  // id, name, is_protected
        (2, "snapshot-2026-03-25".to_string(), true),   // 受保护的快照
        (3, "snapshot-2026-03-26".to_string(), false),
    ];

    let snapshot = mock_snapshots.iter().find(|(id, _, _)| *id == snapshot_id);

    // 5. 验证快照存在
    if snapshot.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Snapshot {} not found", snapshot_id),
            code: "SNAPSHOT_NOT_FOUND".to_string(),
        }));
    }

    let (_, snapshot_name, is_protected) = .expect("Snapshot should exist");

    // 6. 检查快照保护状态
    if *is_protected {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Snapshot '{}' is protected and cannot be deleted", snapshot_name),
            code: "SNAPSHOT_PROTECTED".to_string(),
        }));
    }

    // 7. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 8. 执行删除（模拟）
    // 实际应调用数据库删除操作

    // 9. 返回删除结果
    Ok(HttpResponse::Ok().json(DeleteSnapshotResponse {
        success: true,
        message: "Snapshot deleted successfully".to_string(),
        data: Some(DeletedSnapshotData {
            id: snapshot_id,
            name: snapshot_name.clone(),
            deleted_at: now,
        }),
    }))
}
