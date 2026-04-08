// Phase 126: 打印机列表 API
// GET /api/v1/printers — 获取打印机列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

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

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PrinterListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 打印机列表响应
#[derive(Serialize)]
pub struct PrintersListResponse {
    pub success: bool,
    pub data: Vec<PrinterInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取打印机列表（Phase 126）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回字段：printer_id/name/model/status/ip_address/location/is_default
/// - 支持分页查询 (page, per_page 参数)
pub async fn list_printers(
    req: HttpRequest,
    query: web::Query<PrinterListQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
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
            error: "Only admin users can access printer list".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析分页参数
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let per_page = query.per_page.unwrap_or(20);

    // 5. 模拟打印机数据
    let all_printers = vec![
        PrinterInfo {
            printer_id: 1,
            name: "Office Printer 1".to_string(),
            model: "HP LaserJet Pro M404n".to_string(),
            status: "idle".to_string(),
            ip_address: "192.168.1.101".to_string(),
            location: "Building A, Floor 2".to_string(),
            is_default: true,
        },
        PrinterInfo {
            printer_id: 2,
            name: "Office Printer 2".to_string(),
            model: "Canon imageRUNNER 2530i".to_string(),
            status: "printing".to_string(),
            ip_address: "192.168.1.102".to_string(),
            location: "Building A, Floor 3".to_string(),
            is_default: false,
        },
        PrinterInfo {
            printer_id: 3,
            name: "3D Printer".to_string(),
            model: "Ultimaker S5".to_string(),
            status: "idle".to_string(),
            ip_address: "192.168.1.103".to_string(),
            location: "Building B, Lab".to_string(),
            is_default: false,
        },
        PrinterInfo {
            printer_id: 4,
            name: "Receipt Printer".to_string(),
            model: "Epson TM-T88VI".to_string(),
            status: "offline".to_string(),
            ip_address: "192.168.1.104".to_string(),
            location: "Building A, Floor 1".to_string(),
            is_default: false,
        },
        PrinterInfo {
            printer_id: 5,
            name: "Color Printer".to_string(),
            model: "Xerox VersaLink C405".to_string(),
            status: "error".to_string(),
            ip_address: "192.168.1.105".to_string(),
            location: "Building A, Floor 2".to_string(),
            is_default: false,
        },
    ];

    let total = all_printers.len() as u64;
    let total_pages = ((total + per_page as u64 - 1) / per_page as u64) as u32;
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(all_printers.len());

    let printers: Vec<PrinterInfo> = if start < all_printers.len() {
        all_printers[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(PrintersListResponse {
        success: true,
        data: printers,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
