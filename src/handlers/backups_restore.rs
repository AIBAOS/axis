// Phase 95: 备份恢复 API
// POST /api/v1/backups/{id}/restore — 恢复备份任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 恢复请求
#[derive(Debug, Deserialize)]
pub struct RestoreRequest {
    /// 恢复类型：single_file/full_volume
    pub restore_type: String,
    /// 目标路径（单文件恢复时必填）
    pub target_path: Option<String>,
    /// 源文件路径（单文件恢复时必填）
    pub source_file: Option<String>,
    /// 是否覆盖现有文件
    pub overwrite: Option<bool>,
}

/// 恢复信息
#[derive(Serialize)]
pub struct RestoreInfo {
    pub backup_id: u64,
    pub restore_type: String,
    pub source_path: String,
    pub target_path: String,
    pub files_restored: u64,
    pub bytes_restored: u64,
    pub restored_at: u64,
    pub status: String,
}

/// 恢复响应
#[derive(Serialize)]
pub struct RestoreResponse {
    pub success: bool,
    pub message: String,
    pub data: RestoreInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证恢复类型
fn is_valid_restore_type(restore_type: &str) -> bool {
    let valid_types = ["single_file", "full_volume", "incremental", "differential"];
    valid_types.contains(&restore_type)
}

/// 恢复备份（Phase 95）
/// - JWT 认证，admin 角色可访问
/// - 支持恢复类型：single_file/full_volume/incremental/differential
/// - 验证目标路径格式（绝对路径、禁止路径遍历）
/// - 验证备份任务存在性（404 Not Found）
/// - 返回恢复信息（恢复文件数、字节数、状态）
pub async fn restore_backup(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<RestoreRequest>,
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can restore backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证恢复类型
    if !is_valid_restore_type(&payload.restore_type) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid restore type. Valid values: single_file, full_volume, incremental, differential".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 5. 单文件恢复时验证必填字段
    if payload.restore_type == "single_file" {
        if payload.source_file.is_none() || payload.target_path.is_none() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "source_file and target_path are required for single_file restore".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    // 6. 验证目标路径格式
    if let Some(ref target_path) = payload.target_path {
        if !target_path.starts_with('/') {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "target_path must be an absolute path starting with '/'".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
        if target_path.contains("..") {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "target_path contains invalid path traversal sequence".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
        if target_path.contains('\0') {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "target_path contains invalid characters".to_string(),
                code: "INVALID_PARAMS".to_string(),
            }));
        }
    }

    // 7. 模拟备份任务验证（实际应从数据库读取）
    let mock_backup_tasks = vec![
        (1, "Daily Backup", "/data", "/backup/daily", "full", "active"),
        (2, "Weekly Backup", "/data", "/backup/weekly", "full", "active"),
        (3, "Incremental Backup", "/data", "/backup/inc", "incremental", "active"),
    ];

    let backup_exists = mock_backup_tasks.iter().any(|(id, _, _, _, _, _)| *id == backup_id);
    if !backup_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Backup task {} not found", backup_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 8. 执行恢复（模拟）
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let (_, name, source_path, _, backup_type, _) = mock_backup_tasks
        .into_iter()
        .find(|(id, _, _, _, _, _)| *id == backup_id)
        .unwrap();

    let target = payload.target_path.clone().unwrap_or_else(|| source_path.to_string());
    let overwrite = payload.overwrite.unwrap_or(false);

    // 根据恢复类型计算恢复量
    let (files_restored, bytes_restored) = match payload.restore_type.as_str() {
        "single_file" => (1, 1024 * 1024), // 1 文件，1MB
        "full_volume" => (1500, 50 * 1024 * 1024 * 1024), // 1500 文件，50GB
        "incremental" => (200, 5 * 1024 * 1024 * 1024), // 200 文件，5GB
        "differential" => (500, 15 * 1024 * 1024 * 1024), // 500 文件，15GB
        _ => (0, 0),
    };

    // 9. 返回恢复结果
    Ok(HttpResponse::Ok().json(RestoreResponse {
        success: true,
        message: format!("Backup '{}' restored successfully", name),
        data: RestoreInfo {
            backup_id,
            restore_type: payload.restore_type.clone(),
            source_path: source_path.to_string(),
            target_path: target,
            files_restored,
            bytes_restored,
            restored_at: now,
            status: "completed".to_string(),
        },
    }))
}