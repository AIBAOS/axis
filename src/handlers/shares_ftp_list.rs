// Phase 220: FTP 共享列表 API
// GET /api/v1/shares/ftp — 获取 FTP 共享列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// FTP 共享查询参数
#[derive(Debug, Deserialize)]
pub struct FtpSharesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

/// FTP 共享信息
#[derive(Serialize, Clone)]
pub struct FtpShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub public: bool,
    pub status: String,
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

/// FTP 共享列表响应
#[derive(Serialize)]
pub struct FtpShareListResponse {
    pub success: bool,
    pub data: Vec<FtpShareInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 FTP 共享列表（Phase 220）
/// - JWT 认证，admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：status(active/inactive)
/// - 返回 FTP 共享列表 + 分页信息
pub async fn list_ftp_shares(
    req: HttpRequest,
    query: web::Query<FtpSharesQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1);
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
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can list FTP shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询 FTP 共享列表
    let status_filter_clone = status_filter.clone();
    match repo.get_shares(page, per_page, Some("ftp".to_string()), status_filter) {
        Ok(shares) => {
            // 5. 转换为响应格式
            let data: Vec<FtpShareInfo> = shares.into_iter().map(|s| FtpShareInfo {
                id: s.id,
                name: s.name,
                path: s.path,
                description: s.description,
                public: s.guest_ok,
                status: s.status,
                created_at: s.created_at,
                updated_at: s.updated_at,
            }).collect();

            // 6. 计算分页信息
            let total = repo.count_shares(Some("ftp".to_string()), status_filter_clone).unwrap_or(data.len() as u64);
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            Ok(HttpResponse::Ok().json(FtpShareListResponse {
                success: true,
                data,
                pagination: PaginationInfo {
                    page,
                    per_page,
                    total,
                    total_pages: total_pages as u32,
                },
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询 FTP 共享列表失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
