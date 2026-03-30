// Phase 130: 防火墙规则列表 API
// GET /api/v1/firewall/rules — 获取防火墙规则列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 防火墙规则信息
#[derive(Serialize, Clone)]
pub struct FirewallRule {
    pub rule_id: u64,
    pub name: String,
    pub priority: u32,
    pub action: String, // allow/deny
    pub protocol: String, // tcp/udp/icmp
    pub source_ip: String,
    pub source_port: String,
    pub dest_ip: String,
    pub dest_port: String,
    pub interface: String,
    pub enabled: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct FirewallRulesQuery {
    pub action: Option<String>,
    pub protocol: Option<String>,
    pub enabled: Option<bool>,
    pub interface: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 防火墙规则列表响应
#[derive(Serialize)]
pub struct FirewallRulesResponse {
    pub success: bool,
    pub data: Vec<FirewallRule>,
    pub pagination: PaginationInfo,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取防火墙规则列表（Phase 130）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回字段：rule_id/name/priority/action/protocol/source_ip/source_port/dest_ip/dest_port/interface/enabled/created_at/updated_at
/// - 支持筛选参数：action/protocol/enabled/interface
/// - 支持分页：page/per_page
pub async fn list_firewall_rules(
    req: HttpRequest,
    query: web::Query<FirewallRulesQuery>,
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
    let is_admin = _claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access firewall rules".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析分页参数
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).max(1).min(100);

    // 5. 模拟防火墙规则数据
    let all_rules = vec![
        FirewallRule {
            rule_id: 1,
            name: "Allow HTTP".to_string(),
            priority: 100,
            action: "allow".to_string(),
            protocol: "tcp".to_string(),
            source_ip: "0.0.0.0/0".to_string(),
            source_port: "*".to_string(),
            dest_ip: "192.168.1.100".to_string(),
            dest_port: "80".to_string(),
            interface: "eth0".to_string(),
            enabled: true,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        FirewallRule {
            rule_id: 2,
            name: "Allow HTTPS".to_string(),
            priority: 101,
            action: "allow".to_string(),
            protocol: "tcp".to_string(),
            source_ip: "0.0.0.0/0".to_string(),
            source_port: "*".to_string(),
            dest_ip: "192.168.1.100".to_string(),
            dest_port: "443".to_string(),
            interface: "eth0".to_string(),
            enabled: true,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        FirewallRule {
            rule_id: 3,
            name: "Deny All".to_string(),
            priority: 1000,
            action: "deny".to_string(),
            protocol: "tcp".to_string(),
            source_ip: "0.0.0.0/0".to_string(),
            source_port: "*".to_string(),
            dest_ip: "0.0.0.0/0".to_string(),
            dest_port: "*".to_string(),
            interface: "eth0".to_string(),
            enabled: true,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        FirewallRule {
            rule_id: 4,
            name: "Allow SSH".to_string(),
            priority: 50,
            action: "allow".to_string(),
            protocol: "tcp".to_string(),
            source_ip: "192.168.1.0/24".to_string(),
            source_port: "*".to_string(),
            dest_ip: "192.168.1.100".to_string(),
            dest_port: "22".to_string(),
            interface: "eth0".to_string(),
            enabled: false,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        FirewallRule {
            rule_id: 5,
            name: "Allow ICMP".to_string(),
            priority: 200,
            action: "allow".to_string(),
            protocol: "icmp".to_string(),
            source_ip: "0.0.0.0/0".to_string(),
            source_port: "*".to_string(),
            dest_ip: "0.0.0.0/0".to_string(),
            dest_port: "*".to_string(),
            interface: "eth0".to_string(),
            enabled: true,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
    ];

    // 6. 筛选
    let filtered_rules: Vec<FirewallRule> = all_rules
        .into_iter()
        .filter(|rule| {
            // 按 action 筛选
            if let Some(ref filter_action) = query.action {
                if &rule.action != filter_action {
                    return false;
                }
            }
            
            // 按 protocol 筛选
            if let Some(ref filter_protocol) = query.protocol {
                if &rule.protocol != filter_protocol {
                    return false;
                }
            }
            
            // 按 enabled 筛选
            if let Some(filter_enabled) = query.enabled {
                if rule.enabled != filter_enabled {
                    return false;
                }
            }
            
            // 按 interface 筛选
            if let Some(ref filter_interface) = query.interface {
                if &rule.interface != filter_interface {
                    return false;
                }
            }
            
            true
        })
        .collect();

    let total = filtered_rules.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_rules.len());

    let rules: Vec<FirewallRule> = if start < filtered_rules.len() {
        filtered_rules[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(FirewallRulesResponse {
        success: true,
        data: rules,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
