// 打印机管理处理器（Phase 56+）
// Phase 53: 增强 JWT 认证和 admin 权限校验
// 包含：打印机列表、详情、作业管理等接口

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::models::jwt::JwtClaims;

/// 打印机类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PrinterType {
    Inkjet,
    Laser,
    Thermal,
    DotMatrix,
    Panel3D,
}

impl std::fmt::Display for PrinterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrinterType::Inkjet => write!(f, "inkjet"),
            PrinterType::Laser => write!(f, "laser"),
            PrinterType::Thermal => write!(f, "thermal"),
            PrinterType::DotMatrix => write!(f, "dot_matrix"),
            PrinterType::Panel3D => write!(f, "3d"),
        }
    }
}

/// 打印机状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PrinterStatus {
    Idle,
    Printing,
    Error,
    Offline,
}

impl std::fmt::Display for PrinterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrinterStatus::Idle => write!(f, "idle"),
            PrinterStatus::Printing => write!(f, "printing"),
            PrinterStatus::Error => write!(f, "error"),
            PrinterStatus::Offline => write!(f, "offline"),
        }
    }
}

/// 打印机能力
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterCapabilities {
    pub color: bool,
    pub duplex: bool,
    pub staple: bool,
    pub scanning: bool,
    pub fax: bool,
}

/// 打印机信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Printer {
    pub id: u64,
    pub name: String,
    pub model: String,
    pub manufacturer: String,
    pub printer_type: String,
    pub status: String,
    pub location: String,
    pub ip_address: String,
    pub is_default: bool,
    pub capabilities: PrinterCapabilities,
    pub created_at: String,
    pub updated_at: String,
}

/// 打印机列表分页参数
#[derive(Debug, Deserialize)]
pub struct PrinterListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
    pub printer_type: Option<String>,
}

impl Default for PrinterListQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            status: None,
            printer_type: None,
        }
    }
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 打印机列表响应
#[derive(Debug, Serialize)]
pub struct PrinterListResponse {
    pub success: bool,
    pub data: Vec<Printer>,
    pub pagination: PrinterPagination,
}

#[derive(Debug, Serialize)]
pub struct PrinterPagination {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 获取打印机列表（Phase 53 增强）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持分页和筛选
pub async fn list_printers(
    jwt_claims: web::Data<JwtClaims>,
    query: web::Query<PrinterListQuery>,
) -> Result<HttpResponse> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access printer list".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20) as u64;
    let status_filter = query.status.as_deref();
    let type_filter = query.printer_type.as_deref();

    // 模拟数据
    let all_printers = vec![
        Printer {
            id: 1,
            name: "HP LaserJet Pro".to_string(),
            model: "HP LaserJet Pro MFP M125nw".to_string(),
            manufacturer: "HP".to_string(),
            printer_type: "laser".to_string(),
            status: "idle".to_string(),
            location: "一楼办公区".to_string(),
            ip_address: "192.168.1.101".to_string(),
            is_default: true,
            capabilities: PrinterCapabilities {
                color: false,
                duplex: true,
                staple: false,
                scanning: true,
                fax: false,
            },
            created_at: "2026-03-15T10:00:00Z".to_string(),
            updated_at: "2026-03-18T15:30:00Z".to_string(),
        },
        Printer {
            id: 2,
            name: "Epson WorkForce".to_string(),
            model: "Epson WorkForce wf-7840".to_string(),
            manufacturer: "Epson".to_string(),
            printer_type: "inkjet".to_string(),
            status: "printing".to_string(),
            location: "二楼文档室".to_string(),
            ip_address: "192.168.1.102".to_string(),
            is_default: false,
            capabilities: PrinterCapabilities {
                color: true,
                duplex: true,
                staple: false,
                scanning: true,
                fax: true,
            },
            created_at: "2026-03-16T14:30:00Z".to_string(),
            updated_at: "2026-03-19T01:00:00Z".to_string(),
        },
        Printer {
            id: 3,
            name: "Brother MFC".to_string(),
            model: "Brother MFC-9340CDW".to_string(),
            manufacturer: "Brother".to_string(),
            printer_type: "laser".to_string(),
            status: "offline".to_string(),
            location: "三楼会议室".to_string(),
            ip_address: "192.168.1.103".to_string(),
            is_default: false,
            capabilities: PrinterCapabilities {
                color: true,
                duplex: true,
                staple: true,
                scanning: true,
                fax: true,
            },
            created_at: "2026-03-17T09:15:00Z".to_string(),
            updated_at: "2026-03-17T09:15:00Z".to_string(),
        },
        Printer {
            id: 4,
            name: "Canon imageCLASS".to_string(),
            model: "Canon imageCLASS D1220".to_string(),
            manufacturer: "Canon".to_string(),
            printer_type: "laser".to_string(),
            status: "idle".to_string(),
            location: "四楼行政部".to_string(),
            ip_address: "192.168.1.104".to_string(),
            is_default: false,
            capabilities: PrinterCapabilities {
                color: false,
                duplex: false,
                staple: false,
                scanning: true,
                fax: false,
            },
            created_at: "2026-03-18T11:00:00Z".to_string(),
            updated_at: "2026-03-18T11:00:00Z".to_string(),
        },
    ];

