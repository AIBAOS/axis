// Phase 88 - 克隆存储卷快照 API
// POST /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/clone — 克隆存储卷快照

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 克隆快照请求
#[derive(Deserialize)]
pub struct CloneSnapshotRequest {
    pub new_volume_name: String,
    pub description: Option<String>,
    pub pool_id: Option<u64>,
}

/// 新卷信息
#[derive(Serialize)]
pub struct NewVolumeInfo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pool_id: u64,
    pool_name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub filesystem: String,
    pub status: String,
    pub mount_point: String,
    pub created_at: u64,
    pub created_from_snapshot: u64,
}

/// 克隆快照响应
#[derive(Serialize)]
pub struct CloneSnapshotResponse {
    pub success: bool,
    pub message: String,
    pub data: NewVolumeInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 克隆存储卷快照（Phase 88）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在性（404）
/// - 验证快照 ID 存在性（404）
/// - 验证快照归属（400）
/// - 验证快照状态（400）
/// - 验证新卷名称唯一性（409）
/// - 验证存储池容量（400）
/// - 克隆成功返回 201 Created
pub async fn clone_volume_snapshot(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
    payload: web::Json<CloneSnapshotRequest>,
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

    // 2. 权限校验 - 仅 admin 角色可克隆快照
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can clone snapshots".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let (volume_id, snapshot_id) = path.into_inner();
    let new_volume_name = &payload.new_volume_name;
    let description = &payload.description;
    let pool_id = payload.pool_id;

    // 3. 验证必要参数
    if new_volume_name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "new_volume_name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        serde_json::json!({"id": 1, "name": "System Volume", "pool_id": 1}),
        serde_json::json!({"id": 2, "name": "Data Volume", "pool_id": 2}),
        serde_json::json!({"id": 3, "name": "Backup Volume", "pool_id": 3}),
        serde_json::json!({"id": 4, "name": "Archive Volume", "pool_id": 4}),
    ];

    let volume = mock_volumes.into_iter().find(|v| v["id"] == volume_id);

    match volume {
        Some(v) => {
            let _volume_name = v["name"].as_str().unwrap().to_string();
            let source_pool_id = v["pool_id"].as_u64().unwrap();
            let target_pool_id = pool_id.unwrap_or(source_pool_id);

            // 5. 模拟该存储卷的快照数据（验证快照存在性和归属）
            let mock_snapshots = vec![
                serde_json::json!({"id": 1, "volume_id": 1, "name": "System Volume-snapshot-1", "status": "completed", "size_bytes": 100 * 1024 * 1024 * 1024}),
                serde_json::json!({"id": 2, "volume_id": 1, "name": "System Volume-snapshot-2", "status": "completed", "size_bytes": 100 * 1024 * 1024 * 1024}),
                serde_json::json!({"id": 3, "volume_id": 2, "name": "Data Volume-snapshot-1", "status": "completed", "size_bytes": 500 * 1024 * 1024 * 1024}),
                serde_json::json!({"id": 4, "volume_id": 3, "name": "Backup Volume-snapshot-1", "status": "creating", "size_bytes": 200 * 1024 * 1024 * 1024}),
            ];

            let snapshot = mock_snapshots.into_iter().find(|s| s["id"] == snapshot_id);

            match snapshot {
                Some(s) => {
                    // 6. 验证快照归属
                    let snapshot_volume_id = s["volume_id"].as_u64().unwrap();
                    if snapshot_volume_id != volume_id {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Snapshot {} does not belong to volume {}", snapshot_id, volume_id),
                            code: "SNAPSHOT_MISMATCH".to_string(),
                        }));
                    }

                    // 7. 验证快照状态（只有 completed 状态可克隆）
                    let snapshot_status = s["status"].as_str().unwrap();
                    if snapshot_status != "completed" {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Cannot clone snapshot: snapshot status is '{}'", snapshot_status),
                            code: "SNAPSHOT_NOT_READY".to_string(),
                        }));
                    }

                    let snapshot_size = s["size_bytes"].as_u64().unwrap();

                    // 8. 模拟新卷名称唯一性检查
                    let existing_volumes = vec!["System Volume", "Data Volume", "Backup Volume", "Archive Volume"];
                    if existing_volumes.contains(&new_volume_name.as_str()) {
                        return Ok(HttpResponse::Conflict().json(ErrorResponse {
                            success: false,
                            error: format!("Volume '{}' already exists", new_volume_name),
                            code: "NAME_CONFLICT".to_string(),
                        }));
                    }

                    // 9. 模拟存储池容量检查
                    let mock_pools = vec![
                        serde_json::json!({"id": 1, "name": "System Pool", "available_bytes": 250 * 1024 * 1024 * 1024}),
                        serde_json::json!({"id": 2, "name": "Data Pool", "available_bytes": 2400 * 1024 * 1024 * 1024}),
                        serde_json::json!({"id": 3, "name": "Backup Pool", "available_bytes": 7200 * 1024 * 1024 * 1024}),
                        serde_json::json!({"id": 4, "name": "Archive Pool", "available_bytes": 10000 * 1024 * 1024 * 1024}),
                    ];

                    let pool = mock_pools.into_iter().find(|p| p["id"] == target_pool_id);

                    match pool {
                        Some(p) => {
                            let pool_available = p["available_bytes"].as_u64().unwrap();
                            let pool_name = p["name"].as_str().unwrap().to_string();

                            if snapshot_size > pool_available {
                                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                                    success: false,
                                    error: format!("Insufficient pool capacity: requires {} bytes, available {} bytes", snapshot_size, pool_available),
                                    code: "INSUFFICIENT_CAPACITY".to_string(),
                                }));
                            }

                            // 10. 模拟克隆操作
                            let now = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                                .as_secs();

                            let new_volume = NewVolumeInfo {
                                id: 100,
                                name: new_volume_name.clone(),
                                description: description.clone(),
                                pool_id: target_pool_id,
                                pool_name,
                                size_bytes: snapshot_size,
                                used_bytes: snapshot_size,
                                available_bytes: 0,
                                usage_percent: 100.0,
                                filesystem: "ext4".to_string(),
                                status: "online".to_string(),
                                mount_point: format!("/mnt/{}", new_volume_name.to_lowercase().replace(" ", "_")),
                                created_at: now,
                                created_from_snapshot: snapshot_id,
                            };

                            Ok(HttpResponse::Created().json(CloneSnapshotResponse {
                                success: true,
                                message: "Snapshot cloned successfully".to_string(),
                                data: new_volume,
                            }))
                        }
                        None => {
                            Ok(HttpResponse::NotFound().json(ErrorResponse {
                                success: false,
                                error: format!("Storage pool {} not found", target_pool_id),
                                code: "POOL_NOT_FOUND".to_string(),
                            }))
                        }
                    }
                }
                None => {
                    // 11. 快照不存在
                    Ok(HttpResponse::NotFound().json(ErrorResponse {
                        success: false,
                        error: format!("Snapshot {} not found", snapshot_id),
                        code: "NOT_FOUND".to_string(),
                    }))
                }
            }
        }
        None => {
            // 12. 存储卷不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
