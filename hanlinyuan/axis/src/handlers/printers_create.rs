// Phase 55 - 创建打印机 API
// POST /api/v1/printers - 创建新打印机

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::models::jwt::JwtClaims;

/// 打印机类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrinterType {
    Network,
    USB,
    Virtual,
}

impl std::fmt::Display for PrinterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrinterType::Network => write!(f, "network"),
            PrinterType::USB => write!(f, "usb"),
            PrinterType::Virtual => write!(f, "virtual"),
        }
    }
}

/// 创建打印机响应
#[derive(Debug, Serialize)]
pub struct CreatePrinterResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<PrinterInfo>,
}

/// 打印机信息（简化版）
#[derive(Debug, Serialize)]
pub struct PrinterInfo {
    pub id: u64,
    pub name: String,
    pub printer_type: String,
    pub status: String,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建打印机请求
#[derive(Debug, Deserialize)]
pub struct CreatePrinterRequest {
    pub name: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub manufacturer: Option<String>,
    #[serde(rename = "type")]
    pub printer_type: PrinterType,
    #[serde(default)]
    pub ip_address: Option<String>,
    #[serde(default)]
    pub port: Option<u32>,
    #[serde(default)]
    pub usb_device: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub is_default: Option<bool>,
    #[serde(default)]
    pub capabilities: Option<PrinterCapabilities>,
}

/// 打印机能力
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PrinterCapabilities {
    #[serde(default)]
    pub color: bool,
    #[serde(default)]
    pub duplex: bool,
    #[serde(default)]
    pub staple: bool,
    #[serde(default)]
    pub scanning: bool,
    #[serde(default)]
    pub fax: bool,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 创建打印机
pub async fn create_printer(
    jwt_claims: web::Data<JwtClaims>,
    req: web::Json<CreatePrinterRequest>,
) -> Result<HttpResponse, Error> {
    // JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "仅管理员可创建打印机",
            "code": "FORBIDDEN"
        })));
    }

    let now = Utc::now().to_rfc3339();
    let printer_id = 100; // 模拟 ID

    let response = CreatePrinterResponse {
        success: true,
        message: "打印机创建成功".to_string(),
        data: Some(PrinterInfo {
            id: printer_id,
            name: req.name.clone(),
            printer_type: req.printer_type.to_string(),
            status: "idle".to_string(),
            is_default: req.is_default.unwrap_or(false),
            created_at: now.clone(),
            updated_at: now,
        }),
    };

    Ok(HttpResponse::Ok().json(response))
}
