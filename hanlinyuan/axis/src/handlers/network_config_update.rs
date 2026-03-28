// Phase 123: 网络配置更新 API
// PUT /api/v1/network/config — 更新网络配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新网络配置请求
#[derive(Debug, Deserialize)]
pub struct UpdateNetworkConfigRequest {
    pub hostname: Option<String>,
    pub dhcp_enabled: Option<bool>,
    pub ip_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
    pub dns_primary: Option<String>,
    pub dns_secondary: Option<String>,
}

/// 网络配置信息
#[derive(Serialize, Clone)]
pub struct NetworkConfig {
    pub hostname: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dhcp_enabled: bool,
    pub mac_address: String,
    pub connection_status: String,
}

/// 网络配置响应
#[derive(Serialize)]
pub struct NetworkConfigResponse {
    pub success: bool,
    pub message: String,
    pub data: NetworkConfig,
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

/// 更新网络配置（Phase 123）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持更新字段：hostname/dhcp_enabled/ip_address/subnet_mask/gateway/dns_primary/dns_secondary
/// - DHCP 启用时自动忽略静态 IP 配置
/// - 验证 IP 地址格式合法性
/// - 更新后返回完整网络配置
pub async fn update_network_config(
    req: HttpRequest,
    payload: web::Json<UpdateNetworkConfigRequest>,
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
            error: "Only admin users can update network configuration".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证 IP 地址格式（如果提供）
    if let Some(ref ip) = payload.ip_address {
        if !is_valid_ip(ip) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid IP address format".to_string(),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    if let Some(ref mask) = payload.subnet_mask {
        if !is_valid_ip(mask) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid subnet mask format".to_string(),
                code: "INVALID_SUBNET_MASK".to_string(),
            }));
        }
    }

    if let Some(ref gw) = payload.gateway {
        if !is_valid_ip(gw) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid gateway format".to_string(),
                code: "INVALID_GATEWAY".to_string(),
            }));
        }
    }

    if let Some(ref dns) = payload.dns_primary {
        if !is_valid_ip(dns) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid primary DNS format".to_string(),
                code: "INVALID_DNS".to_string(),
            }));
        }
    }

    if let Some(ref dns) = payload.dns_secondary {
        if !is_valid_ip(dns) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid secondary DNS format".to_string(),
                code: "INVALID_DNS".to_string(),
            }));
        }
    }

    // 5. 模拟当前网络配置
    let mut current_config = NetworkConfig {
        hostname: "nas-server".to_string(),
        ip_address: "192.168.1.100".to_string(),
        subnet_mask: "255.255.255.0".to_string(),
        gateway: "192.168.1.1".to_string(),
        dns_primary: "8.8.8.8".to_string(),
        dns_secondary: "8.8.4.4".to_string(),
        dhcp_enabled: true,
        mac_address: "00:1A:2B:3C:4D:5E".to_string(),
        connection_status: "connected".to_string(),
    };

    // 6. 更新配置
    if let Some(hostname) = &payload.hostname {
        current_config.hostname = hostname.clone();
    }

    if let Some(dhcp_enabled) = &payload.dhcp_enabled {
        current_config.dhcp_enabled = *dhcp_enabled;
        // DHCP 启用时自动忽略静态 IP 配置
        if *dhcp_enabled {
            // 模拟 DHCP 自动获取 IP
            current_config.ip_address = "192.168.1.100".to_string();
            current_config.subnet_mask = "255.255.255.0".to_string();
            current_config.gateway = "192.168.1.1".to_string();
            current_config.dns_primary = "8.8.8.8".to_string();
            current_config.dns_secondary = "8.8.4.4".to_string();
        } else {
            // DHCP 禁用时使用静态 IP 配置
            if let Some(ip) = &payload.ip_address {
                current_config.ip_address = ip.clone();
            }
            if let Some(mask) = &payload.subnet_mask {
                current_config.subnet_mask = mask.clone();
            }
            if let Some(gw) = &payload.gateway {
                current_config.gateway = gw.clone();
            }
            if let Some(dns) = &payload.dns_primary {
                current_config.dns_primary = dns.clone();
            }
            if let Some(dns) = &payload.dns_secondary {
                current_config.dns_secondary = dns.clone();
            }
        }
    } else {
        // 未修改 DHCP 设置时，更新静态 IP 配置
        if let Some(ip) = &payload.ip_address {
            current_config.ip_address = ip.clone();
        }
        if let Some(mask) = &payload.subnet_mask {
            current_config.subnet_mask = mask.clone();
        }
        if let Some(gw) = &payload.gateway {
            current_config.gateway = gw.clone();
        }
        if let Some(dns) = &payload.dns_primary {
            current_config.dns_primary = dns.clone();
        }
        if let Some(dns) = &payload.dns_secondary {
            current_config.dns_secondary = dns.clone();
        }
    }

    Ok(HttpResponse::Ok().json(NetworkConfigResponse {
        success: true,
        message: "Network configuration updated successfully".to_string(),
        data: current_config,
    }))
}
