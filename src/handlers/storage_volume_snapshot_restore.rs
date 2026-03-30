// Phase 87 - 恢复存储卷快照 API
// POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/restore — 恢复存储卷快照

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 恢复快照响应
#[derive(Serialize)]
pub struct RestoreSnapshotResponse {
    pub success: bool,
    pub message: String,
    pub data: RestoreInfo,
}

/// 恢复信息
#[derive(Serialize)]
pub struct RestoreInfo {
    pub source_snapshot_id: u64,
    pub target_volume_id: u64,
    pub target_volume_name: String,
    pub restored_at: u64,
    pub status: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 恢复存储卷快照（Phase 87）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在性（404）
/// - 验证快照 ID 存在性（404）
/// - 验证快照属于该存储卷（400）
/// - 检查存储卷是否正在使用/挂载（400）
/// - 恢复成功返回 200 OK
pub async fn restore_volume_snapshot(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 仅 admin 角色可恢复快照
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can restore snapshots".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let (volume_id, snapshot_id) = path.into_inner();

    // 3. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        serde_json::json!({"id": 1, "name": "System Volume", "is_mounted": false, "status": "online"}),
        serde_json::json!({"id": 2, "name": "Data Volume", "is_mounted": true, "status": "online"}),
        serde_json::json!({"id": 3, "name": "Backup Volume", "is_mounted": false, "status": "online"}),
        serde_json::json!({"id": 4, "name": "Archive Volume", "is_mounted": false, "status": "offline"}),
    ];

    let volume = mock_volumes.into_iter().find(|v| v["id"] == volume_id);

    match volume {
        Some(v) => {
            let volume_name = v["name"].as_str().unwrap_or("unknown").to_string();
            let is_mounted = v["is_mounted"].as_bool().unwrap_or(false);
            let volume_status = v["status"].as_str().unwrap_or("unknown");

            // 4. 检查存储卷是否正在使用/挂载
            if is_mounted {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot restore snapshot: volume '{}' is currently mounted/in use", volume_name),
                    code: "VOLUME_IN_USE".to_string(),
                }));
            }

            // 5. 检查存储卷状态
            if volume_status != "online" {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot restore snapshot: volume status is '{}'", volume_status),
                    code: "VOLUME_NOT_READY".to_string(),
                }));
            }

            // 6. 模拟该存储卷的快照数据（验证快照存在性和归属）
            let mock_snapshots = vec![
                serde_json::json!({"id": 1, "volume_id": 1, "name": "System Volume-snapshot-1", "status": "completed"}),
                serde_json::json!({"id": 2, "volume_id": 1, "name": "System Volume-snapshot-2", "status": "completed"}),
                serde_json::json!({"id": 3, "volume_id": 2, "name": "Data Volume-snapshot-1", "status": "completed"}),
                serde_json::json!({"id": 4, "volume_id": 3, "name": "Backup Volume-snapshot-1", "status": "completed"}),
            ];

            let snapshot = mock_snapshots.into_iter().find(|s| s["id"] == snapshot_id);

            match snapshot {
                Some(s) => {
                    // 7. 验证快照属于该存储卷
                    let snapshot_volume_id = s["volume_id"].as_u64().unwrap_or(0);
                    if snapshot_volume_id != volume_id {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Snapshot {} does not belong to volume {}", snapshot_id, volume_id),
                            code: "SNAPSHOT_MISMATCH".to_string(),
                        }));
                    }

                    // 8. 检查快照状态（只有 completed 状态的快照可以恢复）
                    let snapshot_status = s["status"].as_str().unwrap_or("unknown");
                    if snapshot_status != "completed" {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Cannot restore snapshot: snapshot status is '{}'", snapshot_status),
                            code: "SNAPSHOT_NOT_READY".to_string(),
                        }));
                    }

                    // 9. 模拟执行恢复操作
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                        .as_secs();

                    let restore_info = RestoreInfo {
                        source_snapshot_id: snapshot_id,
                        target_volume_id: volume_id,
                        target_volume_name: volume_name,
                        restored_at: now,
                        status: "completed".to_string(),
                    };

                    Ok(HttpResponse::Ok().json(RestoreSnapshotResponse {
                        success: true,
                        message: "Snapshot restored successfully".to_string(),
                        data: restore_info,
                    }))
                }
                None => {
                    // 10. 快照不存在
                    Ok(HttpResponse::NotFound().json(ErrorResponse {
                        success: false,
                        error: format!("Snapshot {} not found", snapshot_id),
                        code: "NOT_FOUND".to_string(),
                    }))
                }
            }
        }
        None => {
            // 11. 存储卷不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
