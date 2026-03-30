// Phase 115: 更新打印任务 API
// PUT /api/v1/printers/{printer_id}/jobs/{job_id} — 更新打印任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 更新打印任务请求
#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub priority: Option<u32>,
    pub state: Option<String>,
}

/// 打印任务详情
#[derive(Serialize, Clone)]
pub struct PrintJobDetail {
    pub id: u64,
    pub printer_id: u64,
    pub document_name: String,
    pub user: String,
    pub pages: u32,
    pub copies: u32,
    pub state: String,
    pub priority: u32,
    pub submitted_at: u64,
    pub completed_at: Option<u64>,
    pub error_message: Option<String>,
}

/// 更新任务响应
#[derive(Serialize)]
pub struct UpdateJobResponse {
    pub success: bool,
    pub message: String,
    pub data: PrintJobDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 更新打印任务（Phase 115）
/// - JWT 认证，登录用户可访问
/// - 路径参数：printer_id, job_id
/// - 请求体：priority/state (至少一个)
/// - 验证打印机存在性（404）
/// - 验证任务存在性（404）
/// - 验证参数合法性（400）
pub async fn update_job(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
    payload: web::Json<UpdateJobRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let (printer_id, job_id) = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并获取用户信息
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    let _username = claims.sub;

    // 3. 验证至少提供一个字段
    if payload.priority.is_none() && payload.state.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one field (priority or state) must be provided".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 验证 priority 合法性
    if let Some(priority) = payload.priority {
        if priority == 0 || priority > 5 {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "priority must be between 1 and 5".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    // 5. 验证 state 合法性 by cloning to avoid move
    let state_clone = payload.state.clone();
    if let Some(ref state) = state_clone {
        let valid_states = ["pending", "printing", "completed", "failed", "canceled"];
        if !valid_states.contains(&state.as_str()) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid state '{}'. Valid states: {}", state, valid_states.join(", ")),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    // 6. 模拟打印机数据验证
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&printer_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Printer {} not found", printer_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 7. 模拟打印任务数据
    let mut mock_jobs = vec![
        PrintJobDetail {
            id: 1,
            printer_id: 1,
            document_name: "report.pdf".to_string(),
            user: "admin".to_string(),
            pages: 5,
            copies: 2,
            state: "pending".to_string(),
            priority: 3,
            submitted_at: 1711500000,
            completed_at: None,
            error_message: None,
        },
        PrintJobDetail {
            id: 2,
            printer_id: 1,
            document_name: "photo.jpg".to_string(),
            user: "user1".to_string(),
            pages: 1,
            copies: 3,
            state: "printing".to_string(),
            priority: 2,
            submitted_at: 1711500600,
            completed_at: None,
            error_message: None,
        },
    ];

    // 8. 查找任务
    let job = mock_jobs.iter_mut().find(|j| j.id == job_id && j.printer_id == printer_id);

    match job {
        Some(j) => {
            // 9. 更新任务
            if let Some(priority) = payload.priority {
                j.priority = priority;
            }

            if let Some(state) = state_clone {
                j.state = state;
                if j.state == "completed" || j.state == "failed" || j.state == "canceled" {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    j.completed_at = Some(now);
                }
            }

            Ok(HttpResponse::Ok().json(UpdateJobResponse {
                success: true,
                message: "Print job updated successfully".to_string(),
                data: j.clone(),
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Job {} not found for printer {}", job_id, printer_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
