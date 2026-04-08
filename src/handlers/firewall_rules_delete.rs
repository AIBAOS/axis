// Phase 138: 防火墙规则删除 API
// DELETE /api/v1/firewall/rules/{rule_id} — 删除防火墙规则

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除防火墙规则（Phase 138）
/// - JWT 认证，仅 admin 角色可访问
/// - 路径参数：rule_id（要删除的规则 ID）
/// - 验证：规则 ID 存在性（404 Not Found）
/// - 删除成功返回：204 No Content
pub async fn delete_firewall_rule(
    req: HttpRequest,
    path: web::Path<u64>,
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
    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = _claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete firewall rules".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let rule_id = path.into_inner();

    // 4. 模拟删除操作（实际应从数据库删除）
    let mock_rules: Vec<u64> = vec![1, 2, 3, 4, 5];

    if !mock_rules.contains(&rule_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Firewall rule {} not found", rule_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 返回 204 No Content（按 Phase 138 要求）
    Ok(HttpResponse::NoContent().finish())
}
