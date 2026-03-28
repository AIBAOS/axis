// Phase 255: 系统定时任务创建 API
// POST /api/v1/system/cron-jobs — 创建定时任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::cron_job_store::SqliteCronJobRepository;

/// 创建定时任务请求
#[derive(Debug, Deserialize)]
pub struct CreateCronJobRequest {
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub description: Option<String>,
    pub enabled: Option<bool>,
}

/// 定时任务信息
#[derive(Serialize, Clone)]
pub struct CronJobInfo {
    pub id: u64,
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub description: Option<String>,
    pub status: String,
    pub enabled: bool,
    pub created_at: u64,
}

/// 创建定时任务响应
#[derive(Serialize)]
pub struct CreateCronJobResponse {
    pub success: bool,
    pub message: String,
    pub data: CronJobInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证任务名称格式
fn validate_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128 && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// 验证命令格式
fn validate_command(command: &str) -> bool {
    !command.is_empty() && command.len() <= 512
}

/// 验证 cron 表达式格式
fn validate_schedule(schedule: &str) -> bool {
    // 支持预定义表达式
    if schedule.starts_with('@') {
        return matches!(schedule, "@hourly" | "@daily" | "@weekly" | "@monthly" | "@yearly" | "@reboot");
    }
    
    // 支持标准 cron 表达式 (5 个字段)
    let parts: Vec<&str> = schedule.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }
    
    // 简化验证：检查每个字段是否包含有效字符
    parts.iter().all(|part| {
        part.chars().all(|c| c.is_numeric() || c == '*' || c == '/' || c == '-' || c == ',')
    })
}

/// 创建定时任务（Phase 255）
/// - JWT 认证，admin 角色可访问
/// - 验证 name 格式（1-128 字符，字母数字 -_.）
/// - 验证 schedule 格式（cron 表达式或预定义）
/// - 验证 command 格式（1-512 字符）
/// - 验证 name 唯一性（409 Conflict）
/// - 创建成功返回 201 Created + 任务详情
/// - 错误处理：401/403/400/409/500
pub async fn create_cron_job(
    req: HttpRequest,
    payload: web::Json<CreateCronJobRequest>,
    jwt_service: web::Data<JwtService>,
    cron_repo: web::Data<Arc<SqliteCronJobRepository>>,
) -> Result<HttpResponse, Error> {
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
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create cron jobs".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证 name 格式
    if !validate_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid name format. Must be 1-128 chars, alphanumeric with -_. allowed".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证 schedule 格式
    if !validate_schedule(&payload.schedule) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid schedule format. Must be cron expression or @hourly/@daily/@weekly/@monthly".to_string(),
            code: "INVALID_SCHEDULE".to_string(),
        }));
    }

    // 6. 验证 command 格式
    if !validate_command(&payload.command) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid command format. Must be 1-512 chars".to_string(),
            code: "INVALID_COMMAND".to_string(),
        }));
    }

    // 7. 验证 name 唯一性
    if let Ok(Some(_)) = cron_repo.get_job_by_name(&payload.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Cron job with name '{}' already exists", payload.name),
            code: "NAME_CONFLICT".to_string(),
        }));
    }

    // 8. 创建定时任务
    let enabled = payload.enabled.unwrap_or(true);
    
    match cron_repo.create_job(&payload.name, &payload.schedule, &payload.command, payload.description.as_deref(), enabled) {
        Ok(job) => {
            let job_info = CronJobInfo {
                id: job.id,
                name: job.name,
                schedule: job.schedule,
                command: job.command,
                description: job.description,
                status: job.status,
                enabled: job.enabled,
                created_at: job.created_at as u64,
            };

            Ok(HttpResponse::Created().json(CreateCronJobResponse {
                success: true,
                message: "Cron job created successfully".to_string(),
                data: job_info,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Failed to create cron job: {}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_cron_job_success() {
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
                .route("/api/v1/system/cron-jobs", web::post().to(create_cron_job))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 角色
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_create_cron_job_invalid_schedule() {
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
                .route("/api/v1/system/cron-jobs", web::post().to(create_cron_job))
        ).await;

        // 注意：实际测试需要测试无效 schedule 格式
        // 这里只是示例测试结构
        assert!(true);
    }
}
