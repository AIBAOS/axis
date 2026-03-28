// Phase 113 - 打印机任务详情 API
// GET /api/v1/printers/{printer_id}/jobs/{job_id} — 获取打印机任务详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 打印任务详情
#[derive(Serialize)]
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

/// 打印任务详情响应
#[derive(Serialize)]
pub struct PrintJobDetailResponse {
    pub success: bool,
    pub data: PrintJobDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取打印任务详情（Phase 113）
/// - JWT 认证，登录用户可访问
/// - 验证打印机存在性 (404)
/// - 验证任务 ID 存在性 (404)
/// - 验证 CUPS 服务连接状态 (503)
pub async fn get_print_job_detail(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
    _rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 登录用户可访问
    // 已通过 JWT 验证，说明是登录用户

    let (printer_id, job_id) = path.into_inner();

    // 3. 模拟 CUPS 服务连接检查
    let cups_available = true; // 实际实现中应该检查 CUPS 服务是否可用
    if !cups_available {
        return Ok(HttpResponse::ServiceUnavailable().json(ErrorResponse {
            success: false,
            error: "CUPS service is not available".to_string(),
            code: "SERVICE_UNAVAILABLE".to_string(),
        }));
    }

    // 4. 模拟打印机数据（验证打印机存在性）
    let mock_printers = vec![1u64, 2, 3, 4];
    if !mock_printers.contains(&printer_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Printer {} not found", printer_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 模拟打印任务数据
    let mock_jobs = vec![
        (1u64, 1u64, "report.pdf", "admin", 5u32, 1u32, "completed", 0u32, 1711440000u64, Some(1711440300u64), None::<String>),
        (2u64, 1u64, "photo.jpg", "user1", 1u32, 3u32, "printing", 5u32, 1711440500u64, None, None),
        (3u64, 2u64, "document.docx", "user2", 10u32, 2u32, "pending", 10u32, 1711440600u64, None, None),
        (4u64, 2u64, "spreadsheet.xlsx", "admin", 3u32, 1u32, "failed", 5u32, 1711440700u64, None, Some("Paper jam".to_string())),
        (5u64, 3u64, "presentation.pptx", "user1", 20u32, 1u32, "canceled", 5u32, 1711440800u64, None, Some("Canceled by user".to_string())),
    ];

    // 6. 查找打印任务
    let job = mock_jobs.into_iter().find(|(jid, pid, _, _, _, _, _, _, _, _, _)| {
        *jid == job_id && *pid == printer_id
    });

    match job {
        Some((id, pid, doc_name, user, pages, copies, state, priority, submitted, completed, error)) => {
            let job_detail = PrintJobDetail {
                id,
                printer_id: pid,
                document_name: doc_name.to_string(),
                user: user.to_string(),
                pages,
                copies,
                state: state.to_string(),
                priority,
                submitted_at: submitted,
                completed_at: completed,
                error_message: error,
            };

            Ok(HttpResponse::Ok().json(PrintJobDetailResponse {
                success: true,
                data: job_detail,
            }))
        }
        None => {
            // 7. 任务不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Print job {} not found for printer {}", job_id, printer_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
