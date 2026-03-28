//! 网络接口创建 Handler - Phase 132
//! 仅 admin 角色可访问，支持创建以太网/WiFi/桥接/VLAN 接口

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::database::network_store::NetworkRepository;
use crate::middleware::jwt_auth::JwtClaims;

/// 网络接口类型
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Bridge,
    Vlan,
}

impl InterfaceType {
    /// 从字符串解析接口类型
    pub fn from_str(s: &str) -> Option<InterfaceType> {
        match s.to_lowercase().as_str() {
            "ethernet" => Some(InterfaceType::Ethernet),
            "wifi" => Some(InterfaceType::Wifi),
            "bridge" => Some(InterfaceType::Bridge),
            "vlan" => Some(InterfaceType::Vlan),
            _ => None,
        }
    }
    
    /// 获取所有合法的接口类型
    pub fn valid_types() -> &'static [&'static str] {
        &["ethernet", "wifi", "bridge", "vlan"]
    }
}

/// 创建网络接口请求体
#[derive(Debug, Deserialize)]
pub struct CreateInterfaceRequest {
    /// 接口名称（必需，唯一）
    pub name: String,
    /// 接口类型（必需）
    pub r#type: String,
    /// MAC 地址（可选）
    pub mac_address: Option<String>,
    /// IP 地址（可选）
    pub ip_address: Option<String>,
    /// 子网掩码（可选）
    pub netmask: Option<String>,
    /// 网关（可选）
    pub gateway: Option<String>,
    /// 是否启用 DHCP（默认 false）
    pub dhcp_enabled: Option<bool>,
}

/// 网络接口信息响应
#[derive(Debug, Serialize)]
pub struct NetworkInterfaceInfo {
    pub id: String,
    pub name: String,
    pub interface_type: String,
    pub mac_address: Option<String>,
    pub ip_address: Option<String>,
    pub netmask: Option<String>,
    pub gateway: Option<String>,
    pub dhcp_enabled: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建接口响应
#[derive(Debug, Serialize)]
pub struct CreateInterfaceResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<NetworkInterfaceInfo>,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<String>,
}

impl ErrorResponse {
    pub fn bad_request(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("BAD_REQUEST".to_string()),
        }
    }

    pub fn conflict(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("CONFLICT".to_string()),
        }
    }

    pub fn forbidden(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("FORBIDDEN".to_string()),
        }
    }
}

