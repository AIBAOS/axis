// Phase 154: NFS 共享创建 API
// POST /api/v1/shares/nfs — 创建 NFS 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// NFS 客户端配置
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NfsClientConfig {
    pub network: String,
    pub access: String,
}

/// 创建 NFS 共享请求
#[derive(Debug, Deserialize)]
pub struct CreateNfsShareRequest {
    pub name: String,
    pub path: String,
    pub comment: Option<String>,
    pub read_only: Option<bool>,
    pub no_subtree_check: Option<bool>,
    pub sync: Option<bool>,
    pub clients: Vec<NfsClientConfig>,
}

/// 创建的 NFS 共享信息
#[derive(Serialize, Clone)]
pub struct CreatedNfsShare {
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

/// 创建 NFS 共享响应
#[derive(Serialize)]
pub struct CreateNfsShareResponse {
    pub success: bool,
    pub message: String,
    pub data: CreatedNfsShare,
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

/// 创建 NFS 共享（Phase 154）
/// - JWT 认证，仅 admin 角色可访问
/// - 请求体包含：name/path/comment/read_only/no_subtree_check/sync/clients
/// - 验证名称唯一性（409 Conflict）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 验证客户端配置（400 Bad Request）
/// - 创建成功返回 201 Created + 共享详情
pub async fn create_nfs_share(
    req: HttpRequest,
    payload: web::Json<CreateNfsShareRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
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
            error: "Only admin users can create NFS shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证共享名称格式
    if !validate_share_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid share name. Must be 1-64 chars, alphanumeric with -_. allowed".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证共享路径格式
    if !validate_share_path(&payload.path) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid share path. Must start with / and be <= 256 chars".to_string(),
            code: "INVALID_PATH".to_string(),
        }));
    }

    // 6. 验证客户端配置
    if payload.clients.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one client configuration is required".to_string(),
            code: "INVALID_CLIENTS".to_string(),
        }));
    }

    for client in &payload.clients {
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

    // 7. 模拟现有共享数据（用于名称唯一性检查）
    let existing_shares = vec!["Data", "Backup", "Media"];

    // 8. 验证名称唯一性
    if existing_shares.iter().any(|n| n == &payload.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("NFS share name '{}' already exists", payload.name),
            code: "NAME_CONFLICT".to_string(),
        }));
    }

    // 9. 模拟创建 NFS 共享
    let now = chrono::Utc::now().to_rfc3339();
    let new_share = CreatedNfsShare {
        id: 4, // 模拟自增 ID
        name: payload.name.clone(),
        path: payload.path.clone(),
        comment: payload.comment.clone().unwrap_or_default(),
        read_only: payload.read_only.unwrap_or(false),
        no_subtree_check: payload.no_subtree_check.unwrap_or(true),
        sync: payload.sync.unwrap_or(true),
        clients: payload.clients.clone(),
        enabled: true,
        status: "active".to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    // 10. 返回创建成功
    Ok(HttpResponse::Created().json(CreateNfsShareResponse {
        success: true,
        message: "NFS share created successfully".to_string(),
        data: new_share,
    }))
}
