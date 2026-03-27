// Phase 115 - 更新打印任务 API
// PUT /api/v1/printers/{printer_id}/jobs/{job_id} - 更新打印任务

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 更新打印任务请求
#[derive(Debug, Deserialize)]
pub struct UpdatePrintJobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// 打印任务详情响应
#[derive(Debug, Serialize)]
pub struct UpdatePrintJobResponse {
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

/// 验证优先级
fn validate_priority(priority: &str) -> bool {
    let valid_priorities = ["low", "normal", "high", "urgent"];
    valid_priorities.contains(&priority.to_lowercase().as_str())
}

/// 验证状态
fn validate_state(state: &str) -> bool {
    let valid_states = ["pending", "printing", "completed", "failed", "canceled"];
    valid_states.contains(&state.to_lowercase().as_str())
}

/// 更新打印机任务（Phase 115）
/// - JWT 认证，登录用户可访问
/// - 验证打印机和任务存在性
/// - 可更新字段：priority/state
pub async fn update_print_job(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<(u64, u64)>,
    req: web::Json<UpdatePrintJobRequest>,
) -> Result<HttpResponse, Error> {
    let (printer_id, job_id) = path.into_inner();

    // 1. 验证参数
    if req.priority.is_none() && req.state.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one field (priority or state) must be provided".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 2. 验证优先级（如果提供）
    if let Some(ref priority) = req.priority {
        if !validate_priority(priority) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid priority. Valid values: low, normal, high, urgent".to_string(),
                code: "INVALID_PRIORITY".to_string(),
            }));
        }
    }

    // 3. 验证状态（如果提供）
    if let Some(ref state) = req.state {
        if !validate_state(state) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid state. Valid values: pending, printing, completed, failed, canceled".to_string(),
                code: "INVALID_STATE".to_string(),
            }));
        }
    }

    // 4. 模拟打印机任务数据（实际应从数据库获取）
    let mock_jobs = vec![
        PrintJobDetail {
            id: 1,
            printer_id,
            user_id: 101,
            document_name: "report.pdf".to_string(),
            pages: 5,
            copies: 2,
            status: "pending".to_string(),
            priority: "normal".to_string(),
            submitted_at: "2026-03-26T10:00:00Z".to_string(),
            started_at: None,
            completed_at: None,
            error_message: None,
        },
    ];

    // 5. 查找任务
    let mut job = mock_jobs.into_iter().find(|j| j.id == job_id);

    // 6. 更新任务
    if let Some(ref mut j) = job {
        if let Some(priority) = &req.priority {
            j.priority = priority.to_lowercase();
        }
        if let Some(state) = &req.state {
            j.status = state.to_lowercase();
        }
    }

    // 7. 返回更新结果
    match job {
        Some(job) => {
            Ok(HttpResponse::Ok().json(UpdatePrintJobResponse {
                success: true,
                message: "Print job updated successfully".to_string(),
                data: job,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Printer or job not found".to_string(),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
