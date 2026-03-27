// Phase 158: NFS 共享更新 API
// PUT /api/v1/shares/nfs/{id} — 更新 NFS 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// NFS 客户端配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NfsClientConfig {
    pub network: String,
    pub access: String,
}

/// 更新 NFS 共享请求
#[derive(Debug, Deserialize)]
pub struct UpdateNfsShareRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub comment: Option<String>,
    pub read_only: Option<bool>,
    pub no_subtree_check: Option<bool>,
    pub sync: Option<bool>,
    pub clients: Option<Vec<NfsClientConfig>>,
}

/// NFS 共享信息
#[derive(Serialize, Clone)]
pub struct NfsShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub comment: String,
    pub read_only: bool,
    pub no_subtree_check: bool,
    pub sync: bool,
    pub clients: Vec<NfsClientConfig>,
    pub enabled: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 更新 NFS 共享响应
#[derive(Serialize)]
pub struct UpdateNfsShareResponse {
    pub success: bool,
    pub message: String,
    pub data: NfsShareInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证共享名称格式
fn validate_share_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 64 && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// 验证共享路径格式
fn validate_share_path(path: &str) -> bool {
    path.starts_with('/') && path.len() <= 256
}

/// 验证客户端网络格式（CIDR）
fn validate_network(network: &str) -> bool {
    network.contains('/') && network.len() <= 64
}

/// 验证访问权限
fn validate_access(access: &str) -> bool {
    access == "ro" || access == "rw"
}

/// 更新 NFS 共享（Phase 158）
/// - JWT 认证，仅 admin 角色可访问
/// - 请求体包含：name/path/comment/read_only/no_subtree_check/sync/clients（可选，部分更新）
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 验证客户端配置（400 Bad Request）
/// - 验证名称唯一性（409 Conflict，排除自身）
/// - 更新成功返回 200 OK + 共享详情
pub async fn update_nfs_share(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateNfsShareRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let share_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update NFS shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证名称格式（如果提供）
    if let Some(ref name) = payload.name {
        if !validate_share_name(name) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed".to_string(),
                code: "INVALID_NAME".to_string(),
            }));
        }
    }

    // 5. 验证路径格式（如果提供）
    if let Some(ref path) = payload.path {
        if !validate_share_path(path) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid share path. Must start with / and be <= 256 chars".to_string(),
                code: "INVALID_PATH".to_string(),
            }));
        }
    }

    // 6. 验证客户端配置（如果提供）
    if let Some(ref clients) = payload.clients {
        if clients.is_empty() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "At least one client configuration is required".to_string(),
                code: "INVALID_CLIENTS".to_string(),
            }));
        }
        for client in clients {
            if !validate_network(&client.network) {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Invalid client network '{}'. Must be CIDR format (e.g., 192.168.1.0/24)", client.network),
                    code: "INVALID_NETWORK".to_string(),
                }));
            }
            if !validate_access(&client.access) {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Invalid client access '{}'. Must be 'ro' or 'rw'", client.access),
                    code: "INVALID_ACCESS".to_string(),
                }));
            }
        }
    }

    // 7. 模拟现有 NFS 共享数据
    let mut mock_shares = vec![
        NfsShareInfo {
            id: 1,
            name: "Data".to_string(),
            path: "/srv/nfs/data".to_string(),
            comment: "Data shared folder".to_string(),
            read_only: false,
            no_subtree_check: true,
            sync: true,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.1.0/24".to_string(),
                    access: "rw".to_string(),
                },
            ],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareInfo {
            id: 2,
            name: "Backup".to_string(),
            path: "/srv/nfs/backup".to_string(),
            comment: "Backup shared folder".to_string(),
            read_only: true,
            no_subtree_check: true,
            sync: true,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.1.0/24".to_string(),
                    access: "ro".to_string(),
                },
            ],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareInfo {
            id: 3,
            name: "Media".to_string(),
            path: "/srv/nfs/media".to_string(),
            comment: "Media shared folder".to_string(),
            read_only: true,
            no_subtree_check: true,
            sync: false,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.0.0/16".to_string(),
                    access: "ro".to_string(),
                },
            ],
            enabled: false,
            status: "inactive".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 8. 查找共享
    let share_index = mock_shares.iter().position(|s| s.id == share_id);

    // 9. 验证共享存在性
    if share_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("NFS share {} not found", share_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let share_index = share_index.unwrap();

    // 10. 验证名称唯一性（排除自身）
    if let Some(ref new_name) = payload.name {
        let name_exists = mock_shares.iter().any(|s| s.id != share_id && s.name == *new_name);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("NFS share name '{}' already exists", new_name),
                code: "NAME_CONFLICT".to_string(),
            }));
        }
    }

    // 11. 更新共享配置（部分更新）
    let share = &mut mock_shares[share_index];
    
    if let Some(new_name) = &payload.name {
        share.name = new_name.clone();
    }
    if let Some(new_path) = &payload.path {
        share.path = new_path.clone();
    }
    if let Some(ref new_comment) = payload.comment {
        share.comment = new_comment.clone();
    }
    if let Some(new_read_only) = payload.read_only {
        share.read_only = new_read_only;
    }
    if let Some(new_no_subtree_check) = payload.no_subtree_check {
        share.no_subtree_check = new_no_subtree_check;
    }
    if let Some(new_sync) = payload.sync {
        share.sync = new_sync;
    }
    if let Some(ref new_clients) = payload.clients {
        share.clients = new_clients.clone();
    }

    // 12. 更新时间戳
    let now = chrono::Utc::now().to_rfc3339();
    share.updated_at = now.clone();

    // 13. 返回更新成功
    Ok(HttpResponse::Ok().json(UpdateNfsShareResponse {
        success: true,
        message: "NFS share updated successfully".to_string(),
        data: share.clone(),
    }))
}
