// Phase 171: 容器删除 API
// DELETE /api/v1/containers/{id} — 删除容器

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除容器（Phase 171）
/// - JWT 认证，admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 删除成功返回 204 No Content
pub async fn delete_container(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let container_id = path.into_inner();

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
            error: "Only admin users can delete containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟现有容器数据
    let mock_containers = vec![1u64, 2u64, 3u64];

    // 5. 验证容器存在性
    let container_exists = mock_containers.iter().any(|id| *id == container_id);

    if !container_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Container {} not found", container_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 6. 模拟删除容器
    // （在实际实现中，这里会调用 Docker API 删除容器）

    // 7. 返回删除成功（204 No Content）
    Ok(HttpResponse::NoContent().finish())
}
