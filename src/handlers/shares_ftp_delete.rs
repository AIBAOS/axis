// Phase 224: FTP 共享删除 API
// DELETE /api/v1/shares/ftp/{id} — 删除 FTP 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// 删除响应
#[derive(Serialize)]
pub struct DeleteResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除 FTP 共享（Phase 224）
/// - JWT 认证，admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库删除
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（仅 FTP，非 FTP 返回 404）
/// - 删除成功返回 204 No Content
pub async fn delete_ftp_share(
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
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete FTP shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询共享是否存在且为 FTP 协议
    let share = match repo.get_share_by_id(share_id) {
        Ok(Some(s)) => {
            if s.protocol != "ftp" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("FTP share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }
            s
        }
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("FTP share {} not found", share_id),
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

    // 5. 使用数据库删除共享
    match repo.delete_share(share_id) {
        Ok(true) => {
            // 6. 返回删除成功（204 No Content）
            Ok(HttpResponse::NoContent().finish())
        }
        Ok(false) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("FTP share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("删除共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
