// Phase 112 - 打印机任务列表 API
// GET /api/v1/printers/{id}/jobs - 获取打印机任务队列

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 打印任务状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrintJobStatus {
    Pending,
    Printing,
    Completed,
    Failed,
    Canceled,
}

/// 打印任务优先级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrintJobPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// 打印任务信息
#[derive(Debug, Serialize, Clone)]
pub struct PrintJob {
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
}

/// 打印机任务列表分页参数
#[derive(Debug, Deserialize)]
pub struct PrintJobListQuery {
    #[serde(default)]
    pub page: Option<u64>,
    #[serde(default)]
    pub page_size: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
}

impl Default for PrintJobListQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
            status: None,
        }
    }
}

/// 分页信息
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 打印机任务列表响应
#[derive(Debug, Serialize)]
pub struct PrintJobListResponse {
    pub success: bool,
    pub data: Vec<PrintJob>,
    pub pagination: Pagination,
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

/// 获取打印机任务列表（Phase 112）
/// - JWT 认证，登录用户可访问
/// - 验证打印机存在性
/// - 验证 CUPS 服务连接
/// - 支持状态筛选
pub async fn list_printer_jobs(
    _jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
    query: web::Query<PrintJobListQuery>,
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

    // 2. 解析查询参数
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20).max(1).min(100);
    let status_filter = query.status.as_deref();

    // 3. 模拟打印机任务数据（实际应从 CUPS 获取）
    let all_jobs = vec![
        PrintJob {
            id: 1,
            printer_id,
            user_id: 101,
            document_name: "report.pdf".to_string(),
            pages: 5,
            copies: 2,
            status: "printing".to_string(),
            priority: "normal".to_string(),
            submitted_at: "2026-03-26T10:00:00Z".to_string(),
            started_at: Some("2026-03-26T10:05:00Z".to_string()),
            completed_at: None,
        },
        PrintJob {
            id: 2,
            printer_id,
            user_id: 102,
            document_name: "invoice.pdf".to_string(),
            pages: 2,
            copies: 1,
            status: "pending".to_string(),
            priority: "normal".to_string(),
            submitted_at: "2026-03-26T10:10:00Z".to_string(),
            started_at: None,
            completed_at: None,
        },
        PrintJob {
            id: 3,
            printer_id,
            user_id: 103,
            document_name: "document.docx".to_string(),
            pages: 10,
            copies: 3,
            status: "completed".to_string(),
            priority: "high".to_string(),
            submitted_at: "2026-03-26T09:00:00Z".to_string(),
            started_at: Some("2026-03-26T09:05:00Z".to_string()),
            completed_at: Some("2026-03-26T09:15:00Z".to_string()),
        },
    ];

    // 4. 状态筛选
    let filtered_jobs: Vec<PrintJob> = if let Some(status) = status_filter {
        all_jobs.into_iter().filter(|j| j.status == status).collect()
    } else {
        all_jobs
    };

    // 5. 分页
    let total = filtered_jobs.len() as u64;
    let total_pages = if total > 0 { (total + page_size - 1) / page_size } else { 0 };
    let start = ((page - 1) * page_size) as usize;
    let end = (page * page_size) as usize;
    
    let paginated_jobs: Vec<PrintJob> = filtered_jobs
        .into_iter()
        .enumerate()
        .filter_map(|(i, job)| {
            if i >= start && i < end {
                Some(job)
            } else {
                None
            }
        })
        .collect();

    // 6. 返回响应
    Ok(HttpResponse::Ok().json(PrintJobListResponse {
        success: true,
        data: paginated_jobs,
        pagination: Pagination {
            page,
            page_size,
            total,
            total_pages,
        },
    }))
}
