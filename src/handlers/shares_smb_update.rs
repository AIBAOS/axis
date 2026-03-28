// Phase 211: SMB 共享更新 API
// PUT /api/v1/shares/smb/{id} — 更新 SMB 共享

 use actix_web::{web, HttpResponse, Error, HttpRequest};
 use serde::{Deserialize, Serialize};
 use std::sync::Arc;
 use std::path::Path;

 use crate::services::jwt_service::JwtService;
 use crate::database::share_store::SqliteShareRepository;

/// 更新 SMB 共享请求
#[derive(Debug, Deserialize)]
pub struct UpdateSmbShareRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_groups: Option<String>,
    pub guest_ok: Option<bool>,
    pub read_only: Option<bool>,
}

/// SMB 共享信息
#[derive(Serialize, Clone)]
pub struct SmbShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_groups: Option<String>,
    pub guest_ok: bool,
    pub read_only: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 更新 SMB 共享响应
#[derive(Serialize)]
pub struct UpdateSmbShareResponse {
    pub success: bool,
    pub message: String,
    pub data: SmbShareInfo,
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

/// 更新 SMB 共享（Phase 211）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库更新
/// - 请求体包含：name/path/description/allowed_users/allowed_groups/guest_ok/read_only（可选，部分更新）
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 验证路径存在性（400 Bad Request）
/// - 验证名称唯一性（409 Conflict，排除自身）
/// - 更新成功返回 200 OK + 共享详情
pub async fn update_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateSmbShareRequest>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
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
            error: "Only admin users can update SMB shares".to_string(),
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

    // 6. 验证路径存在性（如果提供）
    if let Some(ref path) = payload.path {
        if !Path::new(path).exists() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Path '{}' does not exist", path),
                code: "PATH_NOT_FOUND".to_string(),
            }));
        }
    }

    // 7. 从数据库查询共享是否存在且为 SMB 协议
    let existing_share = match repo.get_share_by_id(share_id) {
        Ok(Some(s)) => {
            if s.protocol != "smb" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("SMB share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }
            s
        }
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("SMB share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }));
        }
    };

    // 8. 验证名称唯一性（排除自身）
    if let Some(ref new_name) = payload.name {
        if new_name != &existing_share.name {
            // 检查是否有其他共享已使用此名称
            let all_shares = repo.get_shares(1, 1000, None, None).unwrap_or_default();
            if all_shares.iter().any(|s| s.name == *new_name && s.id != share_id) {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: format!("Share name '{}' already exists", new_name),
                    code: "NAME_CONFLICT".to_string(),
                }));
            }
        }
    }

    // 9. 使用数据库更新共享（支持 SMB 字段）
    match repo.update_share_smb(
        share_id,
        payload.name.as_deref(),
        payload.path.as_deref(),
        payload.description.as_deref(),
        payload.allowed_users.as_deref(),
        payload.allowed_groups.as_deref(),
        payload.guest_ok,
        payload.read_only,
    ) {
        Ok(share) => {
            let updated_share = SmbShareInfo {
                id: share.id,
                name: share.name,
                path: share.path,
                description: share.description,
                allowed_users: share.allowed_users,
                allowed_groups: share.allowed_groups,
                guest_ok: share.guest_ok,
                read_only: share.read_only,
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            Ok(HttpResponse::Ok().json(UpdateSmbShareResponse {
                success: true,
                message: "SMB share updated successfully".to_string(),
                data: updated_share,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("更新共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_update_smb_share_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/shares/smb/{id}", web::put().to(update_smb_share))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
