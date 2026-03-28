// Phase 183: 网络接口详情 API
// GET /api/v1/network/interfaces/{id} — 获取网络接口详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 网络接口详情信息
#[derive(Serialize, Clone)]
pub struct NetworkInterfaceDetail {
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
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub created_at: String,
    pub updated_at: String,
}

/// 网络接口详情响应
#[derive(Serialize)]
pub struct NetworkInterfaceDetailResponse {
    pub success: bool,
    pub data: NetworkInterfaceDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取网络接口详情（Phase 183）
/// - JWT 认证，任意登录用户可访问
/// - 验证接口 ID 存在性（404 Not Found）
/// - 返回接口详细信息
pub async fn get_network_interface_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let interface_id = path.into_inner();

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
    let mock_interfaces = vec![
        NetworkInterfaceDetail {
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
            rx_bytes: 1073741824, // 1GB
            tx_bytes: 536870912,  // 512MB
            rx_packets: 1000000,
            tx_packets: 500000,
            rx_errors: 0,
            tx_errors: 0,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkInterfaceDetail {
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
            rx_bytes: 2147483648, // 2GB
            tx_bytes: 1073741824, // 1GB
            rx_packets: 2000000,
            tx_packets: 1000000,
            rx_errors: 0,
            tx_errors: 0,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NetworkInterfaceDetail {
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
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 4. 查找接口
    let interface = mock_interfaces.into_iter().find(|i| i.id == interface_id);

    // 5. 验证接口存在性
    match interface {
        Some(interface) => {
            // 6. 返回接口详情
            Ok(HttpResponse::Ok().json(NetworkInterfaceDetailResponse {
                success: true,
                data: interface,
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
