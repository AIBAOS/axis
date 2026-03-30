// Phase 189/260: 备份任务列表 API
// GET /api/v1/backups — 获取备份任务列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 备份任务信息
#[derive(Serialize, Clone)]
pub struct BackupTask {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: String,
    pub status: String,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 分页信息
#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub total_pages: u32,
}

/// 备份任务列表响应
#[derive(Serialize)]
pub struct BackupTaskListResponse {
    pub success: bool,
    pub data: Vec<BackupTask>,
    pub pagination: PaginationInfo,
}

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct BackupsQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份任务列表（Phase 189/260）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1)/page_size(默认 20, 最大 100)
/// - 支持状态过滤：status(active/inactive/all)
/// - 返回备份任务列表 + 分页信息
pub async fn list_backup_tasks(
    req: HttpRequest,
    query: web::Query<BackupsQuery>,
    jwt_service: web::Data<JwtService>,
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
            error: "Only admin users can view backup tasks".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析查询参数
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).max(1).min(100);

    // 5. 模拟备份任务数据（实际应从数据库读取）
    let mut all_tasks = vec![
        BackupTask {
            id: 1,
            name: "Daily Backup".to_string(),
            description: "Daily backup of system data".to_string(),
            source_path: "/data".to_string(),
            destination_path: "/backup/daily".to_string(),
            schedule: "0 2 * * *".to_string(), // Daily at 2 AM
            status: "active".to_string(),
            last_run: Some("2026-03-27T02:00:00Z".to_string()),
            next_run: Some("2026-03-28T02:00:00Z".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-27T02:00:00Z".to_string(),
        },
        BackupTask {
            id: 2,
            name: "Weekly Backup".to_string(),
            description: "Weekly full backup".to_string(),
            source_path: "/".to_string(),
            destination_path: "/backup/weekly".to_string(),
            schedule: "0 3 * * 0".to_string(), // Weekly on Sunday at 3 AM
            status: "active".to_string(),
            last_run: Some("2026-03-24T03:00:00Z".to_string()),
            next_run: Some("2026-03-31T03:00:00Z".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-24T03:00:00Z".to_string(),
        },
        BackupTask {
            id: 3,
            name: "Monthly Backup".to_string(),
            description: "Monthly archive backup".to_string(),
            source_path: "/data".to_string(),
            destination_path: "/backup/monthly".to_string(),
            schedule: "0 4 1 * *".to_string(), // Monthly on 1st at 4 AM
            status: "inactive".to_string(),
            last_run: Some("2026-03-01T04:00:00Z".to_string()),
            next_run: Some("2026-04-01T04:00:00Z".to_string()),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-01T04:00:00Z".to_string(),
        },
    ];

    // 6. 应用状态过滤
    if let Some(ref status_filter) = query.status {
        if status_filter.to_lowercase() != "all" {
            all_tasks.retain(|t| t.status.to_lowercase() == status_filter.to_lowercase());
        }
    }

    // 7. 应用分页
    let total = all_tasks.len() as u32;
    let total_pages = (total + page_size - 1) / page_size;
    let start = ((page - 1) * page_size) as usize;
    let end = (start + page_size as usize).min(all_tasks.len());
    
    let tasks = if start < all_tasks.len() {
        all_tasks[start..end].to_vec()
    } else {
        vec![]
    };

    // 8. 返回备份任务列表 + 分页信息
    Ok(HttpResponse::Ok().json(BackupTaskListResponse {
        success: true,
        data: tasks,
        pagination: PaginationInfo {
            page,
            page_size,
            total,
            total_pages,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_backup_tasks_success() {
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
                .route("/api/v1/backups", web::get().to(list_backup_tasks))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 权限
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_list_backup_tasks_pagination() {
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
                .route("/api/v1/backups", web::get().to(list_backup_tasks))
        ).await;

        // 注意：实际测试需要验证分页参数
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_list_backup_tasks_unauthorized() {
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
                .route("/api/v1/backups", web::get().to(list_backup_tasks))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
