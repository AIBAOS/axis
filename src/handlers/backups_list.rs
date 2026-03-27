// Phase 162: 备份列表 API
// GET /api/v1/backups — 获取备份列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 备份列表查询参数
#[derive(Debug, Deserialize)]
pub struct BackupsListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// 备份信息
#[derive(Serialize, Clone)]
pub struct BackupInfo {
    pub id: u64,
    pub name: String,
    pub r#type: String,
    pub size: u64,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 备份列表响应
#[derive(Serialize)]
pub struct BackupsListResponse {
    pub success: bool,
    pub data: Vec<BackupInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份列表（Phase 162）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), limit(默认 20, 最大 100)
/// - 返回备份列表 + 分页信息
pub async fn list_backups(
    req: HttpRequest,
    query: web::Query<BackupsListQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);

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
            error: "Only admin users can list backups".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟备份数据
    let all_backups = vec![
        BackupInfo {
            id: 1,
            name: "Daily Backup 2026-03-27".to_string(),
            r#type: "daily".to_string(),
            size: 1073741824, // 1 GB
            status: "completed".to_string(),
            created_at: "2026-03-27T00:00:00Z".to_string(),
            completed_at: Some("2026-03-27T01:30:00Z".to_string()),
        },
        BackupInfo {
            id: 2,
            name: "Weekly Backup 2026-03-24".to_string(),
            r#type: "weekly".to_string(),
            size: 5368709120, // 5 GB
            status: "completed".to_string(),
            created_at: "2026-03-24T00:00:00Z".to_string(),
            completed_at: Some("2026-03-24T03:00:00Z".to_string()),
        },
        BackupInfo {
            id: 3,
            name: "Manual Backup 2026-03-26".to_string(),
            r#type: "manual".to_string(),
            size: 2147483648, // 2 GB
            status: "completed".to_string(),
            created_at: "2026-03-26T12:00:00Z".to_string(),
            completed_at: Some("2026-03-26T12:45:00Z".to_string()),
        },
        BackupInfo {
            id: 4,
            name: "Daily Backup 2026-03-26".to_string(),
            r#type: "daily".to_string(),
            size: 1073741824, // 1 GB
            status: "completed".to_string(),
            created_at: "2026-03-26T00:00:00Z".to_string(),
            completed_at: Some("2026-03-26T01:30:00Z".to_string()),
        },
        BackupInfo {
            id: 5,
            name: "Daily Backup 2026-03-25".to_string(),
            r#type: "daily".to_string(),
            size: 1073741824, // 1 GB
            status: "failed".to_string(),
            created_at: "2026-03-25T00:00:00Z".to_string(),
            completed_at: None,
        },
    ];

    // 5. 应用分页
    let total = all_backups.len() as u64;
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(all_backups.len());
    
    let backups = if start < all_backups.len() {
        all_backups[start..end].to_vec()
    } else {
        vec![]
    };

    // 6. 返回备份列表
    Ok(HttpResponse::Ok().json(BackupsListResponse {
        success: true,
        data: backups,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}
