// Phase 101: 用户详情 API
// GET /api/v1/users/{id} — 获取用户详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::services::jwt_service::JwtService;

/// 用户详情
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

/// 用户详情（Phase 101）
/// - JWT 认证，需要登录状态
/// - 路径参数：id (用户 ID)
/// - 返回用户详细信息
/// - 用户不存在返回 404
pub async fn get_user_by_id(
    req: HttpRequest,
    path: web::Path<u64>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let user_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 模拟用户数据
    let users = vec![
        UserDetail {
            id: 1,
            username: "admin".to_string(),
            email: "admin@axis.local".to_string(),
            role: "admin".to_string(),
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        UserDetail {
            id: 2,
            username: "user1".to_string(),
            email: "user1@axis.local".to_string(),
            role: "user".to_string(),
            created_at: 1710600000,
            updated_at: 1710600000,
        },
        UserDetail {
            id: 3,
            username: "user2".to_string(),
            email: "user2@axis.local".to_string(),
            role: "user".to_string(),
            created_at: 1710700000,
            updated_at: 1710700000,
        },
        UserDetail {
            id: 4,
            username: "guest".to_string(),
            email: "guest@axis.local".to_string(),
            role: "guest".to_string(),
            created_at: 1710800000,
            updated_at: 1710800000,
        },
    ];

    // 4. 查找指定 ID 的用户
    let user = users.iter().find(|u| u.id == user_id);

    match user {
        Some(u) => {
            Ok(HttpResponse::Ok().json(UserDetailResponse {
                success: true,
                data: u.clone(),
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("User {} not found", user_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
