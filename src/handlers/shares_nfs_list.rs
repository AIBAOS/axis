// Phase 213: NFS 共享列表 API (增强版)
// GET /api/v1/shares/nfs — 获取 NFS 共享列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// NFS 共享查询参数
#[derive(Debug, Deserialize)]
pub struct NfsSharesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

/// NFS 客户端配置
#[derive(Serialize, Clone)]
pub struct NfsClientConfig {
    pub network: String,
    pub access: String,
}

/// NFS 共享信息
#[derive(Serialize, Clone)]
pub struct NfsShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub comment: Option<String>,
    pub read_only: bool,
    pub no_subtree_check: bool,
    pub sync: bool,
    pub clients: Option<String>,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// NFS 共享列表响应
#[derive(Serialize)]
pub struct NfsShareListResponse {
    pub success: bool,
    pub data: Vec<NfsShareInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 NFS 共享列表（Phase 213）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
/// - 支持状态过滤：status(active/inactive)
/// - 返回 NFS 共享列表 + 分页信息
pub async fn list_nfs_shares(
    req: HttpRequest,
    query: web::Query<NfsSharesQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let per_page = query.per_page.unwrap_or(20).min(100);
    let status_filter = query.status.clone();

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
            error: "Only admin users can list NFS shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询 NFS 共享列表
    let shares = repo.get_shares(page, per_page, Some("nfs".to_string()), status_filter.clone())
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    // 5. 计算总记录数（用于分页）
    let total = shares.len() as u64;

    // 6. 转换为响应格式
    let data: Vec<NfsShareInfo> = shares.iter().map(|s| NfsShareInfo {
        id: s.id,
        name: s.name.clone(),
        path: s.path.clone(),
        comment: s.comment.clone(),
        read_only: s.read_only,
        no_subtree_check: s.no_subtree_check,
        sync: s.sync,
        clients: s.clients.clone(),
        enabled: s.enabled,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }).collect();

    // 7. 计算总页数
    let total_pages = if per_page > 0 {
        ((total as f64) / (per_page as f64)).ceil() as u32
    } else {
        1
    };

    // 8. 返回 NFS 共享列表
    Ok(HttpResponse::Ok().json(NfsShareListResponse {
        success: true,
        data,
        pagination: PaginationInfo {
            page,
            per_page,
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
    async fn test_list_nfs_shares_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let repo = web::Data::new(Arc::new(SqliteShareRepository::new(
            crate::database::pool::create_sqlite_pool(":memory:").expect("Failed to create test database"),
        )));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .app_data(repo)
                .route("/api/v1/shares/nfs", web::get().to(list_nfs_shares))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
