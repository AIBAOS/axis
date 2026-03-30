// WiFi 管理处理器（Phase 54）
// 包含：WiFi 扫描、连接、断开等接口

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// WiFi 安全模式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SecurityType {
    Open,
    WEP,
    WPA2,
    WPA3,
    #[serde(rename = "wpa2_wpa3")]
    Wpa2Wpa3,
}

impl std::fmt::Display for SecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityType::Open => write!(f, "open"),
            SecurityType::WEP => write!(f, "wep"),
            SecurityType::WPA2 => write!(f, "wpa2"),
            SecurityType::WPA3 => write!(f, "wpa3"),
            SecurityType::Wpa2Wpa3 => write!(f, "wpa2_wpa3"),
        }
    }
}

/// WiFi 网络信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WiFiNetwork {
    pub ssid: String,
    pub bssid: String,
    pub frequency: String,
    pub channel: u32,
    pub signal_strength: i32,
    pub security: String,
    pub is_connected: bool,
    pub created_at: String,
}

/// WiFi 扫描查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct WiFiScanQuery {
    pub frequency: Option<String>,
}

/// WiFi 扫描响应
#[derive(Debug, Serialize)]
pub struct WiFiScanResponse {
    pub success: bool,
    pub networks: Vec<WiFiNetwork>,
    pub total: u64,
}

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 执行 WiFi 扫描
/// 需要登录用户访问
pub async fn scan_wifi(
    req: HttpRequest,
    query: web::Query<WiFiScanQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(token.unwrap())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let frequency_filter = query.frequency.as_deref();

    // 模拟 WiFi 扫描结果（真实数据可由 iwlist 或 rtnetlink 获取）
    let mut networks = vec![
        WiFiNetwork {
            ssid: "Home_AP".to_string(),
            bssid: "AA:BB:CC:DD:EE:01".to_string(),
            frequency: "2.4G".to_string(),
            channel: 6,
            signal_strength: -45,
            security: "wpa2".to_string(),
            is_connected: true,
            created_at: "2026-03-19T01:15:00Z".to_string(),
        },
        WiFiNetwork {
            ssid: "Office_WiFi".to_string(),
            bssid: "AA:BB:CC:DD:EE:02".to_string(),
            frequency: "5G".to_string(),
            channel: 36,
            signal_strength: -62,
            security: "wpa3".to_string(),
            is_connected: false,
            created_at: "2026-03-19T01:16:00Z".to_string(),
        },
        WiFiNetwork {
            ssid: "Guest_Network".to_string(),
            bssid: "AA:BB:CC:DD:EE:03".to_string(),
            frequency: "5G".to_string(),
            channel: 48,
            signal_strength: -78,
            security: "wpa2".to_string(),
            is_connected: false,
            created_at: "2026-03-19T01:17:00Z".to_string(),
        },
        WiFiNetwork {
            ssid: "IoT_Devices".to_string(),
            bssid: "AA:BB:CC:DD:EE:04".to_string(),
            frequency: "2.4G".to_string(),
            channel: 11,
            signal_strength: -82,
            security: "open".to_string(),
            is_connected: false,
            created_at: "2026-03-19T01:18:00Z".to_string(),
        },
        WiFiNetwork {
            ssid: "Testing_6G".to_string(),
            bssid: "AA:BB:CC:DD:EE:05".to_string(),
            frequency: "6G".to_string(),
            channel: 1,
            signal_strength: -55,
            security: "wpa3".to_string(),
            is_connected: false,
            created_at: "2026-03-19T01:19:00Z".to_string(),
        },
        WiFiNetwork {
            ssid: "Old_AP".to_string(),
            bssid: "AA:BB:CC:DD:EE:06".to_string(),
            frequency: "2.4G".to_string(),
            channel: 1,
            signal_strength: -90,
            security: "wep".to_string(),
            is_connected: false,
            created_at: "2026-03-19T01:20:00Z".to_string(),
        },
    ];

    // 过滤频率
    if let Some(freq) = frequency_filter {
        networks.retain(|n| n.frequency == freq || {
            // 兼容数字输入
            match freq {
                "2.4" | "2" => n.frequency == "2.4G",
                "5" => n.frequency == "5G",
                "6" => n.frequency == "6G",
                _ => true,
            }
        });
    }

    let total = networks.len() as u64;

    Ok(HttpResponse::Ok().json(WiFiScanResponse {
        success: true,
        networks,
        total,
    }))
}

/// 连接 WiFi 网络
/// 仅管理员可执行
pub async fn connect_wifi(
    req: HttpRequest,
    payload: web::Json<ConnectWiFiRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let claims = jwt_service.validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // 仅管理员可连接 WiFi
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "Only admin users can connect to WiFi networks"
        })));
    }

    let ssid = &payload.ssid;
    let _password = &payload.password;

    // 验证必要参数
    if ssid.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "SSID is required"
        })));
    }

    // 简化模拟：成功连接
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Connecting to '{}'", ssid),
        "ssid": ssid,
        "bssid": "AA:BB:CC:DD:EE:01",
        "frequency": "2.4G",
        "channel": 6
    })))
}

/// 断开 WiFi 连接
/// 仅管理员可执行
pub async fn disconnect_wifi(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let claims = jwt_service.validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // 仅管理员可断开 WiFi
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "Only admin users can disconnect WiFi"
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "WiFi disconnected"
    })))
}

/// 获取当前连接状态
/// 需要登录用户访问
pub async fn get_wifi_status(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(token.unwrap())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "connected": true,
        "ssid": "Home_AP",
        "bssid": "AA:BB:CC:DD:EE:01",
        "frequency": "2.4G",
        "channel": 6,
        "signal_strength": -45,
        "ip_address": "192.168.1.100"
    })))
}

/// 获取 WiFi 接口列表
/// 需要登录用户访问
pub async fn list_wifi_interfaces(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(token.unwrap())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(HttpResponse::Ok().json(vec![
        serde_json::json!({
            "name": "wlan0",
            "type": "wifi",
            "status": "up",
            "mac_address": "AA:BB:CC:DD:EE:FF"
        }),
        serde_json::json!({
            "name": "wlan1",
            "type": "wifi",
            "status": "down",
            "mac_address": "AA:BB:CC:DD:EE:FE"
        })
    ]))
}

/// 忘记 WiFi 网络
/// 仅管理员可执行
pub async fn forget_wifi(
    req: HttpRequest,
    payload: web::Json<ForgetWiFiRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let claims = jwt_service.validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // 仅管理员可忘记网络
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "Only admin users can forget WiFi networks"
        })));
    }

    let ssid = &payload.ssid;

    if ssid.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "SSID is required"
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Forgotten network '{}'", ssid),
        "ssid": ssid
    })))
}

/// 获取已保存的 WiFi 网络列表
/// 需要登录用户访问
pub async fn list_saved_wifi(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(token.unwrap())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(HttpResponse::Ok().json(vec![
        serde_json::json!({
            "ssid": "Home_AP",
            "bssid": "AA:BB:CC:DD:EE:01",
            "frequency": "2.4G",
            "security": "wpa2",
            "connected": true
        }),
        serde_json::json!({
            "ssid": "Office_WiFi",
            "bssid": "AA:BB:CC:DD:EE:02",
            "frequency": "5G",
            "security": "wpa3",
            "connected": false
        })
    ]))
}

#[derive(Debug, Deserialize)]
pub struct ConnectWiFiRequest {
    pub ssid: String,
    pub password: Option<String>,
    pub auto_connect: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ForgetWiFiRequest {
    pub ssid: String,
}
