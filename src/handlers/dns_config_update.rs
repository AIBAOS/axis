// Phase 125: DNS 配置更新 API
// PUT /api/v1/network/dns — 更新 DNS 配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新 DNS 配置请求
#[derive(Debug, Deserialize)]
pub struct UpdateDnsConfigRequest {
    pub dns_primary: Option<String>,
    pub dns_secondary: Option<String>,
    pub dns_mode: Option<String>, // auto 或 manual
}

/// DNS 配置信息
#[derive(Serialize, Clone)]
pub struct DnsConfig {
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dns_search_domains: Vec<String>,
    pub dns_mode: String,
}

/// DNS 配置响应
#[derive(Serialize)]
pub struct DnsConfigResponse {
    pub success: bool,
    pub message: String,
    pub data: DnsConfig,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证 IP 地址格式
fn is_valid_ip(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    for part in parts {
        match part.parse::<u8>() {
            Ok(_) => continue,
            Err(_) => return false,
        }
    }
    true
}

/// 更新 DNS 配置（Phase 125）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持字段：dns_primary/dns_secondary/dns_mode
/// - 验证 DNS 服务器 IP 格式合法性
/// - manual 模式时 dns_primary 必填
/// - auto 模式时从 DHCP 获取
pub async fn update_dns_config(
    req: HttpRequest,
    payload: web::Json<UpdateDnsConfigRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并获取用户角色
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update DNS configuration".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证 dns_mode
    if let Some(ref mode) = payload.dns_mode {
        if mode != "auto" && mode != "manual" {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid dns_mode. Valid values: auto, manual".to_string(),
                code: "INVALID_DNS_MODE".to_string(),
            }));
        }

        // manual 模式时 dns_primary 必填
        if mode == "manual" && payload.dns_primary.is_none() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "dns_primary is required in manual mode".to_string(),
                code: "MISSING_DNS_PRIMARY".to_string(),
            }));
        }
    }

    // 5. 验证 DNS 服务器 IP 格式
    if let Some(ref dns) = payload.dns_primary {
        if !dns.is_empty() && !is_valid_ip(dns) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid primary DNS IP address format".to_string(),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    if let Some(ref dns) = payload.dns_secondary {
        if !dns.is_empty() && !is_valid_ip(dns) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid secondary DNS IP address format".to_string(),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    // 6. 模拟当前 DNS 配置
    let mut current_config = DnsConfig {
        dns_primary: "8.8.8.8".to_string(),
        dns_secondary: "8.8.4.4".to_string(),
        dns_search_domains: vec!["local".to_string(), "lan".to_string()],
        dns_mode: "manual".to_string(),
    };

    // 7. 更新配置
    if let Some(mode) = &payload.dns_mode {
        current_config.dns_mode = mode.clone();
        
        if mode == "auto" {
            // auto 模式时从 DHCP 获取
            current_config.dns_primary = "DHCP".to_string();
            current_config.dns_secondary = "DHCP".to_string();
        }
    }

    if let Some(dns) = &payload.dns_primary {
        current_config.dns_primary = dns.clone();
    }

    if let Some(dns) = &payload.dns_secondary {
        current_config.dns_secondary = dns.clone();
    }

    Ok(HttpResponse::Ok().json(DnsConfigResponse {
        success: true,
        message: "DNS configuration updated successfully".to_string(),
        data: current_config,
    }))
}
