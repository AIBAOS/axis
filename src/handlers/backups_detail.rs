// Phase 164: 备份详情 API
// GET /api/v1/backups/{id} — 获取备份详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 备份详情信息
#[derive(Serialize, Clone)]
pub struct BackupDetail {
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

/// 备份详情响应
#[derive(Serialize)]
pub struct BackupDetailResponse {
    pub success: bool,
    pub data: BackupDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份详情（Phase 164）
/// - JWT 认证，admin 角色可访问
/// - 验证备份 ID 存在性（404 Not Found）
/// - 返回备份详情
pub async fn get_backup(
    req: HttpRequest,
    path: web::Path<u64>,
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
            error: "Only admin users can view backup details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟备份数据
    let mock_backups = vec![
        BackupDetail {
            id: 1,
            name: "Daily Backup 2026-03-27".to_string(),
            r#type: "daily".to_string(),
            size: 1073741824, // 1 GB
            status: "completed".to_string(),
            source_path: "/srv/data".to_string(),
            destination_path: "/srv/backups/daily".to_string(),
            compression: true,
            encryption: false,
            created_at: "2026-03-27T00:00:00Z".to_string(),
            completed_at: Some("2026-03-27T01:30:00Z".to_string()),
        },
        BackupDetail {
            id: 2,
            name: "Weekly Backup 2026-03-24".to_string(),
            r#type: "weekly".to_string(),
            size: 5368709120, // 5 GB
            status: "completed".to_string(),
            source_path: "/srv/data".to_string(),
            destination_path: "/srv/backups/weekly".to_string(),
            compression: true,
            encryption: false,
            created_at: "2026-03-24T00:00:00Z".to_string(),
            completed_at: Some("2026-03-24T03:00:00Z".to_string()),
        },
        BackupDetail {
            id: 3,
            name: "Manual Backup 2026-03-26".to_string(),
            r#type: "manual".to_string(),
            size: 2147483648, // 2 GB
            status: "completed".to_string(),
            source_path: "/srv/sensitive".to_string(),
            destination_path: "/srv/backups/secure".to_string(),
            compression: true,
            encryption: true,
            created_at: "2026-03-26T12:00:00Z".to_string(),
            completed_at: Some("2026-03-26T12:45:00Z".to_string()),
        },
    ];

    // 5. 查找备份
    let backup = mock_backups.into_iter().find(|b| b.id == backup_id);

    // 6. 验证备份存在性
    match backup {
        Some(backup) => {
            // 7. 返回备份详情
            Ok(HttpResponse::Ok().json(BackupDetailResponse {
                success: true,
                data: backup,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Backup {} not found", backup_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
