// Phase 129: 网络接口列表 API
// GET /api/v1/network/interfaces — 获取网络接口列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 网络接口信息
#[derive(Serialize, Clone)]
pub struct NetworkInterface {
    pub interface_id: u64,
    pub name: String,
    pub r#type: String, // mac/ethernet/wifi
    pub mac_address: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub status: String, // up/down
    pub speed_mbps: u32,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct NetworkInterfaceQuery {
    pub r#type: Option<String>,
    pub status: Option<String>,
}

/// 网络接口列表响应
#[derive(Serialize)]
pub struct NetworkInterfacesResponse {
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

/// 获取网络接口列表（Phase 129）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回字段：interface_id/name/type/mac_address/ip_address/netmask/gateway/status/speed_mbps
/// - 支持筛选参数：?type=ethernet 或 ?status=up
/// - 验证：JWT 有效性（401）、角色权限（403）
pub async fn list_network_interfaces(
    req: HttpRequest,
    query: web::Query<NetworkInterfaceQuery>,
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
            error: "Only admin users can access network interfaces".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟网络接口数据
    let all_interfaces = vec![
        NetworkInterface {
            interface_id: 1,
            name: "eth0".to_string(),
            r#type: "ethernet".to_string(),
            mac_address: "00:1A:2B:3C:4D:5E".to_string(),
            ip_address: "192.168.1.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            status: "up".to_string(),
            speed_mbps: 1000,
        },
        NetworkInterface {
            interface_id: 2,
            name: "wlan0".to_string(),
            r#type: "wifi".to_string(),
            mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
            ip_address: "192.168.1.101".to_string(),
            netmask: "255.255.255.0".to_string(),
            gateway: "192.168.1.1".to_string(),
            status: "up".to_string(),
            speed_mbps: 300,
        },
        NetworkInterface {
            interface_id: 3,
            name: "lo".to_string(),
            r#type: "loopback".to_string(),
            mac_address: "00:00:00:00:00:00".to_string(),
            ip_address: "127.0.0.1".to_string(),
            netmask: "255.0.0.0".to_string(),
            gateway: "".to_string(),
            status: "up".to_string(),
            speed_mbps: 0,
        },
        NetworkInterface {
            interface_id: 4,
            name: "eth1".to_string(),
            r#type: "ethernet".to_string(),
            mac_address: "00:1A:2B:3C:4D:5F".to_string(),
            ip_address: "0.0.0.0".to_string(),
            netmask: "0.0.0.0".to_string(),
            gateway: "".to_string(),
            status: "down".to_string(),
            speed_mbps: 1000,
        },
    ];

    // 5. 筛选
    let filtered_interfaces: Vec<NetworkInterface> = all_interfaces
        .into_iter()
        .filter(|iface| {
            // 按 type 筛选
            if let Some(ref filter_type) = query.r#type {
                if &iface.r#type != filter_type {
                    return false;
                }
            }
            
            // 按 status 筛选
            if let Some(ref filter_status) = query.status {
                if &iface.status != filter_status {
                    return false;
                }
            }
            
            true
        })
        .collect();

    Ok(HttpResponse::Ok().json(NetworkInterfacesResponse {
        success: true,
        data: filtered_interfaces,
    }))
}
