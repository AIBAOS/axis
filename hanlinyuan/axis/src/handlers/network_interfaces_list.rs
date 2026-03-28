// Phase 182: 网络接口列表 API
// GET /api/v1/network/interfaces — 获取网络接口列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 网络接口信息
#[derive(Serialize, Clone)]
pub struct NetworkInterface {
    pub id: u64,
    pub name: String,
    pub interface: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub mac_address: String,
    pub status: String,
    pub interface_type: String,
    pub speed_mbps: Option<u32>,
    pub mtu: u32,
    pub created_at: String,
    pub updated_at: String,
}

/// 网络接口列表响应
#[derive(Serialize)]
pub struct NetworkInterfaceListResponse {
    pub success: bool,
    pub data: Vec<NetworkInterface>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取网络接口列表（Phase 182）
/// - JWT 认证，任意登录用户可访问
/// - 返回所有网络接口列表
pub async fn list_network_interfaces(
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
    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 模拟网络接口数据
    let interfaces = vec![
        NetworkInterface {
            id: 1,
            name: "Primary Network".to_string(),
            interface: "eth0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            mac_address: "00:1A:2B:3C:4D:5E".to_string(),
            status: "up".to_string(),
            interface_type: "ethernet".to_string(),
            speed_mbps: Some(1000),
            mtu: 1500,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkInterface {
            id: 2,
            name: "Secondary Network".to_string(),
            interface: "eth1".to_string(),
            ip_address: "192.168.2.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.2.1".to_string(),
            mac_address: "00:1A:2B:3C:4D:5F".to_string(),
            status: "up".to_string(),
            interface_type: "ethernet".to_string(),
            speed_mbps: Some(1000),
            mtu: 1500,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkInterface {
            id: 3,
            name: "Wireless Network".to_string(),
            interface: "wlan0".to_string(),
            ip_address: "192.168.1.150".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            mac_address: "00:1A:2B:3C:4D:60".to_string(),
            status: "down".to_string(),
            interface_type: "wireless".to_string(),
            speed_mbps: Some(300),
            mtu: 1500,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 4. 返回网络接口列表
    Ok(HttpResponse::Ok().json(NetworkInterfaceListResponse {
        success: true,
        data: interfaces,
    }))
}
