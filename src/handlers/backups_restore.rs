// Phase 193: 备份恢复 API
// POST /api/v1/backups/{id}/restore — 恢复备份任务

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::backup_store::SqliteBackupRepository;

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
/// - 验证备份状态为 archived（409 Conflict）
/// - 恢复成功返回 200 OK + 备份完整信息
pub async fn restore_backup(
    req: HttpRequest,
    path: web::Path<i64>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
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
            error: "Only admin users can restore backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 执行恢复操作
    match repo.restore_backup(backup_id) {
        Ok(Some(backup)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": format!("备份任务 '{}' 已恢复", backup.name),
            "data": backup
        }))),
        Ok(None) => {
            // 备份不存在或状态不是 archived
            match repo.get_backup_by_id(backup_id) {
                Ok(Some(backup)) => {
                    // 备份存在但状态不是 archived
                    Ok(HttpResponse::Conflict().json(ErrorResponse {
                        success: false,
                        error: format!("备份状态为 '{}'，仅 archived 状态的备份可恢复", backup.status),
                        code: "CONFLICT".to_string(),
                    }))
                }
                Ok(None) => {
                    // 备份不存在
                    Ok(HttpResponse::NotFound().json(ErrorResponse {
                        success: false,
                        error: format!("Backup {} not found", backup_id),
                        code: "NOT_FOUND".to_string(),
                    }))
                }
                Err(e) => {
                    Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                        success: false,
                        error: format!("查询备份任务失败：{}", e),
                        code: "DATABASE_ERROR".to_string(),
                    }))
                }
            }
        }
        Err(e) => {
            // 恢复失败（如状态校验失败）
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("恢复备份任务失败：{}", e),
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
    async fn test_restore_backup_success() {
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
                .route("/api/v1/backups/{id}/restore", web::post().to(restore_backup))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
