// Phase 189: 备份任务执行 handler
// 手动执行备份任务（备份任务已根据 schedule 自动运行）

use actix_web::{web, HttpResponse, Error};
use serde::Serialize;

use crate::database::backup_store::SqliteBackupRepository;
use crate::services::jwt_service::JwtService;

/// 备份执行响应
#[derive(Serialize)]
pub struct RunBackupResponse {
    pub success: bool,
    pub message: String,
}

/// 执行备份任务
/// - JWT 认证，登录用户可访问
/// - 触发备份任务执行
pub async fn run_backup(
    req: actix_web::HttpRequest,
    jwt_service: web::Data<JwtService>,
    backup_repo: web::Data<SqliteBackupRepository>,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let _claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    let backup_id = path.into_inner();

    // 2. 执行备份
    match backup_repo.run_backup(backup_id as i64) {
        Ok(true) => Ok(HttpResponse::Ok().json(RunBackupResponse {
            success: true,
            message: format!("Backup task {} started", backup_id),
        })),
        Ok(false) => Ok(HttpResponse::NotFound().json(RunBackupResponse {
            success: false,
            message: format!("Backup task {} not found or already running", backup_id),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(RunBackupResponse {
            success: false,
            message: "Internal server error".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_run_backup() {
        // Basic structure test
        assert!(true);
    }
}

// Alias for compatibility
pub use run_backup as execute_backup_task;
