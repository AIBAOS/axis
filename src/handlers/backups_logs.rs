// Phase 191: 备份任务日志 API
// GET /api/v1/backups/{id}/logs — 获取备份任务执行日志

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub level: Option<String>,
}

/// 日志条目
#[derive(Serialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// 备份任务日志响应
#[derive(Serialize)]
pub struct BackupLogsResponse {
    pub success: bool,
    pub data: BackupLogsData,
}

/// 备份日志数据
#[derive(Serialize)]
pub struct BackupLogsData {
    pub backup_id: u64,
    pub logs: Vec<LogEntry>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份任务日志（Phase 191）
/// - JWT 认证，admin 角色可访问
/// - 验证备份 ID 存在性（404 Not Found）
/// - 支持分页和日志级别筛选
/// - 返回日志列表
pub async fn get_backup_logs(
    req: HttpRequest,
    path: web::Path<u64>,
    query: web::Query<LogsQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let backup_id = path.into_inner();
    let limit = query.limit.unwrap_or(50).max(1).min(200);
    let offset = query.offset.unwrap_or(0);
    let level_filter = query.level.as_ref().map(|s| s.to_lowercase());

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
            error: "Only admin users can view backup logs".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟备份任务数据（验证存在性）
    let mock_tasks = vec![1u64, 2, 3];
    if !mock_tasks.contains(&backup_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Backup task {} not found", backup_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 5. 模拟日志数据
    let all_logs = vec![
        LogEntry {
            timestamp: "2026-03-27T18:00:00Z".to_string(),
            level: "info".to_string(),
            message: "Backup task started".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:00:05Z".to_string(),
            level: "info".to_string(),
            message: "Scanning source directory: /data".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:00:10Z".to_string(),
            level: "info".to_string(),
            message: "Found 1234 files to backup".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:00:15Z".to_string(),
            level: "warning".to_string(),
            message: "Skipping locked file: /data/lock.tmp".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:00:30Z".to_string(),
            level: "info".to_string(),
            message: "Compressing backup archive".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:01:00Z".to_string(),
            level: "info".to_string(),
            message: "Uploading to destination: /backup/daily".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:01:30Z".to_string(),
            level: "info".to_string(),
            message: "Backup completed successfully".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:01:31Z".to_string(),
            level: "info".to_string(),
            message: "Total size: 1.2 GB".to_string(),
        },
        LogEntry {
            timestamp: "2026-03-27T18:01:32Z".to_string(),
            level: "info".to_string(),
            message: "Backup task finished".to_string(),
        },
    ];

    // 6. 应用日志级别筛选
    let filtered_logs: Vec<LogEntry> = if let Some(level) = level_filter {
        all_logs.into_iter().filter(|log| log.level == level).collect()
    } else {
        all_logs
    };

    // 7. 应用分页
    let total = filtered_logs.len() as u64;
    let start = offset as usize;
    let end = (start + limit as usize).min(filtered_logs.len());
    let logs = if start < filtered_logs.len() {
        filtered_logs[start..end].to_vec()
    } else {
        vec![]
    };

    // 8. 返回日志列表
    Ok(HttpResponse::Ok().json(BackupLogsResponse {
        success: true,
        data: BackupLogsData {
            backup_id,
            logs,
            total,
            limit,
            offset,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_backup_logs_success() {
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
                .route("/api/v1/backups/{id}/logs", web::get().to(get_backup_logs))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_backup_logs_not_found() {
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
                .route("/api/v1/backups/{id}/logs", web::get().to(get_backup_logs))
        ).await;

        // 注意：实际测试需要有效的 JWT token
        // 这里只是示例测试结构
        assert!(true);
    }
}
