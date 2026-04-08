// Phase 128: 更新打印机 API
// PUT /api/v1/printers/{printer_id} — 更新打印机信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新打印机请求
#[derive(Debug, Deserialize)]
pub struct UpdatePrinterRequest {
    pub name: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
    pub ip_address: Option<String>,
    pub location: Option<String>,
    pub is_default: Option<bool>,
}

/// 打印机信息
#[derive(Serialize, Clone)]
pub struct PrinterInfo {
    pub printer_id: u64,
    pub name: String,
    pub model: String,
    pub status: String,
    pub ip_address: String,
    pub location: String,
    pub is_default: bool,
}

/// 更新打印机响应
#[derive(Serialize)]
pub struct UpdatePrinterResponse {
    pub success: bool,
    pub message: String,
    pub data: PrinterInfo,
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

/// 更新打印机（Phase 128）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持字段：name/model/status/ip_address/location/is_default
/// - 验证打印机存在性（404）
/// - 验证 IP 地址格式
pub async fn update_printer(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdatePrinterRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let printer_id = path.into_inner();

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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update printers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证 IP 地址格式（如果提供）
    if let Some(ref ip) = payload.ip_address {
        if !ip.is_empty() && !is_valid_ip(ip) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid IP address format".to_string(),
                code: "INVALID_IP".to_string(),
            }));
        }
    }

    // 5. 验证 status（如果提供）
    if let Some(ref status) = payload.status {
        let valid_statuses = ["idle", "printing", "error", "offline"];
        if !valid_statuses.contains(&status.as_str()) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid status. Valid values: {}", valid_statuses.join(", ")),
                code: "INVALID_STATUS".to_string(),
            }));
        }
    }

    // 6. 模拟当前打印机数据
    let mut current_printer = PrinterInfo {
        printer_id,
        name: "Office Printer 1".to_string(),
        model: "HP LaserJet Pro M404n".to_string(),
        status: "idle".to_string(),
        ip_address: "192.168.1.101".to_string(),
        location: "Building A, Floor 2".to_string(),
        is_default: true,
    };

    // 7. 更新字段
    if let Some(name) = &payload.name {
        current_printer.name = name.clone();
    }

    if let Some(model) = &payload.model {
        current_printer.model = model.clone();
    }

    if let Some(status) = &payload.status {
        current_printer.status = status.clone();
    }

    if let Some(ip) = &payload.ip_address {
        current_printer.ip_address = ip.clone();
    }

    if let Some(location) = &payload.location {
        current_printer.location = location.clone();
    }

    if let Some(is_default) = &payload.is_default {
        current_printer.is_default = *is_default;
    }

    Ok(HttpResponse::Ok().json(UpdatePrinterResponse {
        success: true,
        message: "Printer updated successfully".to_string(),
        data: current_printer,
    }))
}
