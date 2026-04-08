// Phase 116: 取消打印任务 API
// DELETE /api/v1/printers/{printer_id}/jobs/{job_id} — 取消/删除打印任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 删除任务响应
#[derive(Serialize)]
pub struct DeleteJobResponse {
    pub success: bool,
    pub message: String,
    pub data: DeletedJobInfo,
}

/// 已删除任务信息
#[derive(Serialize, Clone)]
pub struct DeletedJobInfo {
    pub id: u64,
    pub printer_id: u64,
    pub document_name: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 取消打印任务（Phase 116）
/// - JWT 认证，登录用户可访问
/// - 路径参数：printer_id, job_id
/// - 验证打印机存在性（404）
/// - 验证任务存在性（404）
/// - 验证用户权限（403，只能取消自己的任务，admin 除外）
/// - 验证 CUPS 服务连接（503）
/// - 删除成功返回 200 OK
pub async fn delete_job(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
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

    let username = claims.sub;
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");

    // 3. 模拟 CUPS 服务连接检查
    let cups_available = true; // 模拟 CUPS 服务可用
    if !cups_available {
        return Ok(HttpResponse::ServiceUnavailable().json(ErrorResponse {
            success: false,
            error: "CUPS service is unavailable".to_string(),
            code: "SERVICE_UNAVAILABLE".to_string(),
        }));
    }

    // 4. 模拟打印机数据验证
    let mock_printers = vec![1, 2, 3, 4];
    if !mock_printers.contains(&printer_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Printer {} not found", printer_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 模拟打印任务数据
    let mock_jobs = vec![
        (1, 1, "report.pdf", "admin"),
        (2, 1, "photo.jpg", "user1"),
        (3, 2, "document.docx", "admin"),
        (4, 3, "failed.pdf", "user2"),
    ];

    // 6. 查找任务
    let job = mock_jobs.iter().find(|(jid, pid, _, _)| *jid == job_id && *pid == printer_id);

    match job {
        Some((id, _, document_name, task_user)) => {
            // 7. 验证用户权限（admin 可操作任意，普通用户只能操作自己的）
            if !is_admin && task_user != &username {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "You can only cancel your own print jobs".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 8. 模拟删除操作
            Ok(HttpResponse::Ok().json(DeleteJobResponse {
                success: true,
                message: "Print job canceled successfully".to_string(),
                data: DeletedJobInfo {
                    id: *id,
                    printer_id,
                    document_name: document_name.to_string(),
                },
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