    // 过滤
    let printers: Vec<Printer> = all_printers
        .into_iter()
        .filter(|p| {
            if let Some(status) = status_filter {
                if p.status != status {
                    return false;
                }
            }
            if let Some(type_) = type_filter {
                if p.printer_type != type_ {
                    return false;
                }
            }
            true
        })
        .collect();

    let total = printers.len() as u64;
    let start = (page - 1) * limit;
    let end = start + limit;

    let paginated_printers: Vec<Printer> = printers
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let idx = i as u64;
            if idx >= start && idx < end {
                Some(p)
            } else {
                None
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(PrinterListResponse {
        success: true,
        data: paginated_printers,
        pagination: PrinterPagination {
            page,
            limit,
            total,
            total_pages: (total + limit - 1) / limit,
        },
    }))
}

/// 获取打印机详情（Phase 54 增强）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回打印机详细信息
/// - 打印机不存在返回 404 Not Found
pub async fn get_printer(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access printer details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let id = path.into_inner();

    let mock_printers = vec![
        Printer {
            id: 1,
            name: "HP LaserJet Pro".to_string(),
            model: "HP LaserJet Pro MFP M125nw".to_string(),
            manufacturer: "HP".to_string(),
            printer_type: "laser".to_string(),
            status: "idle".to_string(),
            location: "一楼办公区".to_string(),
            ip_address: "192.168.1.101".to_string(),
            is_default: true,
            capabilities: PrinterCapabilities {
                color: false,
                duplex: true,
                staple: false,
                scanning: true,
                fax: false,
            },
            created_at: "2026-03-15T10:00:00Z".to_string(),
            updated_at: "2026-03-18T15:30:00Z".to_string(),
        },
        Printer {
            id: 2,
            name: "Epson WorkForce".to_string(),
            model: "Epson WorkForce wf-7840".to_string(),
            manufacturer: "Epson".to_string(),
            printer_type: "inkjet".to_string(),
            status: "printing".to_string(),
            location: "二楼文档室".to_string(),
            ip_address: "192.168.1.102".to_string(),
            is_default: false,
            capabilities: PrinterCapabilities {
                color: true,
                duplex: true,
                staple: false,
                scanning: true,
                fax: true,
            },
            created_at: "2026-03-16T14:30:00Z".to_string(),
            updated_at: "2026-03-19T01:00:00Z".to_string(),
        },
    ];

    match mock_printers.iter().find(|p| p.id == id) {
        Some(printer) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": printer
        }))),
        None => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Printer {} not found", id),
            code: "NOT_FOUND".to_string(),
        })),
    }
}

/// 更新打印机配置
pub async fn update_printer(
    path: web::Path<u64>,
    payload: web::Json<UpdatePrinterRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证打印机存在
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Printer {} not found", id)
        })));
    }

    let updated_at = Utc::now().to_rfc3339();

    Ok(HttpResponse::Ok().json(Printer {
        id,
        name: payload.name.clone().unwrap_or_else(|| "unknown".to_string()),
        model: payload.model.clone().unwrap_or_else(|| "unknown".to_string()),
        manufacturer: payload.manufacturer.clone().unwrap_or_else(|| "unknown".to_string()),
        printer_type: payload.printer_type.as_deref().unwrap_or("laser").to_string(),
        status: payload.status.as_deref().unwrap_or("idle").to_string(),
        location: payload.location.clone().unwrap_or_else(|| "unknown".to_string()),
        ip_address: payload.ip_address.clone().unwrap_or_else(|| "0.0.0.0".to_string()),
        is_default: payload.is_default.unwrap_or(false),
        capabilities: PrinterCapabilities {
            color: payload.capabilities.as_ref().map(|c| c.color).unwrap_or(true),
            duplex: payload.capabilities.as_ref().map(|c| c.duplex).unwrap_or(true),
            staple: payload.capabilities.as_ref().map(|c| c.staple).unwrap_or(false),
            scanning: payload.capabilities.as_ref().map(|c| c.scanning).unwrap_or(true),
            fax: payload.capabilities.as_ref().map(|c| c.fax).unwrap_or(false),
        },
        created_at: "2026-03-15T10:00:00Z".to_string(),
        updated_at,
    }))
}

