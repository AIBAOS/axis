// Phase 118: 打印机统计信息 API
// GET /api/v1/printers/stats — 获取打印机统计信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 打印机状态统计
#[derive(Serialize, Clone)]
pub struct PrinterStatusStats {
    pub idle: u32,
    pub printing: u32,
    pub error: u32,
    pub offline: u32,
}

/// 打印任务统计
#[derive(Serialize, Clone)]
pub struct JobStats {
    pub pending: u32,
    pub printing: u32,
    pub completed: u32,
    pub failed: u32,
    pub canceled: u32,
}

/// 打印机统计信息
#[derive(Serialize)]
pub struct PrinterStats {
    pub total_printers: u32,
    pub by_status: PrinterStatusStats,
    pub total_jobs: u32,
    pub jobs_by_status: JobStats,
}

/// 统计信息响应
#[derive(Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub data: PrinterStats,
}

/// 打印机统计信息（Phase 118）
/// - JWT 认证，登录用户可访问
/// - 返回打印机和打印任务统计信息
pub async fn get_printer_stats(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 模拟打印机数据
    let mock_printers = vec![
        (1, "HP LaserJet Pro", "idle"),
        (2, "Canon PIXMA", "printing"),
        (3, "Epson WorkForce", "error"),
        (4, "Brother MFC", "idle"),
        (5, "Xerox VersaLink", "offline"),
    ];

    // 4. 模拟打印任务数据
    let mock_jobs = vec![
        (1, "pending"),
        (2, "printing"),
        (3, "completed"),
        (4, "completed"),
        (5, "failed"),
        (6, "canceled"),
        (7, "pending"),
        (8, "printing"),
    ];

    // 5. 统计打印机状态
    let mut by_status = PrinterStatusStats {
        idle: 0,
        printing: 0,
        error: 0,
        offline: 0,
    };

    for (_, _, status) in &mock_printers {
        match *status {
            "idle" => by_status.idle += 1,
            "printing" => by_status.printing += 1,
            "error" => by_status.error += 1,
            "offline" => by_status.offline += 1,
            _ => {}
        }
    }

    // 6. 统计打印任务状态
    let mut jobs_by_status = JobStats {
        pending: 0,
        printing: 0,
        completed: 0,
        failed: 0,
        canceled: 0,
    };

    for (_, status) in &mock_jobs {
        match *status {
            "pending" => jobs_by_status.pending += 1,
            "printing" => jobs_by_status.printing += 1,
            "completed" => jobs_by_status.completed += 1,
            "failed" => jobs_by_status.failed += 1,
            "canceled" => jobs_by_status.canceled += 1,
            _ => {}
        }
    }

    // 7. 构建响应
    let stats = PrinterStats {
        total_printers: mock_printers.len() as u32,
        by_status,
        total_jobs: mock_jobs.len() as u32,
        jobs_by_status,
    };

    Ok(HttpResponse::Ok().json(StatsResponse {
        success: true,
        data: stats,
    }))
}
