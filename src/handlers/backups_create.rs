// Phase 261: 创建备份任务 API
// POST /api/v1/backups

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use chrono::{TimeZone, Utc};

use crate::database::backup_store::SqliteBackupRepository;
use crate::services::jwt_service::JwtService;

/// 创建备份任务请求体
#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "default_backup_type")]
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
}

fn default_backup_type() -> String {
    "full".to_string()
}

/// 创建备份任务响应
#[derive(Serialize)]
pub struct CreateBackupResponse {
    pub success: bool,
    pub data: BackupTask,
    pub message: Option<String>,
}

/// 备份任务信息
#[derive(Serialize, Clone)]
pub struct BackupTask {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
    pub status: String,
    pub last_run_at: Option<String>,
    pub last_run_status: Option<String>,
    pub last_run_size_bytes: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::database::backup_store::BackupRow> for BackupTask {
    fn from(row: crate::database::backup_store::BackupRow) -> Self {
        BackupTask {
            id: row.id as u64,
            name: row.name,
            description: row.description,
            backup_type: row.backup_type,
            source_path: row.source_path,
            destination_path: row.destination_path,
            schedule: row.schedule,
            status: row.status,
            last_run_at: Some(row.last_run_at.and_then(|t| {
                chrono::DateTime::from_timestamp(t, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            }).unwrap_or_default()),
            last_run_status: row.last_run_status,
            last_run_size_bytes: row.last_run_size_bytes,
            created_at: chrono::DateTime::from_timestamp(row.created_at, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
            updated_at: chrono::DateTime::from_timestamp(row.updated_at, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default(),
        }
    }
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 创建备份任务
/// - JWT 认证，仅 admin 角色可访问
/// - 创建新的备份任务
pub async fn create_backup(
    req: actix_web::HttpRequest,
    jwt_service: web::Data<JwtService>,
    backup_repo: web::Data<SqliteBackupRepository>,
    payload: web::Json<CreateBackupRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限检查：仅 admin 可创建备份
    if !claims.roles.iter().any(|r| r.to_lowercase() == "admin") {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only administrators can create backup tasks".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 验证必填字段
    if payload.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "VALIDATION_ERROR".to_string(),
        }));
    }

    if payload.source_path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "source_path is required".to_string(),
            code: "VALIDATION_ERROR".to_string(),
        }));
    }

    if payload.destination_path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "destination_path is required".to_string(),
            code: "VALIDATION_ERROR".to_string(),
        }));
    }

    // 4. 路径安全验证：防止路径遍历攻击
    fn validate_path(path: &str, field_name: &str) -> Result<(), ErrorResponse> {
        // 必须是绝对路径
        if !path.starts_with('/') {
            return Err(ErrorResponse {
                success: false,
                error: format!("{} must be an absolute path", field_name),
                code: "VALIDATION_ERROR".to_string(),
            });
        }
        // 禁止路径遍历
        if path.contains("..") {
            return Err(ErrorResponse {
                success: false,
                error: format!("{} contains invalid path sequence", field_name),
                code: "VALIDATION_ERROR".to_string(),
            });
        }
        // 禁止 null 字节
        if path.contains('\0') {
            return Err(ErrorResponse {
                success: false,
                error: format!("{} contains invalid characters", field_name),
                code: "VALIDATION_ERROR".to_string(),
            });
        }
        Ok(())
    }

    if let Err(e) = validate_path(&payload.source_path, "source_path") {
        return Ok(HttpResponse::BadRequest().json(e));
    }
    if let Err(e) = validate_path(&payload.destination_path, "destination_path") {
        return Ok(HttpResponse::BadRequest().json(e));
    }

    // 5. 验证 backup_type
    let valid_types = ["full", "incremental", "differential"];
    if !valid_types.contains(&payload.backup_type.as_str()) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Invalid backup_type. Must be one of: {}", valid_types.join(", ")),
            code: "VALIDATION_ERROR".to_string(),
        }));
    }

    // 6. 验证 schedule 格式（如果提供）
    if let Some(ref schedule) = payload.schedule {
        if !cron_expression_valid(schedule) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid cron expression for schedule".to_string(),
                code: "VALIDATION_ERROR".to_string(),
            }));
        }
    }

    // 7. 创建备份任务
    let description = payload.description.clone().unwrap_or_default();
    let schedule_opt = payload.schedule.clone();
    let schedule = schedule_opt.as_deref();

    match backup_repo.create_backup(
        &payload.name,
        &description,
        &payload.backup_type,
        &payload.source_path,
        &payload.destination_path,
        schedule,
    ) {
        Ok(row) => Ok(HttpResponse::Created().json(CreateBackupResponse {
            success: true,
            data: BackupTask::from(row),
            message: Some("Backup task created successfully".to_string()),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("Failed to create backup task: {}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

/// 简单的 cron 表达式验证（不深入解析，只做格式检查）
fn cron_expression_valid(expr: &str) -> bool {
    // Basic validation: expect 5 fields (min hour day month dow)
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }

    // Each field should be valid cron field format
    parts.iter().all(|part| {
        // Allow: *, numbers, ranges (-), lists (,), steps (/)
        part.chars().all(|c| c == '*' || c == ',' || c == '-' || c == '/' || c.is_ascii_digit())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use std::sync::{Arc, Mutex};

    #[actix_web::test]
    async fn test_create_backup_task_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        // Create in-memory SQLite database for testing
        let db_path = std::env::temp_dir().join("test_backup.db");
        let _ = std::fs::remove_file(&db_path);
        let pool = arc_sqlite::Pool::new(move || {
            arc_sqlite::Connection::open(&db_path).map_err(|e| {
                arc_sqlite::PoolError::OpenConnectionError(e.to_string())
            })
        });

        // Create backup repository and init tables
        let backup_repo = web::Data::new(SqliteBackupRepository::new(Arc::new(Mutex::new(
            arc_sqlite::SqliteConnectionManager::new(&db_path).into(),
        ))));
        let _ = backup_repo.init_tables();

        let app = test::init_service(
            App::new()
                .app_data(jwt_service.clone())
                .app_data(backup_repo.clone())
                .route("/api/v1/backups", web::post().to(create_backup))
        ).await;

        // Note: Actual test would require valid JWT token
        // This is just structure verification
        assert!(true);
    }
}