/// IPv4 地址格式校验
fn validate_ipv4(ip: &str) -> bool {
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

/// MAC 地址格式校验（支持 XX:XX:XX:XX:XX:XX 和 XX-XX-XX-XX-XX-XX）
fn validate_mac(mac: &str) -> bool {
    // 移除分隔符后检查长度
    let cleaned = mac.replace(':', "").replace('-', "");
    if cleaned.len() != 12 {
        return false;
    }
    
    // 检查是否都是十六进制字符
    cleaned.chars().all(|c| c.is_ascii_hexdigit())
}

/// 创建网络接口处理器
/// 
/// POST /api/v1/network/interfaces
/// 
/// 权限：仅 admin 角色可访问
/// 请求体：
/// ```json
/// {
///   "name": "eth0",
///   "type": "ethernet",
///   "mac_address": "00:1A:2B:3C:4D:5E",
///   "ip_address": "192.168.1.100",
///   "netmask": "255.255.255.0",
///   "gateway": "192.168.1.1",
///   "dhcp_enabled": false
/// }
/// ```
/// 
/// 响应：
/// - 201 Created: 创建成功，返回接口信息
/// - 400 Bad Request: 参数格式错误（IP/MAC/type 无效）
/// - 403 Forbidden: 非 admin 角色
/// - 409 Conflict: 接口名称已存在
pub async fn create_interface(
    claims: JwtClaims,
    network_repo: web::Data<dyn NetworkRepository>,
    req: web::Json<CreateInterfaceRequest>,
) -> Result<HttpResponse, Error> {
    // 权限校验：仅 admin 角色可创建网络接口
    if !claims.roles.contains(&"admin".to_string()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse::forbidden(
            "仅 admin 角色可创建网络接口",
        )));
    }
    
    // 验证接口类型
    let interface_type = InterfaceType::from_str(&req.r#type)
        .ok_or_else(|| {
            actix_web::error::ErrorBadRequest(format!(
                "无效的接口类型，合法值：{}",
                InterfaceType::valid_types().join(", ")
            ))
        })?;
    
    // 验证 MAC 地址格式（如果提供）
    if let Some(ref mac) = req.mac_address {
        if !validate_mac(mac) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                "MAC 地址格式无效，应为 XX:XX:XX:XX:XX:XX 格式",
            )));
        }
    }
    
    // 验证 IP 地址格式（如果提供）
    if let Some(ref ip) = req.ip_address {
        if !validate_ipv4(ip) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                "IP 地址格式无效，应为 IPv4 格式（如 192.168.1.1）",
            )));
        }
    }
    
    // 验证子网掩码格式（如果提供）
    if let Some(ref netmask) = req.netmask {
        if !validate_ipv4(netmask) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                "子网掩码格式无效，应为 IPv4 格式",
            )));
        }
    }
    
    // 验证网关格式（如果提供）
    if let Some(ref gateway) = req.gateway {
        if !validate_ipv4(gateway) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse::bad_request(
                "网关格式无效，应为 IPv4 格式",
            )));
        }
    }
    
    // 检查接口名称唯一性
    let existing = network_repo.find_by_name(&req.name).await
        .map_err(|e| {
            log::error!("查询接口失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    if existing.is_some() {
        return Ok(HttpResponse::Conflict().json(ErrorResponse::conflict(
            &format!("接口名称已存在：{}", req.name),
        )));
    }
    
    // 创建网络接口
    let now = Utc::now().timestamp();
    let interface = network_repo.create_interface(
        &req.name,
        &req.r#type.to_lowercase(),
        req.mac_address.as_deref(),
        req.ip_address.as_deref(),
        req.netmask.as_deref(),
        req.gateway.as_deref(),
        req.dhcp_enabled.unwrap_or(false),
    ).await
    .map_err(|e| {
        log::error!("创建网络接口失败：{}", e);
        actix_web::error::ErrorInternalServerError("数据库创建失败")
    })?;
    
    // 记录审计日志
    log::info!(
        "admin {} 创建网络接口：name={}, type={}",
        claims.user_id,
        req.name,
        req.r#type,
    );
    
    Ok(HttpResponse::Created().json(CreateInterfaceResponse {
        success: true,
        message: "网络接口创建成功".to_string(),
        data: Some(NetworkInterfaceInfo {
            id: interface.id,
            name: interface.name,
            interface_type: interface.interface_type,
            mac_address: interface.mac_address,
            ip_address: interface.ip_address,
            netmask: interface.netmask,
            gateway: interface.gateway,
            dhcp_enabled: interface.dhcp_enabled,
            status: interface.status,
            created_at: interface.created_at,
            updated_at: interface.updated_at,
        }),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_ipv4() {
        assert!(validate_ipv4("192.168.1.1"));
        assert!(validate_ipv4("0.0.0.0"));
        assert!(validate_ipv4("255.255.255.255"));
        assert!(validate_ipv4("10.0.0.1"));
        
        assert!(!validate_ipv4("256.1.1.1"));
        assert!(!validate_ipv4("192.168.1"));
        assert!(!validate_ipv4("192.168.1.1.1"));
        assert!(!validate_ipv4("abc.def.ghi.jkl"));
        assert!(!validate_ipv4("192.168.1."));
    }
    
    #[test]
    fn test_validate_mac() {
        assert!(validate_mac("00:1A:2B:3C:4D:5E"));
        assert!(validate_mac("aa:bb:cc:dd:ee:ff"));
        assert!(validate_mac("00-1A-2B-3C-4D-5E"));
        assert!(validate_mac("AA-BB-CC-DD-EE-FF"));
        
        assert!(!validate_mac("00:1A:2B:3C:4D"));
        assert!(!validate_mac("00:1A:2B:3C:4D:5E:6F"));
        assert!(!validate_mac("00:1A:2B:3C:4D:GG"));
        assert!(!validate_mac("001A2B3C4D5E"));
    }
    
    #[test]
    fn test_interface_type_from_str() {
        assert_eq!(InterfaceType::from_str("ethernet"), Some(InterfaceType::Ethernet));
        assert_eq!(InterfaceType::from_str("Ethernet"), Some(InterfaceType::Ethernet));
        assert_eq!(InterfaceType::from_str("ETHERNET"), Some(InterfaceType::Ethernet));
        assert_eq!(InterfaceType::from_str("wifi"), Some(InterfaceType::Wifi));
        assert_eq!(InterfaceType::from_str("bridge"), Some(InterfaceType::Bridge));
        assert_eq!(InterfaceType::from_str("vlan"), Some(InterfaceType::Vlan));
        assert_eq!(InterfaceType::from_str("invalid"), None);
    }
    
    #[test]
    fn test_error_response() {
        let err = ErrorResponse::bad_request("测试错误");
        assert!(!err.success);
        assert_eq!(err.message, "测试错误");
        assert_eq!(err.error_code, Some("BAD_REQUEST".to_string()));
        
        let err = ErrorResponse::conflict("名称已存在");
        assert_eq!(err.error_code, Some("CONFLICT".to_string()));
        
        let err = ErrorResponse::forbidden("权限不足");
        assert_eq!(err.error_code, Some("FORBIDDEN".to_string()));
    }
}
