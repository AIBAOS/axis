// Phase 226 - 用户详情 API（增强版）
// GET /api/v1/users/{id} — 获取用户详情（SQLite 持久化版）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::user_store::SqliteUserRepository;
use crate::models::jwt::JwtClaims;
use crate::services::jwt_service::JwtService;

/// 用户详情（Phase 226 增强版）
#[derive(Serialize, Clone)]
pub struct UserDetail {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_login: Option<u64>,
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

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 获取用户详情（Phase 226）
/// - JWT 认证，登录用户可访问
/// - 权限控制：admin 可查看任意用户，普通用户只能查看自己
/// - 路径参数：id (用户 ID)
/// - 返回字段：id/username/email/roles/is_active/created_at/updated_at/last_login（不含密码）
/// - 用户不存在返回 404 Not Found
pub async fn get_user_detail_v2(
    req: HttpRequest,
    path: web::Path<u64>,
    user_repo: web::Data<SqliteUserRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let target_user_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并提取 claims
    let claims = jwt_service
        .verify_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 权限校验
    // admin 可查看任意用户，普通用户只能查看自己
    let current_user_id: u64 = claims.sub.parse().unwrap_or(0);
    let is_admin_user = is_admin(&claims);

    if !is_admin_user && current_user_id != target_user_id {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "You can only view your own profile".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询用户详情
    let user = match user_repo.get_ref().find_by_id(target_user_id) {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("User {} not found", target_user_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
        Err(e) => {
            log::error!("Failed to get user {}: {}", target_user_id, e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("Database error: {}", e),
                code: "DATABASE_ERROR".to_string(),
            }));
        }
    };

    // 5. 构建响应（不含密码）
    let user_detail = UserDetail {
        id: user.id,
        username: user.username,
        email: user.email,
        roles: user.roles,
        is_active: user.is_active,
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_login: user.last_login,
    };

    // 6. 返回用户详情
    Ok(HttpResponse::Ok().json(UserDetailResponse {
        success: true,
        data: user_detail,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_admin() {
        let admin_claims = JwtClaims {
            sub: "1".to_string(),
            roles: vec!["admin".to_string()],
            exp: 0,
            iat: 0,
        };
        assert!(is_admin(&admin_claims));

        let user_claims = JwtClaims {
            sub: "2".to_string(),
            roles: vec!["user".to_string()],
            exp: 0,
            iat: 0,
        };
        assert!(!is_admin(&user_claims));
    }
}
