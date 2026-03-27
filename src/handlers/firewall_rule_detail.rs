// Phase 136 - 防火墙规则详情 API
// GET /api/v1/firewall/rules/{rule_id} — 获取防火墙规则详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 防火墙规则详情
#[derive(Serialize, Clone)]
pub struct FirewallRuleDetail {
    pub rule_id: u64,
    pub name: String,
    pub priority: u32,
    pub action: String, // allow/deny/drop
    pub protocol: String, // tcp/udp/icmp/any
    pub source_ip: Option<String>,
    pub source_port: Option<String>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<String>,
    pub interface: Option<String>,
    pub enabled: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 防火墙规则详情响应
#[derive(Serialize)]
pub struct FirewallRuleDetailResponse {
    pub success: bool,
    pub data: FirewallRuleDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取防火墙规则详情（Phase 136）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证规则 ID 存在性 (404)
pub async fn get_firewall_rule_detail(
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

    // 2. 权限校验 - 仅 admin 角色可访问
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access firewall rules".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let rule_id = path.into_inner();

    // 3. 模拟防火墙规则数据
    let mock_rules = vec![
        (1u64, "Allow HTTP", 100u32, "allow", "tcp", Some("0.0.0.0/0"), Some("80"), Some("192.168.1.0/24"), Some("80"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (2u64, "Allow HTTPS", 110u32, "allow", "tcp", Some("0.0.0.0/0"), Some("443"), Some("192.168.1.0/24"), Some("443"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (3u64, "Deny All", 999u32, "deny", "any", None, None, None, None, None, true, 1711440000u64, 1711440000u64),
        (4u64, "Allow SSH", 50u32, "allow", "tcp", Some("10.0.0.0/8"), Some("22"), Some("192.168.1.0/24"), Some("22"), Some("eth0"), true, 1711440000u64, 1711440000u64),
        (5u64, "Allow ICMP", 200u32, "allow", "icmp", Some("0.0.0.0/0"), None, Some("192.168.1.0/24"), None, Some("eth0"), false, 1711440000u64, 1711440000u64),
    ];

    // 4. 查找规则
    let rule = mock_rules.into_iter().find(|(rid, _, _, _, _, _, _, _, _, _, _, _, _)| *rid == rule_id);

    match rule {
        Some((rid, name, priority, action, protocol, source_ip, source_port, dest_ip, dest_port, interface, enabled, created_at, updated_at)) => {
            let rule_detail = FirewallRuleDetail {
                rule_id: rid,
                name: name.to_string(),
                priority,
                action: action.to_string(),
                protocol: protocol.to_string(),
                source_ip: source_ip.map(String::from),
                source_port: source_port.map(String::from),
                dest_ip: dest_ip.map(String::from),
                dest_port: dest_port.map(String::from),
                interface: interface.map(String::from),
                enabled,
                created_at,
                updated_at,
            };

            Ok(HttpResponse::Ok().json(FirewallRuleDetailResponse {
                success: true,
                data: rule_detail,
            }))
        }
        None => {
            // 5. 规则不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Firewall rule {} not found", rule_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
