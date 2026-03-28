// Phase 127: 打印机详情 API
// GET /api/v1/printers/{printer_id} — 获取打印机详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 打印机详情信息
#[derive(Serialize, Clone)]
pub struct PrinterDetail {
    pub printer_id: u64,
    pub name: String,
    pub model: String,
    pub status: String,
    pub ip_address: String,
    pub location: String,
    pub is_default: bool,
    pub capabilities: Option<Vec<String>>,
}

/// 打印机详情响应
#[derive(Serialize)]
pub struct PrinterDetailResponse {
    pub success: bool,
    pub data: PrinterDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取打印机详情（Phase 127）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回字段：printer_id/name/model/status/ip_address/location/is_default/capabilities
/// - 验证打印机存在性，不存在返回 404
pub async fn get_printer_detail(
    req: HttpRequest,
    path: web::Path<u64>,
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
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access printer details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟打印机数据
    let printers = vec![
        (
            1,
            "Office Printer 1",
            "HP LaserJet Pro M404n",
            "idle",
            "192.168.1.101",
            "Building A, Floor 2",
            true,
            vec!["duplex".to_string(), "color".to_string(), "wifi".to_string()],
        ),
        (
            2,
            "Office Printer 2",
            "Canon imageRUNNER 2530i",
            "printing",
            "192.168.1.102",
            "Building A, Floor 3",
            false,
            vec!["duplex".to_string(), "scan".to_string()],
        ),
        (
            3,
            "3D Printer",
            "Ultimaker S5",
            "idle",
            "192.168.1.103",
            "Building B, Lab",
            false,
            vec!["3d_printing".to_string()],
        ),
    ];

    let printer = printers.iter().find(|(id, _, _, _, _, _, _, _)| *id == printer_id);

    match printer {
        Some((id, name, model, status, ip_address, location, is_default, capabilities)) => {
            Ok(HttpResponse::Ok().json(PrinterDetailResponse {
                success: true,
                data: PrinterDetail {
                    printer_id: *id,
                    name: name.to_string(),
                    model: model.to_string(),
                    status: status.to_string(),
                    ip_address: ip_address.to_string(),
                    location: location.to_string(),
                    is_default: *is_default,
                    capabilities: Some(capabilities.clone()),
                },
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Printer {} not found", printer_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
