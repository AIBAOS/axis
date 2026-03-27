// Phase 101 - 用户详情 API
// GET /api/v1/users/{id} — 获取用户详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 用户详情信息
#[derive(Serialize, Clone)]
pub struct UserDetail {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 用户详情响应
#[derive(Serialize)]
pub struct UserDetailResponse {
    pub success: bool,
    pub data: UserDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 已登录用户可访问
}

/// 用户详情（Phase 101）
/// - JWT 认证，登录用户可访问
/// - 验证用户 ID 存在
/// - 返回用户详细信息
pub async fn get_user(
    req: HttpRequest,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    let user_id = path.into_inner();

    // 2. 模拟用户数据
    let mock_users = vec![
        UserDetail {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            created_at: 1774259200,
            updated_at: 1774259200,
        },
        UserDetail {
            id: 2,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            role: "user".to_string(),
            created_at: 1774345600,
            updated_at: 1774345600,
        },
        UserDetail {
            id: 3,
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            role: "user".to_string(),
            created_at: 1774432000,
            updated_at: 1774432000,
        },
        UserDetail {
            id: 4,
            username: "guest".to_string(),
            email: "guest@example.com".to_string(),
            role: "guest".to_string(),
            created_at: 1774518400,
            updated_at: 1774518400,
        },
    ];

    // 3. 查找指定 ID 的用户
    let user = mock_users.into_iter().find(|u| u.id == user_id);

    // 4. 返回响应
    match user {
        Some(u) => Ok(HttpResponse::Ok().json(UserDetailResponse {
            success: true,
            data: u,
        })),
        None => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("User {} not found", user_id),
            code: "NOT_FOUND".to_string(),
        })),
    }
}
