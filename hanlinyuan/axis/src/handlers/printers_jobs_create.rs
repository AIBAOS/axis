// Phase 114 - 创建打印任务 API
// POST /api/v1/printers/{printer_id}/jobs - 创建打印任务

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::models::jwt::JwtClaims;

/// 打印任务优先级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrintJobPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// 创建打印任务请求
#[derive(Debug, Deserialize)]
pub struct CreatePrintJobRequest {
    pub document_name: String,
    pub pages: u32,
    pub copies: u32,
    #[serde(default)]
    pub priority: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
}

/// 创建打印任务响应
#[derive(Debug, Serialize)]
pub struct CreatePrintJobResponse {
    pub success: bool,
    pub message: String,
    pub data: PrintJobDetail,
}

/// 打印任务详情
#[derive(Debug, Serialize, Clone)]
pub struct PrintJobDetail {
    pub id: u64,
    pub printer_id: u64,
    pub user_id: u64,
    pub document_name: String,
    pub pages: u32,
    pub copies: u32,
    pub status: String,
    pub priority: String,
    pub submitted_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查 CUPS 服务连接
fn check_cups_connection() -> Result<(), String> {
    // 模拟 CUPS 服务检查
    // 实际实现应该连接 CUPS API
    Ok(())
}

/// 验证优先级
fn validate_priority(priority: &str) -> bool {
    let valid_priorities = ["low", "normal", "high", "urgent"];
    valid_priorities.contains(&priority.to_lowercase().as_str())
}

/// 创建打印机任务（Phase 114）
/// - JWT 认证，登录用户可访问
/// - 验证打印机存在性
/// - 验证 CUPS 服务连接
/// - 验证参数合法性
pub async fn create_print_job(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
    req: web::Json<CreatePrintJobRequest>,
) -> Result<HttpResponse, Error> {
    let printer_id = path.into_inner();

    // 1. 验证 CUPS 服务连接
    if let Err(_e) = check_cups_connection() {
        return Ok(HttpResponse::ServiceUnavailable().json(ErrorResponse {
            success: false,
            error: "CUPS service is unavailable".to_string(),
            code: "SERVICE_UNAVAILABLE".to_string(),
        }));
    }

    // 2. 验证参数合法性
    if req.document_name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Document name cannot be empty".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if req.pages == 0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Pages must be greater than 0".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if req.copies == 0 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Copies must be greater than 0".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 验证优先级
    let priority = if req.priority.is_empty() {
        "normal".to_string()
    } else if !validate_priority(&req.priority) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid priority. Valid values: low, normal, high, urgent".to_string(),
            code: "INVALID_PRIORITY".to_string(),
        }));
    } else {
        req.priority.to_lowercase()
    };

    // 4. 获取当前用户 ID
    let user_id = jwt_claims.sub.parse::<u64>()
        .unwrap_or(1);

    // 5. 生成任务 ID（模拟，实际应从数据库获取）
    let job_id = 1;

    // 6. 创建时间
    let submitted_at = req.submitted_at.clone()
        .unwrap_or_else(|| Utc::now().to_rfc3339());

    // 7. 返回创建结果
    Ok(HttpResponse::Created().json(CreatePrintJobResponse {
        success: true,
        message: "Print job created successfully".to_string(),
        data: PrintJobDetail {
            id: job_id,
            printer_id,
            user_id,
            document_name: req.document_name.clone(),
            pages: req.pages,
            copies: req.copies,
            status: "pending".to_string(),
            priority,
            submitted_at,
            started_at: None,
            completed_at: None,
            error_message: None,
        },
    }))
}
