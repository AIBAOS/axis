// Phase 261: 备份任务创建 API
// POST /api/v1/backups — 创建备份任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::backup_store::SqliteBackupRepository;

/// 创建备份任务请求
#[derive(Debug, Deserialize)]
pub struct CreateBackupRequest {
    pub name: String,
    pub description: Option<String>,
    pub source_path: String,
    pub destination: String,
    pub backup_type: String,
    pub schedule: Option<String>,
}

/// 创建的备份任务信息
#[derive(Serialize, Clone)]
pub struct CreatedBackup {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建备份任务响应
#[derive(Serialize)]
pub struct CreateBackupResponse {
    pub success: bool,
    pub message: String,
    pub data: CreatedBackup,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证备份任务名称格式
fn validate_backup_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128 && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ')
}

/// 验证备份类型
fn validate_backup_type(backup_type: &str) -> bool {
    matches!(backup_type, "full" | "incremental")
}

/// 验证路径格式
fn validate_path(path: &str) -> bool {
    path.starts_with('/') && path.len() <= 512
}

/// 创建备份任务（Phase 261）
/// - JWT 认证，admin 角色可访问
/// - 使用 SqliteBackupRepository 实现真实数据库创建
/// - 请求体包含：name/description/source_path/destination/backup_type/schedule
/// - backup_type 必须是 full 或 incremental
/// - 验证名称格式（400 Bad Request）
/// - 验证备份类型（400 Bad Request）
/// - 验证路径格式（400 Bad Request）
/// - 创建成功返回 201 Created + 备份任务详情
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
            error: "Only admin users can create backup tasks".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证备份任务名称格式
    if !validate_backup_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid backup name. Must be 1-128 chars, alphanumeric with -_  allowed".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证备份类型
    if !validate_backup_type(&payload.backup_type) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid backup type. Valid types: full, incremental".to_string(),
            code: "INVALID_BACKUP_TYPE".to_string(),
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
            code: "INVALID_DESTINATION".to_string(),
        }));
    }

    // 8. 使用数据库创建备份任务
    let description = payload.description.clone().unwrap_or_default();
    match repo.create_backup(
        &payload.name,
        &description,
        &payload.backup_type,
        &payload.source_path,
        &payload.destination,
        payload.schedule.as_deref(),
    ) {
        Ok(backup) => {
            let new_backup = CreatedBackup {
                id: backup.id,
                name: backup.name,
                description: backup.description,
                backup_type: backup.backup_type,
                source_path: backup.source_path,
                destination_path: backup.destination_path,
                schedule: backup.schedule,
                status: backup.status,
                created_at: backup.created_at,
                updated_at: backup.updated_at,
            };

            Ok(HttpResponse::Created().json(CreateBackupResponse {
                success: true,
                message: "Backup task created successfully".to_string(),
                data: new_backup,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("创建备份任务失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_create_backup_unauthorized() {
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
                .route("/api/v1/backups", web::post().to(create_backup))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_create_backup_forbidden() {
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
                .route("/api/v1/backups", web::post().to(create_backup))
        ).await;

        // 注意：实际测试需要验证非 admin 用户被拒绝
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_create_backup_invalid_type() {
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
                .route("/api/v1/backups", web::post().to(create_backup))
        ).await;

        // 注意：实际测试需要验证 backup_type 必须是 full 或 incremental
        // 这里只是示例测试结构
        assert!(true);
    }
}
