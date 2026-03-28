// Phase 179: 网络配置列表 API
// GET /api/v1/network/config — 获取网络配置列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 网络配置信息
#[derive(Serialize, Clone)]
pub struct NetworkConfig {
    pub id: u64,
    pub interface: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dhcp_enabled: bool,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 网络配置列表响应
#[derive(Serialize)]
pub struct NetworkConfigListResponse {
    pub success: bool,
    pub data: Vec<NetworkConfig>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取网络配置列表（Phase 179）
/// - JWT 认证，admin 角色可访问
/// - 返回所有网络接口配置列表
pub async fn list_network_config(
    req: HttpRequest,
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
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view network configuration".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟网络配置数据
    let network_configs = vec![
        NetworkConfig {
            id: 1,
            interface: "eth0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            dns_primary: "8.8.8.8".to_string(),
            dns_secondary: "8.8.4.4".to_string(),
            dhcp_enabled: false,
            enabled: true,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkConfig {
            id: 2,
            interface: "eth1".to_string(),
            ip_address: "192.168.2.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.2.1".to_string(),
            dns_primary: "1.1.1.1".to_string(),
            dns_secondary: "1.0.0.1".to_string(),
            dhcp_enabled: true,
            enabled: true,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkConfig {
            id: 3,
            interface: "wlan0".to_string(),
            ip_address: "192.168.1.150".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            dns_primary: "8.8.8.8".to_string(),
            dns_secondary: "8.8.4.4".to_string(),
            dhcp_enabled: true,
            enabled: false,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 返回网络配置列表
    Ok(HttpResponse::Ok().json(NetworkConfigListResponse {
        success: true,
        data: network_configs,
    }))
}
