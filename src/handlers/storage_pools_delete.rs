// Phase 66 - 存储池删除 API
// DELETE /api/v1/storage/pools/{id} — 删除存储池

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

/// 删除存储池（Phase 66）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储池存在性（404 Not Found）
/// - 验证是否有卷在使用（400 Bad Request）
/// - 删除成功返回 204 No Content
pub async fn delete_storage_pool(
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

    // 2. 权限校验 - 仅 admin 角色可删除存储池
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete storage pools".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let pool_id = path.into_inner();

    // 3. 模拟查找存储池（后续可连接数据库）
    let mock_pools = vec![
        serde_json::json!({
            "id": 1,
            "name": "System Pool",
            "has_volumes": true,
        }),
        serde_json::json!({
            "id": 2,
            "name": "Data Pool",
            "has_volumes": true,
        }),
        serde_json::json!({
            "id": 3,
            "name": "Backup Pool",
            "has_volumes": false,
        }),
        serde_json::json!({
            "id": 4,
            "name": "Archive Pool",
            "has_volumes": false,
        }),
    ];

    let pool = mock_pools.into_iter().find(|p| p["id"] == pool_id);

    match pool {
        Some(p) => {
            // 4. 检查是否有卷在使用该存储池
            let has_volumes = p["has_volumes"].as_bool().unwrap_or(false);
            if has_volumes {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Cannot delete storage pool '{}': volumes are using this pool", p["name"].as_str().unwrap_or("unknown")),
                    code: "POOL_IN_USE".to_string(),
                }));
            }

            // 5. 模拟删除存储池（后续可连接数据库）
            // 删除操作成功，返回 204 No Content
            Ok(HttpResponse::NoContent().finish())
        }
        None => {
            // 6. 存储池不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Storage pool {} not found", pool_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
