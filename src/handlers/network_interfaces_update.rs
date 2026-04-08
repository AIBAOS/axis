// Phase 135: 网络接口更新 API
// PUT /api/v1/network/interfaces/{id} — 更新网络接口配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 网络接口信息
#[derive(Serialize, Clone)]
pub struct NetworkInterfaceInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub mac_address: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_servers: Vec<String>,
    pub enabled: bool,
    pub mtu: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 更新网络接口请求
#[derive(Deserialize)]
pub struct UpdateNetworkInterfaceRequest {
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Option<Vec<String>>,
    pub enabled: Option<bool>,
    pub mtu: Option<u32>,
}

/// 网络接口更新响应
#[derive(Serialize)]
pub struct UpdateNetworkInterfaceResponse {
    pub success: bool,
    pub message: String,
    pub data: NetworkInterfaceInfo,
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
    let octets: Vec<&str> = ip.split('.').collect();
    if octets.len() != 4 {
        return false;
    }
    for octet in octets {
        if let Ok(n) = octet.parse::<u8>() {
            if n > 255 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// 验证子网掩码格式
fn is_valid_subnet_mask(mask: &str) -> bool {
    let octets: Vec<&str> = mask.split('.').collect();
    if octets.len() != 4 {
        return false;
    }
    for octet in octets {
        if let Ok(n) = octet.parse::<u8>() {
            if n > 255 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

/// 更新网络接口（Phase 135）
/// - JWT 认证，仅 admin 角色可访问
/// - 路径参数：id（网络接口 ID）
/// - 支持部分更新字段：description/ip_address/subnet_mask/gateway/dns_servers/enabled/mtu
/// - 验证 IP 地址格式（400）
/// - 验证子网掩码格式（400）
pub async fn update_network_interface(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateNetworkInterfaceRequest>,
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
            error: "Only admin users can update network interfaces".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let interface_id = path.into_inner();

    // 4. 模拟网络接口数据
    let mut mock_interfaces = vec![
        NetworkInterfaceInfo {
            id: 1,
            name: "eth0".to_string(),
            description: "Primary Network Interface".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            ip_address: "192.168.1.100".to_string(),
            subnet_mask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            enabled: true,
            mtu: 1500,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
        NetworkInterfaceInfo {
            id: 2,
            name: "eth1".to_string(),
            description: "Secondary Network Interface".to_string(),
            mac_address: "00:11:22:33:44:56".to_string(),
            ip_address: "10.0.0.100".to_string(),
            subnet_mask: "255.255.255.0".to_string(),
            gateway: "10.0.0.1".to_string(),
            dns_servers: vec!["1.1.1.1".to_string()],
            enabled: false,
            mtu: 1500,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
    ];

    // 5. 查找接口
    let interface_index = mock_interfaces.iter().position(|i| i.id == interface_id);

    match interface_index {
        Some(idx) => {
            let interface = &mut mock_interfaces[idx];
            let req_data = payload.into_inner();

            // 6. 验证并应用更新
            if let Some(ref new_ip) = req_data.ip_address {
                if !is_valid_ip(new_ip) {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: format!("Invalid IP address format: {}", new_ip),
                        code: "INVALID_IP".to_string(),
                    }));
                }
                interface.ip_address = new_ip.clone();
            }

            if let Some(ref new_subnet) = req_data.subnet_mask {
                if !is_valid_subnet_mask(new_subnet) {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: format!("Invalid subnet mask format: {}", new_subnet),
                        code: "INVALID_SUBNET".to_string(),
                    }));
                }
                interface.subnet_mask = new_subnet.clone();
            }

            if let Some(ref new_gateway) = req_data.gateway {
                if !new_gateway.is_empty() && !is_valid_ip(new_gateway) {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: format!("Invalid gateway IP format: {}", new_gateway),
                        code: "INVALID_GATEWAY".to_string(),
                    }));
                }
                interface.gateway = new_gateway.clone();
            }

            if let Some(new_dns) = req_data.dns_servers {
                // 验证所有 DNS 服务器格式
                for dns in &new_dns {
                    if !is_valid_ip(dns) {
                        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                            success: false,
                            error: format!("Invalid DNS server IP format: {}", dns),
                            code: "INVALID_DNS".to_string(),
                        }));
                    }
                }
                interface.dns_servers = new_dns;
            }

            if let Some(new_description) = req_data.description {
                interface.description = new_description;
            }

            if let Some(new_enabled) = req_data.enabled {
                interface.enabled = new_enabled;
            }

            if let Some(new_mtu) = req_data.mtu {
                if new_mtu < 68 || new_mtu > 65535 {
                    return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                        success: false,
                        error: format!("Invalid MTU value: {}. Must be between 68 and 65535", new_mtu),
                        code: "INVALID_MTU".to_string(),
                    }));
                }
                interface.mtu = new_mtu;
            }

            // 7. 更新 updated_at
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Time error"))?
                .as_secs();
            interface.updated_at = now;

            Ok(HttpResponse::Ok().json(UpdateNetworkInterfaceResponse {
                success: true,
                message: format!("Network interface {} updated successfully", interface.name),
                data: interface.clone(),
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Network interface {} not found", interface_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
