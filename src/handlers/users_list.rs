// Phase 161: 用户列表 API
// GET /api/v1/users — 获取用户列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 用户列表查询参数
#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// 用户信息
#[derive(Serialize, Clone)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// 用户列表响应
#[derive(Serialize)]
pub struct UserListResponse {
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

/// 获取用户列表（Phase 161）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), limit(默认 20)
/// - 返回用户列表 + 分页信息
pub async fn list_users(
    req: HttpRequest,
    query: web::Query<UserListQuery>,
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
            error: "Only admin users can list users".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟用户数据
    let all_users = vec![
        UserInfo {
            id: 1,
            username: "admin".to_string(),
            email: "admin@axis.local".to_string(),
            role: "admin".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        UserInfo {
            id: 2,
            username: "user1".to_string(),
            email: "user1@axis.local".to_string(),
            role: "user".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        UserInfo {
            id: 3,
            username: "user2".to_string(),
            email: "user2@axis.local".to_string(),
            role: "user".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        UserInfo {
            id: 4,
            username: "guest".to_string(),
            email: "guest@axis.local".to_string(),
            role: "guest".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 应用分页
    let total = all_users.len() as u64;
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(all_users.len());
    
    let users = if start < all_users.len() {
        all_users[start..end].to_vec()
    } else {
        vec![]
    };

    // 6. 返回用户列表
    Ok(HttpResponse::Ok().json(UserListResponse {
        success: true,
        data: users,
        pagination: PaginationInfo {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}
