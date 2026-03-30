// Phase 82 - 创建存储卷快照 API
// POST /api/v1/storage/volumes/{id}/snapshots — 创建存储卷快照

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 创建快照请求
#[derive(Deserialize)]
pub struct CreateSnapshotRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_protected: Option<bool>,
}

/// 快照响应
#[derive(Serialize)]
pub struct SnapshotResponse {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub volume_id: u64,
    pub volume_name: String,
    pub size_bytes: u64,
    pub created_at: u64,
    pub created_by: String,
    pub is_protected: bool,
}

/// 创建快照响应
#[derive(Serialize)]
pub struct CreateSnapshotResponse {
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

/// 创建存储卷快照（Phase 82）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷存在性（404）
/// - 验证快照名称唯一性（409）
/// - 创建成功返回 201 Created
pub async fn create_volume_snapshot(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<CreateSnapshotRequest>,
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

    // 2. 权限校验 - 仅 admin 角色可创建快照
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create snapshots".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let volume_id = path.into_inner();
    let snapshot_name = &payload.name;
    let description = &payload.description;
    let is_protected = payload.is_protected.unwrap_or(false);

    // 3. 验证必要参数
    if snapshot_name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟查找存储卷（验证存在性）
    let mock_volumes = vec![
        serde_json::json!({"id": 1, "name": "System Volume", "status": "online"}),
        serde_json::json!({"id": 2, "name": "Data Volume", "status": "online"}),
        serde_json::json!({"id": 3, "name": "Backup Volume", "status": "online"}),
        serde_json::json!({"id": 4, "name": "Archive Volume", "status": "offline"}),
    ];

    let volume = mock_volumes.into_iter().find(|v| v["id"] == volume_id);

    match volume {
        Some(v) => {
            // 5. 验证存储卷状态允许创建快照
            let volume_status = v["status"].as_str().unwrap_or("unknown");
            if volume_status != "online" {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot create snapshot: volume status is '{}'", volume_status),
                    code: "VOLUME_NOT_READY".to_string(),
                }));
            }

            let volume_name = v["name"].as_str().unwrap_or("unknown").to_string();

            // 6. 模拟快照名称唯一性检查
            let existing_snapshots = vec![
                "System Volume-snapshot-1",
                "System Volume-snapshot-2",
                "Data Volume-snapshot-1",
            ];
            if existing_snapshots.contains(&snapshot_name.as_str()) {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: format!("Snapshot '{}' already exists", snapshot_name),
                    code: "NAME_CONFLICT".to_string(),
                }));
            }

            // 7. 模拟创建快照
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();

            // 模拟生成的快照 ID
            let new_id = 100u64;

            // 模拟快照大小（与存储卷已用空间相同）
            let snapshot_size = 100 * 1024 * 1024 * 1024; // 100GB

            let snapshot = SnapshotResponse {
                id: new_id,
                name: snapshot_name.clone(),
                description: description.clone(),
                volume_id,
                volume_name,
                size_bytes: snapshot_size,
                created_at: now,
                created_by: "admin".to_string(),
                is_protected,
            };

            Ok(HttpResponse::Created().json(CreateSnapshotResponse {
                success: true,
                message: "Snapshot created successfully".to_string(),
                data: snapshot,
            }))
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
