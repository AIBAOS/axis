// Phase 83 - 存储卷快照列表 API（增强版）
// GET /api/v1/storage/volumes/{volume_id}/snapshots — 获取存储卷的快照列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

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

/// 分页查询参数
#[derive(Deserialize)]
pub struct SnapshotsQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub status: Option<String>,
    pub is_protected: Option<bool>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub total_count: u64,
    pub limit: u32,
    pub offset: u32,
}

/// 快照列表响应
#[derive(Serialize)]
pub struct SnapshotListResponse {
    pub success: bool,
    pub data: Vec<SnapshotInfo>,
    pub pagination: PaginationMeta,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取存储卷快照列表（Phase 83）
/// - JWT 认证，登录用户可访问
/// - 验证存储卷存在性（404 Not Found）
/// - 支持分页：limit, offset
/// - 支持筛选：status, is_protected
/// - 无快照返回空数组
pub async fn list_volume_snapshots(
    req: HttpRequest,
    path: web::Path<u64>,
    query: web::Query<SnapshotsQuery>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
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

    let volume_id = path.into_inner();

    // 3. 解析查询参数
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let status_filter = query.status.as_deref();
    let is_protected_filter = query.is_protected;

    // 4. 模拟存储卷数据（验证存在性）
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

            // 5. 模拟该存储卷的快照数据
            let all_snapshots = vec![
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
                SnapshotInfo {
                    id: 4,
                    name: format!("{}-snapshot-4", volume_name),
                    description: Some("Failed backup".to_string()),
                    volume_id,
                    volume_name: volume_name.clone(),
                    size_bytes: 100 * 1024 * 1024 * 1024,
                    created_at: 1712131200,
                    created_by: "admin".to_string(),
                    is_protected: false,
                    status: "failed".to_string(),
                },
            ];

            // 6. 应用筛选
            let mut filtered_snapshots = all_snapshots;

            if let Some(status) = status_filter {
                filtered_snapshots.retain(|s| s.status == status);
            }

            if let Some(is_protected) = is_protected_filter {
                filtered_snapshots.retain(|s| s.is_protected == is_protected);
            }

            let total_count = filtered_snapshots.len() as u64;
            let start = offset as usize;
            let end = (offset + limit) as usize;

            // 7. 分页
            let snapshots: Vec<SnapshotInfo> = filtered_snapshots
                .into_iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    if i >= start && i < end {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect();

            Ok(HttpResponse::Ok().json(SnapshotListResponse {
                success: true,
                data: snapshots,
                pagination: PaginationMeta {
                    total_count,
                    limit,
                    offset,
                },
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
