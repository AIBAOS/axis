// Phase 137 - 防火墙规则更新 API
// PUT /api/v1/firewall/rules/{rule_id} — 更新防火墙规则

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 更新防火墙规则请求
#[derive(Deserialize)]
pub struct UpdateFirewallRuleRequest {
    pub name: Option<String>,
    pub priority: Option<u32>,
    pub action: Option<String>,
    pub protocol: Option<String>,
    pub source_ip: Option<String>,
    pub source_port: Option<String>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<String>,
    pub interface: Option<String>,
    pub enabled: Option<bool>,
}

/// 防火墙规则详情
#[derive(Serialize, Clone)]
pub struct FirewallRuleDetail {
    pub rule_id: u64,
    pub name: String,
    pub priority: u32,
    pub action: String,
    pub protocol: String,
    pub source_ip: Option<String>,
    pub source_port: Option<String>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<String>,
    pub interface: Option<String>,
    pub enabled: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 防火墙规则更新响应
#[derive(Serialize)]
pub struct UpdateFirewallRuleResponse {
    pub success: bool,
    pub message: String,
    pub data: FirewallRuleDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证 IPv4 地址或 CIDR 格式
fn validate_ip_or_cidr(ip: &str) -> bool {
    // 简单验证：检查是否包含 CIDR 前缀
    let parts: Vec<&str> = ip.split('/').collect();
    if parts.len() > 2 {
        return false;
    }
    
    // 验证 IP 地址部分
    let ip_part = parts[0];
    if ip_part.parse::<Ipv4Addr>().is_err() {
        return false;
    }
    
    // 如果有 CIDR 前缀，验证其合法性
    if parts.len() == 2 {
        if let Ok(prefix) = parts[1].parse::<u32>() {
            if prefix > 32 {
                return false;
            }
        } else {
            return false;
        }
    }
    
    true
}

/// 验证端口范围格式
fn validate_port_range(port: &str) -> bool {
    let parts: Vec<&str> = port.split('-').collect();
    if parts.len() > 2 {
        return false;
    }
    
    for part in parts {
        if let Ok(port_num) = part.parse::<u32>() {
            if port_num > 65535 {
                return false;
            }
        } else {
            return false;
        }
    }
    
    true
}

/// 更新防火墙规则（Phase 137）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持部分更新
/// - 验证规则 ID 存在性 (404)
/// - 验证规则名称唯一性 (409)
/// - 验证 priority/action/protocol/IP/port 合法性 (400)
pub async fn update_firewall_rule(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateFirewallRuleRequest>,
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
    let user_id = claims.user_id;
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update firewall rules".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let rule_id = path.into_inner();

    // 3. 验证参数字段合法性
    if let Some(ref name) = payload.name {
        if name.is_empty() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "name cannot be empty".to_string(),
                code: "INVALID_NAME".to_string(),
            }));
        }
    }

    if let Some(priority) = payload.priority {
        if priority > 9999 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "priority must be between 0 and 9999".to_string(),
                code: "INVALID_PRIORITY".to_string(),
            }));
        }
    }

    if let Some(ref action) = payload.action {
        let valid_actions = vec!["allow", "deny", "drop"];
        if !valid_actions.contains(&action.as_str()) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid action. Valid actions: {}", valid_actions.join(", ")),
                code: "INVALID_ACTION".to_string(),
            }));
        }
    }

    if let Some(ref protocol) = payload.protocol {
        let valid_protocols = vec!["tcp", "udp", "icmp", "any"];
        if !valid_protocols.contains(&protocol.as_str()) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid protocol. Valid protocols: {}", valid_protocols.join(", ")),
                code: "INVALID_PROTOCOL".to_string(),
            }));
        }
    }

    if let Some(ref ip) = payload.source_ip {
        if !ip.is_empty() && !validate_ip_or_cidr(ip) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid source_ip format: {}", ip),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    if let Some(ref ip) = payload.dest_ip {
        if !ip.is_empty() && !validate_ip_or_cidr(ip) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid dest_ip format: {}", ip),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    if let Some(ref port) = payload.source_port {
        if !port.is_empty() && !validate_port_range(port) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid source_port format: {}", port),
                code: "INVALID_PORT".to_string(),
            }));
        }
    }

    if let Some(ref port) = payload.dest_port {
        if !port.is_empty() && !validate_port_range(port) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid dest_port format: {}", port),
                code: "INVALID_PORT".to_string(),
            }));
        }
    }

    // 4. 模拟防火墙规则数据
    let mut mock_rules: Vec<(u64, String, u32, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, bool, u64, u64)> = vec![
        (1, "Allow HTTP".to_string(), 100, "allow".to_string(), "tcp".to_string(), Some("0.0.0.0/0".to_string()), Some("80".to_string()), Some("192.168.1.0/24".to_string()), Some("80".to_string()), Some("eth0".to_string()), true, 1711440000, 1711440000),
        (2, "Allow HTTPS".to_string(), 110, "allow".to_string(), "tcp".to_string(), Some("0.0.0.0/0".to_string()), Some("443".to_string()), Some("192.168.1.0/24".to_string()), Some("443".to_string()), Some("eth0".to_string()), true, 1711440000, 1711440000),
        (3, "Deny All".to_string(), 999, "deny".to_string(), "any".to_string(), None, None, None, None, None, true, 1711440000, 1711440000),
        (4, "Allow SSH".to_string(), 50, "allow".to_string(), "tcp".to_string(), Some("10.0.0.0/8".to_string()), Some("22".to_string()), Some("192.168.1.0/24".to_string()), Some("22".to_string()), Some("eth0".to_string()), true, 1711440000, 1711440000),
        (5, "Allow ICMP".to_string(), 200, "allow".to_string(), "icmp".to_string(), Some("0.0.0.0/0".to_string()), None, Some("192.168.1.0/24".to_string()), None, Some("eth0".to_string()), false, 1711440000, 1711440000),
    ];

    // 5. 查找规则
    let rule_index = mock_rules.iter().position(|(rid, _, _, _, _, _, _, _, _, _, _, _, _)| *rid == rule_id);

    match rule_index {
        Some(idx) => {
            // 6. 验证规则名称唯一性（排除自身）
            if let Some(ref new_name) = payload.name {
                let name_exists = mock_rules.iter().any(|(rid, name, _, _, _, _, _, _, _, _, _, _, _)| {
                    *rid != rule_id && name == new_name
                });
                if name_exists {
                    return Ok(HttpResponse::Conflict().json(ErrorResponse {
                        success: false,
                        error: format!("Firewall rule name '{}' already exists", new_name),
                        code: "NAME_CONFLICT".to_string(),
                    }));
                }
            }

            // 7. 更新字段（部分更新）
            if let Some(new_name) = &payload.name {
                mock_rules[idx].1 = new_name.clone();
            }
            if let Some(new_priority) = payload.priority {
                mock_rules[idx].2 = new_priority;
            }
            if let Some(new_action) = &payload.action {
                mock_rules[idx].3 = new_action.clone();
            }
            if let Some(new_protocol) = &payload.protocol {
                mock_rules[idx].4 = new_protocol.clone();
            }
            if let Some(ref new_ip) = payload.source_ip {
                mock_rules[idx].5 = Some(new_ip.clone());
            }
            if let Some(ref new_port) = payload.source_port {
                mock_rules[idx].6 = Some(new_port.clone());
            }
            if let Some(ref new_ip) = payload.dest_ip {
                mock_rules[idx].7 = Some(new_ip.clone());
            }
            if let Some(ref new_port) = payload.dest_port {
                mock_rules[idx].8 = Some(new_port.clone());
            }
            if let Some(new_interface) = &payload.interface {
                mock_rules[idx].9 = Some(new_interface.clone());
            }
            if let Some(new_enabled) = payload.enabled {
                mock_rules[idx].10 = new_enabled;
            }

            // 更新 updated_at
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();

            // 8. 返回更新后的规则详情
            let rule = &mock_rules[idx];
            let rule_detail = FirewallRuleDetail {
                rule_id,
                name: rule.1.clone(),
                priority: rule.2,
                action: rule.3.clone(),
                protocol: rule.4.clone(),
                source_ip: rule.5.clone(),
                source_port: rule.6.clone(),
                dest_ip: rule.7.clone(),
                dest_port: rule.8.clone(),
                interface: rule.9.clone(),
                enabled: rule.10,
                created_at: rule.11,
                updated_at: now,
            };

            Ok(HttpResponse::Ok().json(UpdateFirewallRuleResponse {
                success: true,
                message: "Firewall rule updated successfully".to_string(),
                data: rule_detail,
            }))
        }
        None => {
            // 9. 规则不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Firewall rule {} not found", rule_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