/// 删除打印机
pub async fn delete_printer(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证打印机存在
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Printer {} not found", id)
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Printer {} deleted", id)
    })))
}

/// 创建打印任务
pub async fn create_print_job(
    path: web::Path<u64>,
    payload: web::Json<CreatePrintJobRequest>,
) -> Result<HttpResponse> {
    let printer_id = path.into_inner();

    // 验证打印机是否存在（简化模拟）
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&printer_id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Printer {} not found", printer_id)
        })));
    }

    // 验证打印机状态
    if let Some(status) = payload.status.as_deref() {
        if status == "offline" {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "message": format!("Printer {} is offline", printer_id)
            })));
        }
    }

    Ok(HttpResponse::Created().json(PrintJobResponse {
        success: true,
        job_id: 100 + printer_id,
        status: "queued".to_string(),
        message: format!("Print job created for printer {}", printer_id),
        estimated_completion: "2026-03-19T02:35:00Z".to_string(),
        print_settings: serde_json::json!({
            "copies": payload.copies.unwrap_or(1),
            "color": payload.color.unwrap_or(false),
            "duplex": payload.duplex.unwrap_or(false),
        }),
    }))
}

/// 取消打印任务
pub async fn cancel_print_job(
    path: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let (_printer_id, job_id) = path.into_inner();

    // 简化模拟：验证 job_id
    let valid_jobs = vec![101, 102, 103];
    if !valid_jobs.contains(&job_id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Job {} not found", job_id)
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Job {} cancelled", job_id)
    })))
}

/// 获取打印机状态
pub async fn get_printer_status(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    let mock_status = vec![
        (1, "idle".to_string(), 0),
        (2, "printing".to_string(), 3),
        (3, "offline".to_string(), 0),
        (4, "idle".to_string(), 1),
    ];

    match mock_status.iter().find(|(p_id, _, _)| *p_id == id) {
        Some((_, status, queue_size)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "id": id,
            "status": status,
            "queue_size": queue_size
        }))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Printer {} not found", id)
        }))),
    }
}

/// 获取打印机队列状态
pub async fn get_queue_status() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(vec![
        serde_json::json!({
            "job_id": 101,
            "printer_id": 1,
            "filename": "document.pdf",
            "status": "printing",
            "created_at": "2026-03-18T20:00:00Z"
        }),
        serde_json::json!({
            "job_id": 102,
            "printer_id": 2,
            "filename": "report.docx",
            "status": "queued",
            "created_at": "2026-03-19T01:30:00Z"
        }),
        serde_json::json!({
            "job_id": 103,
            "printer_id": 4,
            "filename": "presentation.pptx",
            "status": "queued",
            "created_at": "2026-03-19T01:45:00Z"
        }),
    ]))
}

/// 打印测试页
pub async fn print_test_page(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 验证打印机存在
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Printer {} not found", id)
        })));
    }

    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "message": format!("Test page sent to printer {}", id),
        "job_id": 200 + id,
        "estimated_completion": "2026-03-19T02:45:00Z"
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdatePrinterRequest {
    pub name: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub printer_type: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub ip_address: Option<String>,
    pub is_default: Option<bool>,
    pub capabilities: Option<PrinterCapabilities>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePrintJobRequest {
    pub file_path: String,
    pub copies: Option<u32>,
    pub color: Option<bool>,
    pub duplex: Option<bool>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PrintJobResponse {
    pub success: bool,
    pub job_id: u64,
    pub status: String,
    pub message: String,
    pub estimated_completion: String,
    pub print_settings: serde_json::Value,
}
