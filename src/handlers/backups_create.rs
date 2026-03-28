// 备份创建 API — SQLite 持久化版
// POST /api/v1/backups — 创建备份任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::backup_store::SqliteBackupRepository;

/// 创建备份请求
#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_backup_type")]
    pub backup_type: String,
    pub source_path: String,
    pub destination: String,
    pub schedule: Option<String>,
}

fn default_backup_type() -> String { "full".to_string() }

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
    matches!(backup_type, "full" | "incremental")
}

/// 验证路径格式
fn validate_path(path: &str) -> bool {
    path.starts_with('/') && path.len() <= 512
}

/// 创建备份任务
/// - JWT 认证，admin 角色可访问
/// - 请求体包含：name, description, backup_type (full/incremental), source_path, destination, schedule (可选)
/// - 验证名称格式（400 Bad Request）
/// - 验证备份类型（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 使用 SqliteBackupRepository 持久化
/// - 创建成功返回 201 Created + 备份详情
pub async fn create_backup(
    req: HttpRequest,
    payload: web::Json<CreateBackupRequest>,
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
            error: "Only admin users can create backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证备份名称格式
    if !validate_backup_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid backup name. Must be 1-128 chars".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证备份类型
    if !validate_backup_type(&payload.backup_type) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid backup type. Valid types: full, incremental".to_string(),
            code: "INVALID_TYPE".to_string(),
        }));
    }

    // 6. 验证源路径格式
    if !validate_path(&payload.source_path) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid source path. Must start with / and be <= 512 chars".to_string(),
            code: "INVALID_SOURCE_PATH".to_string(),
        }));
    }

    // 7. 验证目标路径格式
    if !validate_path(&payload.destination) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid destination path. Must start with / and be <= 512 chars".to_string(),
            code: "INVALID_DESTINATION_PATH".to_string(),
        }));
    }

    // 8. 使用 SQLite 持久化创建备份任务
    let backup = repo.create_backup(
        &payload.name,
        &payload.description,
        &payload.backup_type,
        &payload.source_path,
        &payload.destination,
        payload.schedule.as_deref(),
    ).map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to create backup: {}", e)))?;

    // 9. 返回创建成功
    Ok(HttpResponse::Created().json(serde_json::json!({
        "success": true,
        "message": "Backup task created successfully",
        "data": backup
    })))
}
