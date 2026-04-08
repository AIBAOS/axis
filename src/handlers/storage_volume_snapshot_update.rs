// Phase 86 - 更新存储卷快照 API
// PUT /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 更新存储卷快照

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 更新存储卷快照请求
#[derive(Deserialize)]
pub struct UpdateSnapshotRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_protected: Option<bool>,
}

/// 存储卷快照响应
#[derive(Serialize, Clone)]
pub struct SnapshotResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub volume_id: u64,
    pub volume_name: String,
    pub size_bytes: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: String,
    pub is_protected: bool,
    pub status: String,
}

/// 更新存储卷快照响应
#[derive(Serialize)]
pub struct UpdateSnapshotResponse {
    pub success: bool,
    pub message: String,
    pub data: SnapshotResponse,
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

/// 更新存储卷快照（Phase 86）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在
/// - 验证快照 ID 存在
/// - 验证快照名称唯一性（如果修改名称）
/// - 返回更新后的快照详情
pub async fn update_snapshot(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<(u64, u64)>,
    payload: web::Json<UpdateSnapshotRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update snapshots".to_string(),
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

    let (_, volume_name) = volume.expect("Volume should exist after check");

    // 4. 模拟该存储卷下的快照数据
    let mut mock_snapshots = vec![
        (1, "snapshot-2026-03-20".to_string(), "Initial snapshot".to_string(), 1099511627776u64, 1710892800, 1710892800, "admin".to_string(), false, "completed".to_string()),
        (2, "snapshot-2026-03-25".to_string(), "Before upgrade".to_string(), 1099511627776u64, 1711324800, 1711324800, "admin".to_string(), true, "completed".to_string()),
        (3, "snapshot-2026-03-26".to_string(), "Daily backup".to_string(), 1099511627776u64, 1711411200, 1711411200, "admin".to_string(), false, "completed".to_string()),
    ];

    // 5. 查找快照
    let snapshot_idx = mock_snapshots.iter().position(|(id, _, _, _, _, _, _, _, _)| *id == snapshot_id);
    if snapshot_idx.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Snapshot {} not found", snapshot_id),
            code: "SNAPSHOT_NOT_FOUND".to_string(),
        }));
    }

    let idx = snapshot_idx.expect("Snapshot index should exist");

    // 6. 提取旧值并转换为可修改的值
    let old_name = mock_snapshots[idx].1.clone();
    let old_description = mock_snapshots[idx].2.clone();
    let old_is_protected = mock_snapshots[idx].7;

    // 7. 更新字段
    let new_name = payload.name.clone().unwrap_or_else(|| old_name.clone());
    let new_description = payload.description.clone().or_else(|| Some(old_description.clone()));
    let new_is_protected = payload.is_protected.unwrap_or(old_is_protected);

    // 8. 模拟快照名称唯一性检查（如果名称改变）
    if payload.name.is_some() && &new_name != &old_name {
        let name_exists = mock_snapshots.iter().enumerate().any(|(i, (id, n, _, _, _, _, _, _, _))| i != idx && *id != snapshot_id && n == &new_name);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: "Snapshot name already exists".to_string(),
                code: "SNAPSHOT_EXISTS".to_string(),
            }));
        }
    }

    // 9. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 10. 更新快照数据（模拟）
    mock_snapshots[idx].1 = new_name.clone();
    if let Some(desc) = &new_description {
        mock_snapshots[idx].2 = desc.clone();
    }
    mock_snapshots[idx].7 = new_is_protected;
    mock_snapshots[idx].5 = now;

    // 11. 返回更新结果
    Ok(HttpResponse::Ok().json(UpdateSnapshotResponse {
        success: true,
        message: "Snapshot updated successfully".to_string(),
        data: SnapshotResponse {
            id: snapshot_id,
            name: new_name.clone(),
            description: new_description,
            volume_id,
            volume_name: volume_name.clone(),
            size_bytes: mock_snapshots[idx].3,
            created_at: mock_snapshots[idx].4,
            updated_at: now,
            created_by: mock_snapshots[idx].6.clone(),
            is_protected: new_is_protected,
            status: mock_snapshots[idx].8.clone(),
        },
    }))
}
