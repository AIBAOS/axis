// Phase 218: WebDAV 共享更新 API
// PUT /api/v1/shares/webdav/{id} — 更新 WebDAV 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// 更新 WebDAV 共享请求
#[derive(Debug, Deserialize)]
pub struct UpdateWebdavShareRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
}

/// WebDAV 共享信息
#[derive(Serialize, Clone)]
pub struct WebdavShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub public: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 更新 WebDAV 共享响应
#[derive(Serialize)]
pub struct UpdateWebdavShareResponse {
    pub success: bool,
    pub message: String,
    pub data: WebdavShareInfo,
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

/// 更新 WebDAV 共享（Phase 218）
/// - JWT 认证，admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库更新
/// - 请求体包含：name/path/description/public（可选，部分更新）
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（仅 WebDAV）
/// - 验证名称格式（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 更新成功返回 200 OK + 共享详情
pub async fn update_webdav_share(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateWebdavShareRequest>,
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
            error: "Only admin users can update WebDAV shares".to_string(),
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

    // 6. 从数据库查询共享是否存在且为 WebDAV 协议
    let existing_share = match repo.get_share_by_id(share_id) {
        Ok(Some(s)) => {
            if s.protocol != "webdav" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("WebDAV share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }
            s
        }
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("WebDAV share {} not found", share_id),
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

    // 7. 使用数据库更新共享
    match repo.update_share(share_id, payload.name.clone(), payload.path.clone(), None, None) {
        Ok(share) => {
            let updated_share = WebdavShareInfo {
                id: share.id,
                name: share.name,
                path: share.path,
                description: payload.description.clone().or(existing_share.description),
                public: payload.public.unwrap_or(share.status == "active"),
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            Ok(HttpResponse::Ok().json(UpdateWebdavShareResponse {
                success: true,
                message: "WebDAV share updated successfully".to_string(),
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
