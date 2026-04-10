// Phase 225: 用户列表 API
// GET /api/v1/users — 获取用户列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::user_store::SqliteUserRepository;
use crate::models::user::UserRepository;

/// 用户查询参数
#[derive(Debug, Deserialize)]
pub struct UsersQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub role: Option<String>,
}

/// 用户信息（响应用，不包含敏感信息）
#[derive(Serialize, Clone)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_login: Option<u64>,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 用户列表响应
#[derive(Serialize)]
pub struct UsersListResponse {
    pub success: bool,
    pub data: Vec<UserInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取用户列表（Phase 225）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：role
/// - 返回用户列表 + 分页信息
pub async fn list_users(
    req: HttpRequest,
    query: web::Query<UsersQuery>,
    jwt_service: web::Data<JwtService>,
    user_repo: web::Data<Arc<SqliteUserRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).max(1).min(100); // Bug #90 修复

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
            error: "Only admin users can list users".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库获取用户列表
    match user_repo.get_ref().list_all() {
        Ok(users) => {
            // 5. 应用角色筛选
            let filtered_users: Vec<_> = if let Some(ref role) = query.role {
                users.into_iter().filter(|u| u.roles.contains(role)).collect()
            } else {
                users
            };

            // 6. 应用分页
            let total = filtered_users.len() as u64;
            let start = ((page - 1) * per_page) as usize;
            let end = (start + per_page as usize).min(filtered_users.len());
            
            let users_page = if start < filtered_users.len() {
                filtered_users[start..end].to_vec()
            } else {
                vec![]
            };

            // 7. 转换为响应格式
            let data: Vec<UserInfo> = users_page.into_iter().map(|u| UserInfo {
                id: u.id,
                username: u.username,
                email: u.email,
                roles: u.roles,
                is_active: u.is_active,
                created_at: u.created_at,
                updated_at: u.updated_at,
                last_login: u.last_login,
            }).collect();

            // 8. 计算分页信息
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            Ok(HttpResponse::Ok().json(UsersListResponse {
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
                error: format!("查询用户列表失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
