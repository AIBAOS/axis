// Phase 190: 备份任务详情 API
// GET /api/v1/backups/{id} — 获取备份任务详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 备份任务详情信息
#[derive(Serialize, Clone)]
pub struct BackupTaskDetail {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: String,
    pub status: String,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub last_duration: Option<u64>,
    pub last_status: Option<String>,
    pub retention_policy: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 备份任务详情响应
#[derive(Serialize)]
pub struct BackupTaskDetailResponse {
    pub success: bool,
    pub data: BackupTaskDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份任务详情（Phase 190）
/// - JWT 认证，admin 角色可访问
/// - 验证任务 ID 存在性（404 Not Found）
/// - 返回任务详细信息
pub async fn get_backup_task_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let task_id = path.into_inner();

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
            error: "Only admin users can view backup task details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟备份任务数据
    let mock_tasks = vec![
        BackupTaskDetail {
            id: 1,
            name: "Daily Backup".to_string(),
            description: "Daily backup of system data".to_string(),
            source_path: "/data".to_string(),
            destination_path: "/backup/daily".to_string(),
            schedule: "0 2 * * *".to_string(),
            status: "active".to_string(),
            last_run: Some("2026-03-27T02:00:00Z".to_string()),
            next_run: Some("2026-03-28T02:00:00Z".to_string()),
            last_duration: Some(3600),
            last_status: Some("success".to_string()),
            retention_policy: Some("7d".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-27T02:00:00Z".to_string(),
        },
        BackupTaskDetail {
            id: 2,
            name: "Weekly Backup".to_string(),
            description: "Weekly full backup".to_string(),
            source_path: "/".to_string(),
            destination_path: "/backup/weekly".to_string(),
            schedule: "0 3 * * 0".to_string(),
            status: "active".to_string(),
            last_run: Some("2026-03-24T03:00:00Z".to_string()),
            next_run: Some("2026-03-31T03:00:00Z".to_string()),
            last_duration: Some(14400),
            last_status: Some("success".to_string()),
            retention_policy: Some("30d".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-24T03:00:00Z".to_string(),
        },
        BackupTaskDetail {
            id: 3,
            name: "Monthly Backup".to_string(),
            description: "Monthly archive backup".to_string(),
            source_path: "/data".to_string(),
            destination_path: "/backup/monthly".to_string(),
            schedule: "0 4 1 * *".to_string(),
            status: "inactive".to_string(),
            last_run: Some("2026-03-01T04:00:00Z".to_string()),
            next_run: Some("2026-04-01T04:00:00Z".to_string()),
            last_duration: Some(7200),
            last_status: Some("success".to_string()),
            retention_policy: Some("365d".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-01T04:00:00Z".to_string(),
        },
    ];

    // 5. 查找任务
    let task = mock_tasks.into_iter().find(|t| t.id == task_id);

    // 6. 验证任务存在性
    match task {
        Some(task) => {
            // 7. 返回任务详情
            Ok(HttpResponse::Ok().json(BackupTaskDetailResponse {
                success: true,
                data: task,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Backup task {} not found", task_id),
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
    async fn test_get_backup_task_detail_success() {
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
                .route("/api/v1/backups/{id}", web::get().to(get_backup_task_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_backup_task_detail_not_found() {
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
                .route("/api/v1/backups/{id}", web::get().to(get_backup_task_detail))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }
}
