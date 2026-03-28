// Phase 69 - 删除存储卷 API
// DELETE /api/v1/storage/volumes/{id} — 删除存储卷

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除存储卷（Phase 69）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷存在性（404 Not Found）
/// - 检查卷是否正在使用（400 Bad Request）
/// - 系统卷不可删除（403 Forbidden）
/// - 删除成功返回 204 No Content
pub async fn delete_storage_volume(
    req: HttpRequest,
    path: web::Path<u64>,
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

    // 2. 权限校验 - 仅 admin 角色可删除存储卷
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete storage volumes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let volume_id = path.into_inner();

    // 3. 模拟查找存储卷（后续可连接数据库）
    let mock_volumes = vec![
        serde_json::json!({
            "id": 1,
            "name": "System Volume",
            "is_system": true,
            "in_use": true,
        }),
        serde_json::json!({
            "id": 2,
            "name": "Data Volume",
            "is_system": false,
            "in_use": true,
        }),
        serde_json::json!({
            "id": 3,
            "name": "Backup Volume",
            "is_system": false,
            "in_use": false,
        }),
        serde_json::json!({
            "id": 4,
            "name": "Archive Volume",
            "is_system": false,
            "in_use": false,
        }),
    ];

    let volume = mock_volumes.into_iter().find(|v| v["id"] == volume_id);

    match volume {
        Some(v) => {
            // 4. 检查是否为系统卷（系统卷不可删除）
            let is_system = v["is_system"].as_bool().unwrap_or(false);
            if is_system {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot delete system volume '{}'", v["name"].as_str().unwrap()),
                    code: "SYSTEM_VOLUME_PROTECTED".to_string(),
                }));
            }

            // 5. 检查卷是否正在使用
            let in_use = v["in_use"].as_bool().unwrap_or(false);
            if in_use {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot delete volume '{}': volume is currently in use", v["name"].as_str().unwrap()),
                    code: "VOLUME_IN_USE".to_string(),
                }));
            }

            // 6. 模拟删除存储卷（后续可连接数据库）
            // 删除操作成功，返回 204 No Content
            Ok(HttpResponse::NoContent().finish())
        }
        None => {
            // 7. 存储卷不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage volume {} not found", volume_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
