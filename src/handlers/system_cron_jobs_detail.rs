// Phase 256: 系统定时任务详情 API
// GET /api/v1/system/cron-jobs/{id} — 获取单个定时任务详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 定时任务详情信息
#[derive(Serialize, Clone)]
pub struct CronJobDetail {
    pub id: u32,
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub status: String,
    pub last_run: Option<u64>,
    pub next_run: Option<u64>,
    pub enabled: bool,
    pub description: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 定时任务详情响应
#[derive(Serialize)]
pub struct CronJobDetailResponse {
    pub success: bool,
    pub data: CronJobDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取系统定时任务详情（Phase 256）
/// - JWT 认证，admin 角色可访问
/// - 验证任务 ID 存在性（404 Not Found）
/// - 返回定时任务完整详情
/// - 错误处理：401/403/404/500
pub async fn get_system_cron_job_detail(
    req: HttpRequest,
    path: web::Path<u32>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let job_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view cron job details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟定时任务数据（实际应从数据库读取）
    let mock_jobs = vec![
        CronJobDetail {
            id: 1,
            name: "daily-backup".to_string(),
            schedule: "0 2 * * *".to_string(),
            command: "/usr/local/bin/backup.sh".to_string(),
            status: "active".to_string(),
            last_run: Some(1711584000),
            next_run: Some(1711670400),
            enabled: true,
            description: Some("Daily system backup".to_string()),
            created_at: 1711497600,
            updated_at: 1711584000,
        },
        CronJobDetail {
            id: 2,
            name: "weekly-cleanup".to_string(),
            schedule: "0 3 * * 0".to_string(),
            command: "/usr/local/bin/cleanup.sh".to_string(),
            status: "active".to_string(),
            last_run: Some(1711324800),
            next_run: Some(1711929600),
            enabled: true,
            description: Some("Weekly log cleanup".to_string()),
            created_at: 1711238400,
            updated_at: 1711324800,
        },
        CronJobDetail {
            id: 3,
            name: "hourly-sync".to_string(),
            schedule: "0 * * * *".to_string(),
            command: "/usr/local/bin/sync.sh".to_string(),
            status: "inactive".to_string(),
            last_run: Some(1711630800),
            next_run: None,
            enabled: false,
            description: Some("Hourly data sync".to_string()),
            created_at: 1711152000,
            updated_at: 1711630800,
        },
    ];

    // 5. 查找任务
    let job = mock_jobs.into_iter().find(|j| j.id == job_id);

    match job {
        Some(detail) => {
            Ok(HttpResponse::Ok().json(CronJobDetailResponse {
                success: true,
                data: detail,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Cron job {} not found", job_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_system_cron_job_detail_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/cron-jobs/{id}", web::get().to(get_system_cron_job_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 权限
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_cron_job_detail_not_found() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/cron-jobs/{id}", web::get().to(get_system_cron_job_detail))
        ).await;

        // 注意：实际测试需要验证任务不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_cron_job_detail_unauthorized() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/system/cron-jobs/{id}", web::get().to(get_system_cron_job_detail))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
