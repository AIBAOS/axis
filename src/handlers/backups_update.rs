// Phase 165: 备份更新 API
// PUT /api/v1/backups/{id} — 更新备份任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新备份请求
#[derive(Debug, Deserialize)]
pub struct UpdateBackupRequest {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub size: Option<u64>,
    pub status: Option<String>,
    pub source_path: Option<String>,
    pub destination_path: Option<String>,
    pub compression: Option<bool>,
    pub encryption: Option<bool>,
}

/// 备份信息
#[derive(Serialize, Clone)]
pub struct BackupInfo {
    pub id: u64,
    pub name: String,
    pub r#type: String,
    pub size: u64,
    pub status: String,
    pub source_path: String,
    pub destination_path: String,
    pub compression: bool,
    pub encryption: bool,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// 更新备份响应
#[derive(Serialize)]
pub struct UpdateBackupResponse {
    pub success: bool,
    pub message: String,
    pub data: BackupInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证备份名称格式
fn validate_backup_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128
}

/// 验证备份类型
fn validate_backup_type(backup_type: &str) -> bool {
    matches!(backup_type, "daily" | "weekly" | "monthly" | "manual")
}

/// 验证路径格式
fn validate_path(path: &str) -> bool {
    path.starts_with('/') && path.len() <= 512
}

/// 验证备份状态
fn validate_status(status: &str) -> bool {
    matches!(status, "pending" | "running" | "completed" | "failed")
}

/// 更新备份任务（Phase 165）
/// - JWT 认证，admin 角色可访问
/// - 请求体包含：name/type/size/status/source_path/destination_path/compression/encryption（可选，部分更新）
/// - 验证备份 ID 存在性（404 Not Found）
/// - 验证名称格式（400 Bad Request）
/// - 验证备份类型（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 验证状态（400 Bad Request）
/// - 更新成功返回 200 OK + 备份详情
pub async fn update_backup(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateBackupRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let backup_id = path.into_inner();

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
            error: "Only admin users can update backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证备份名称格式（如果提供）
    if let Some(ref name) = payload.name {
        if !validate_backup_name(name) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid backup name. Must be 1-128 chars".to_string(),
                code: "INVALID_NAME".to_string(),
            }));
        }
    }

    // 5. 验证备份类型（如果提供）
    if let Some(ref backup_type) = payload.r#type {
        if !validate_backup_type(backup_type) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid backup type. Valid types: daily, weekly, monthly, manual".to_string(),
                code: "INVALID_TYPE".to_string(),
            }));
        }
    }

    // 6. 验证源路径格式（如果提供）
    if let Some(ref source_path) = payload.source_path {
        if !validate_path(source_path) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid source path. Must start with / and be <= 512 chars".to_string(),
                code: "INVALID_SOURCE_PATH".to_string(),
            }));
        }
    }

    // 7. 验证目标路径格式（如果提供）
    if let Some(ref destination_path) = payload.destination_path {
        if !validate_path(destination_path) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid destination path. Must start with / and be <= 512 chars".to_string(),
                code: "INVALID_DESTINATION_PATH".to_string(),
            }));
        }
    }

    // 8. 验证备份状态（如果提供）
    if let Some(ref status) = payload.status {
        if !validate_status(status) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid status. Valid statuses: pending, running, completed, failed".to_string(),
                code: "INVALID_STATUS".to_string(),
            }));
        }
    }

    // 9. 模拟现有备份数据
    let mut mock_backups = vec![
        BackupInfo {
            id: 1,
            name: "Daily Backup 2026-03-27".to_string(),
            r#type: "daily".to_string(),
            size: 1073741824,
            status: "completed".to_string(),
            source_path: "/srv/data".to_string(),
            destination_path: "/srv/backups/daily".to_string(),
            compression: true,
            encryption: false,
            created_at: "2026-03-27T00:00:00Z".to_string(),
            completed_at: Some("2026-03-27T01:30:00Z".to_string()),
        },
        BackupInfo {
            id: 2,
            name: "Weekly Backup 2026-03-24".to_string(),
            r#type: "weekly".to_string(),
            size: 5368709120,
            status: "completed".to_string(),
            source_path: "/srv/data".to_string(),
            destination_path: "/srv/backups/weekly".to_string(),
            compression: true,
            encryption: false,
            created_at: "2026-03-24T00:00:00Z".to_string(),
            completed_at: Some("2026-03-24T03:00:00Z".to_string()),
        },
        BackupInfo {
            id: 3,
            name: "Manual Backup 2026-03-26".to_string(),
            r#type: "manual".to_string(),
            size: 2147483648,
            status: "completed".to_string(),
            source_path: "/srv/sensitive".to_string(),
            destination_path: "/srv/backups/secure".to_string(),
            compression: true,
            encryption: true,
            created_at: "2026-03-26T12:00:00Z".to_string(),
            completed_at: Some("2026-03-26T12:45:00Z".to_string()),
        },
    ];

    // 10. 查找备份
    let backup_index = mock_backups.iter().position(|b| b.id == backup_id);

    // 11. 验证备份存在性
    if backup_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Backup {} not found", backup_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let backup_index = backup_index.unwrap();

    // 12. 部分更新备份配置
    let backup = &mut mock_backups[backup_index];
    
    if let Some(new_name) = &payload.name {
        backup.name = new_name.clone();
    }
    if let Some(new_type) = &payload.r#type {
        backup.r#type = new_type.clone();
    }
    if let Some(new_size) = payload.size {
        backup.size = new_size;
    }
    if let Some(new_status) = &payload.status {
        backup.status = new_status.clone();
    }
    if let Some(new_source_path) = &payload.source_path {
        backup.source_path = new_source_path.clone();
    }
    if let Some(new_destination_path) = &payload.destination_path {
        backup.destination_path = new_destination_path.clone();
    }
    if let Some(new_compression) = payload.compression {
        backup.compression = new_compression;
    }
    if let Some(new_encryption) = payload.encryption {
        backup.encryption = new_encryption;
    }

    // 13. 更新时间戳
    let now = chrono::Utc::now().to_rfc3339();

    // 14. 返回更新成功
    Ok(HttpResponse::Ok().json(UpdateBackupResponse {
        success: true,
        message: "Backup updated successfully".to_string(),
        data: backup.clone(),
    }))
}
