// Phase 206: NFS 共享删除 API (数据库版本)
// DELETE /api/v1/shares/nfs/{id} — 删除 NFS 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除 NFS 共享（Phase 206 - 数据库版本）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库删除
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（非 NFS 返回 404）
/// - 检查共享是否正在使用（409 Conflict）
/// - 删除成功返回 204 No Content
pub async fn delete_nfs_share(
    req: HttpRequest,
    path: web::Path<u64>,
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete NFS shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询共享
    match repo.get_share_by_id(share_id) {
        Ok(Some(share)) => {
            // 5. 验证是 NFS 协议
            if share.protocol != "nfs" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("NFS share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 6. 检查共享是否正在使用（简化实现：active 状态视为使用中）
            // 实际实现可检查 /proc/mounts 或系统挂载点
            if share.status == "active" {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: format!("NFS share '{}' is currently in use and cannot be deleted", share.name),
                    code: "CONFLICT".to_string(),
                }));
            }

            // 7. 执行删除
            match repo.delete_share(share_id) {
                Ok(true) => Ok(HttpResponse::NoContent().finish()),
                Ok(false) => Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("NFS share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                })),
                Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("删除共享失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                })),
            }
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("NFS share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
