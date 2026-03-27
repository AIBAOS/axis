// Phase 185: 网络接口删除 API
// DELETE /api/v1/network/interfaces/{id} — 删除网络接口

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

/// 删除网络接口（Phase 185）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证接口 ID 存在性（404 Not Found）
/// - 删除成功返回 204 No Content
pub async fn delete_network_interface(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let interface_id = path.into_inner();

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
            error: "Only admin users can delete network interfaces".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟现有接口数据
    let mock_interfaces = vec![1u64, 2u64, 3u64];

    // 5. 验证接口存在性
    let interface_exists = mock_interfaces.iter().any(|i| *i == interface_id);

    if !interface_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Network interface {} not found", interface_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 6. 模拟删除接口
    // （在实际实现中，这里会调用系统命令删除网络接口配置）

    // 7. 返回删除成功（204 No Content）
    Ok(HttpResponse::NoContent().finish())
}
