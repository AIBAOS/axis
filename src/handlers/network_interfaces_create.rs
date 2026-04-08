// Phase 184: 创建网络接口 API
// POST /api/v1/network/interfaces — 创建网络接口

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 创建网络接口请求
#[derive(Debug, Deserialize)]
pub struct CreateNetworkInterfaceRequest {
    pub name: String,
    pub interface: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub interface_type: String,
    pub speed_mbps: Option<u32>,
    pub mtu: Option<u32>,
}

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

/// 创建网络接口响应
#[derive(Serialize)]
pub struct CreateNetworkInterfaceResponse {
    pub success: bool,
    pub message: String,
    pub data: NetworkInterface,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证 IP 地址格式
fn validate_ip_address(ip: &str) -> bool {
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

/// 创建网络接口（Phase 184）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证 IP 地址格式（400 Bad Request）
/// - 验证接口名称唯一性（409 Conflict）
/// - 创建成功返回 201 Created + 接口详情
pub async fn create_network_interface(
    req: HttpRequest,
    payload: web::Json<CreateNetworkInterfaceRequest>,
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create network interfaces".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证 IP 地址格式
    if !validate_ip_address(&payload.ip_address) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid IP address format".to_string(),
            code: "INVALID_IP_ADDRESS".to_string(),
        }));
    }

    if !validate_ip_address(&payload.netmask) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid netmask format".to_string(),
            code: "INVALID_NETMASK".to_string(),
        }));
    }

    if !validate_ip_address(&payload.gateway) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid gateway format".to_string(),
            code: "INVALID_GATEWAY".to_string(),
        }));
    }

    // 5. 模拟现有接口数据（用于名称唯一性检查）
    let existing_interfaces = vec!["eth0", "eth1", "wlan0"];

    // 6. 验证接口名称唯一性
    if existing_interfaces.contains(&payload.interface.as_str()) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Network interface '{}' already exists", payload.interface),
            code: "INTERFACE_EXISTS".to_string(),
        }));
    }

    // 7. 生成 MAC 地址（模拟）
    let mac_address = format!(
        "00:1A:2B:{:02X}:{:02X}:{:02X}",
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    );

    // 8. 创建新接口
    let now = chrono::Utc::now().to_rfc3339();
    let new_interface = NetworkInterface {
        id: 4, // 模拟自增 ID
        name: payload.name.clone(),
        interface: payload.interface.clone(),
        ip_address: payload.ip_address.clone(),
        netmask: payload.netmask.clone(),
        gateway: payload.gateway.clone(),
        mac_address,
        status: "up".to_string(),
        interface_type: payload.interface_type.clone(),
        speed_mbps: payload.speed_mbps,
        mtu: payload.mtu.unwrap_or(1500),
        created_at: now.clone(),
        updated_at: now,
    };

    // 9. 返回创建成功
    Ok(HttpResponse::Created().json(CreateNetworkInterfaceResponse {
        success: true,
        message: "Network interface created successfully".to_string(),
        data: new_interface,
    }))
}
