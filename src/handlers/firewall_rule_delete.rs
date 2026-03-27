// Phase 138 - 防火墙规则删除 API
// DELETE /api/v1/firewall/rules/{rule_id} — 删除防火墙规则

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

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

/// 删除防火墙规则（Phase 138）
/// - JWT 认证，仅 admin 角色可访问
/// - 路径参数：rule_id（规则 ID）
/// - 验证规则 ID 存在性（404 Not Found）
/// - 删除成功返回 204 No Content
pub async fn delete_firewall_rule(
    req: HttpRequest,
    path: web::Path<u64>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let rule_id = path.into_inner();

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

    // 2. 权限校验 - 仅 admin 角色可访问
    let user_id = claims.user_id;
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete firewall rules".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 模拟防火墙规则数据
    let mut mock_rules = vec![
        (1u64, "Allow HTTP", 100u32, "allow", "tcp", Some("0.0.0.0/0"), Some("80"), Some("192.168.1.0/24"), Some("80"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (2u64, "Allow HTTPS", 110u32, "allow", "tcp", Some("0.0.0.0/0"), Some("443"), Some("192.168.1.0/24"), Some("443"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (3u64, "Deny All", 999u32, "deny", "any", None, None, None, None, None, true, 1711440000u64, 1711440000u64),
        (4u64, "Allow SSH", 50u32, "allow", "tcp", Some("10.0.0.0/8"), Some("22"), Some("192.168.1.0/24"), Some("22"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (5u64, "Allow ICMP", 200u32, "allow", "icmp", Some("0.0.0.0/0"), None, Some("192.168.1.0/24"), None, Some("eth0"), false, 1711440000u64, 1711440000u64),
    ];

    // 4. 验证规则 ID 存在性
    let rule_index = mock_rules.iter().position(|(rid, _, _, _, _, _, _, _, _, _, _, _, _)| *rid == rule_id);

    if rule_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Firewall rule {} not found", rule_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 模拟删除规则
    mock_rules.remove(rule_index.unwrap());

    // 6. 返回删除成功（204 No Content）
    Ok(HttpResponse::NoContent().finish())
}
