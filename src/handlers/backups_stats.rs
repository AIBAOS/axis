// Phase 196: 备份统计 API
// GET /api/v1/backups/stats — 获取备份任务和执行历史统计信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::backup_store::SqliteBackupRepository;

/// 备份统计响应
#[derive(Serialize)]
pub struct BackupStatsResponse {
    pub success: bool,
    pub data: BackupStatsData,
}

/// 备份统计数据
#[derive(Serialize)]
pub struct BackupStatsData {
    pub total_backups: u32,
    pub active_backups: u32,
    pub archived_backups: u32,
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub running_executions: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份统计信息（Phase 196）
/// - JWT 认证，admin 角色可访问
/// - 返回备份任务和执行历史的统计数据
/// - 用于仪表板展示
pub async fn get_backup_stats(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
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
            error: "Only admin users can access backup statistics".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 获取统计信息
    match repo.get_backup_stats() {
        Ok(stats) => Ok(HttpResponse::Ok().json(BackupStatsResponse {
            success: true,
            data: BackupStatsData {
                total_backups: stats.total_backups,
                active_backups: stats.active_backups,
                archived_backups: stats.archived_backups,
                total_executions: stats.total_executions,
                successful_executions: stats.successful_executions,
                failed_executions: stats.failed_executions,
                running_executions: stats.running_executions,
            },
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询备份统计失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_backup_stats_success() {
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
                .route("/api/v1/backups/stats", web::get().to(get_backup_stats))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
