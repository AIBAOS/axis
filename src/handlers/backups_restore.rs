// Phase 193: 备份恢复 API
// POST /api/v1/backups/{id}/restore — 恢复备份任务（状态从 archived → active）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;
use crate::database::backup_store::SqliteBackupRepository;
use crate::services::jwt_service::JwtService;

/// 备份恢复响应
#[derive(Serialize, Clone)]
pub struct RestoreBackupResponse {
    pub success: bool,
    pub message: String,
    pub data: BackupInfo,
}

/// 备份信息
#[derive(Serialize, Clone)]
pub struct BackupInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
    pub status: String,
    pub last_run_at: Option<i64>,
    pub last_run_status: Option<String>,
    pub last_run_size_bytes: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 恢复备份任务（Phase 193）
/// - JWT 认证，admin 角色可访问
/// - 验证备份 ID 存在性（404 Not Found）
/// - 验证备份状态（仅 archived 状态可恢复，400 Bad Request）
/// - 检查是否存在同名活跃备份（409 Conflict）
/// - 恢复成功返回 200 OK + 备份详情
pub async fn restore_backup(
    req: HttpRequest,
    path: web::Path<u64>,
    backup_repo: web::Data<Arc<SqliteBackupRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let backup_id = path.into_inner() as i64;

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

    // 2. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can restore backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 查询备份
    let backup = backup_repo.get_backup_by_id(backup_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // 4. 验证备份存在性
    let backup = backup.ok_or_else(|| {
        actix_web::error::ErrorNotFound("Backup not found")
    })?;

    // 5. 验证备份状态（仅 archived 状态可恢复）
    if backup.status != "archived" {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Backup status is '{}'. Only archived backups can be restored", backup.status),
            code: "INVALID_STATUS".to_string(),
        }));
    }

    // 6. 检查是否存在同名活跃备份（409 Conflict）
    let all_backups = backup_repo.get_backups(None, None, 1, 1000).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?.0;

    let has_conflicting_active = all_backups.iter().any(|b| {
        b.id != backup_id as i64 && b.status == "active" && b.name == backup.name
    });

    if has_conflicting_active {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("A backup with name '{}' is already active", backup.name),
            code: "CONFLICT".to_string(),
        }));
    }

    // 7. 使用存储库更新状态为 active
    backup_repo.restore_backup(backup_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Restore failed: {}", e))
    })?;

    let updated_backup = backup_repo.get_backup_by_id(backup_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?.ok_or_else(|| {
        actix_web::error::ErrorNotFound("Backup not found after update")
    })?;

    // 8. 返回恢复成功
    Ok(HttpResponse::Ok().json(RestoreBackupResponse {
        success: true,
        message: format!("Backup '{}' restored successfully", updated_backup.name),
        data: BackupInfo {
            id: updated_backup.id as u64,
            name: updated_backup.name,
            description: updated_backup.description,
            backup_type: updated_backup.backup_type,
            source_path: updated_backup.source_path,
            destination_path: updated_backup.destination_path,
            schedule: updated_backup.schedule,
            status: updated_backup.status,
            last_run_at: updated_backup.last_run_at,
            last_run_status: updated_backup.last_run_status,
            last_run_size_bytes: updated_backup.last_run_size_bytes,
            created_at: updated_backup.created_at,
            updated_at: updated_backup.updated_at,
        },
    }))
}
