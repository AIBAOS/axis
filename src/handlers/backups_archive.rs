// Phase 194: 备份归档 API
// POST /api/v1/backups/{id}/archive — 归档备份任务（状态从 active/completed → archived）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;
use crate::database::backup_store::SqliteBackupRepository;
use crate::services::jwt_service::JwtService;

/// 备份归档响应
#[derive(Serialize, Clone)]
pub struct ArchiveBackupResponse {
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

/// 归档备份任务（Phase 194）
/// - JWT 认证，admin 角色可访问
/// - 验证备份 ID 存在性（404 Not Found）
/// - 验证备份状态（仅 active/completed 状态可归档，400 Bad Request）
/// - 检查是否已归档（409 Conflict）
/// - 归档成功返回 200 OK + 备份详情
pub async fn archive_backup(
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
            error: "Only admin users can archive backups".to_string(),
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

    // 5. 验证备份状态（仅 active/completed 状态可归档）
    if backup.status != "active" && backup.status != "completed" {
        if backup.status == "archived" {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: "Backup is already archived".to_string(),
                code: "ALREADY_ARCHIVED".to_string(),
            }));
        }
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Backup status is '{}'. Only active or completed backups can be archived", backup.status),
            code: "INVALID_STATUS".to_string(),
        }));
    }

    // 6. 使用存储库归档备份
    backup_repo.archive_backup(backup_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Archive failed: {}", e))
    })?;

    let archived_backup = backup_repo.get_backup_by_id(backup_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?.ok_or_else(|| {
        actix_web::error::ErrorNotFound("Backup not found after archive")
    })?;

    // 7. 返回归档成功
    Ok(HttpResponse::Ok().json(ArchiveBackupResponse {
        success: true,
        message: format!("Backup '{}' archived successfully", archived_backup.name),
        data: BackupInfo {
            id: archived_backup.id as u64,
            name: archived_backup.name,
            description: archived_backup.description,
            backup_type: archived_backup.backup_type,
            source_path: archived_backup.source_path,
            destination_path: archived_backup.destination_path,
            schedule: archived_backup.schedule,
            status: archived_backup.status,
            last_run_at: archived_backup.last_run_at,
            last_run_status: archived_backup.last_run_status,
            last_run_size_bytes: archived_backup.last_run_size_bytes,
            created_at: archived_backup.created_at,
            updated_at: archived_backup.updated_at,
        },
    }))
}
