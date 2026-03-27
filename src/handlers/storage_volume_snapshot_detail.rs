// Phase 84 - 存储卷快照详情 API
// GET /api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id} — 获取存储卷快照详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 快照详情响应
#[derive(Serialize)]
pub struct SnapshotDetailResponse {
    pub success: bool,
    pub data: SnapshotInfo,
}

/// 快照信息
#[derive(Serialize)]
pub struct SnapshotInfo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub volume_id: u64,
    pub volume_name: String,
    pub size_bytes: u64,
    pub created_at: u64,
    pub created_by: String,
    pub is_protected: bool,
    pub status: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储卷快照详情（Phase 84）
/// - JWT 认证，登录用户可访问
/// - 验证存储卷 ID 存在性（404 Not Found）
/// - 验证快照 ID 存在性（404 Not Found）
/// - 返回快照详细信息
pub async fn get_volume_snapshot(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（登录用户）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 登录用户可访问（无需 admin）
    // 已通过 JWT 验证，说明是登录用户

    let (volume_id, snapshot_id) = path.into_inner();

    // 3. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        serde_json::json!({"id": 1, "name": "System Volume"}),
        serde_json::json!({"id": 2, "name": "Data Volume"}),
        serde_json::json!({"id": 3, "name": "Backup Volume"}),
        serde_json::json!({"id": 4, "name": "Archive Volume"}),
    ];

    let volume = mock_volumes.into_iter().find(|v| v["id"] == volume_id);

    match volume {
        Some(v) => {
            let volume_name = v["name"].as_str().unwrap().to_string();

            // 4. 模拟该存储卷的快照数据
            let mock_snapshots = vec![
                SnapshotInfo {
                    id: 1,
                    name: format!("{}-snapshot-1", volume_name),
                    description: Some("Initial backup".to_string()),
                    volume_id,
                    volume_name: volume_name.clone(),
                    size_bytes: 100 * 1024 * 1024 * 1024, // 100GB
                    created_at: 1710489600,
                    created_by: "admin".to_string(),
                    is_protected: false,
                    status: "completed".to_string(),
                },
                SnapshotInfo {
                    id: 2,
                    name: format!("{}-snapshot-2", volume_name),
                    description: Some("Before update".to_string()),
                    volume_id,
                    volume_name: volume_name.clone(),
                    size_bytes: 100 * 1024 * 1024 * 1024,
                    created_at: 1711440000,
                    created_by: "admin".to_string(),
                    is_protected: true,
                    status: "completed".to_string(),
                },
                SnapshotInfo {
                    id: 3,
                    name: format!("{}-snapshot-3", volume_name),
                    description: None,
                    volume_id,
                    volume_name: volume_name.clone(),
                    size_bytes: 100 * 1024 * 1024 * 1024,
                    created_at: 1712044800,
                    created_by: "user1".to_string(),
                    is_protected: false,
                    status: "creating".to_string(),
                },
            ];

            // 5. 查找快照
            let snapshot = mock_snapshots.into_iter().find(|s| s.id == snapshot_id);

            match snapshot {
                Some(s) => {
                    // 6. 返回快照详情
                    Ok(HttpResponse::Ok().json(SnapshotDetailResponse {
                        success: true,
                        data: s,
                    }))
                }
                None => {
                    // 7. 快照不存在
                    Ok(HttpResponse::NotFound().json(ErrorResponse {
                        success: false,
                        error: format!("Snapshot {} not found", snapshot_id),
                        code: "NOT_FOUND".to_string(),
                    }))
                }
            }
        }
        None => {
            // 8. 存储卷不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
