// Phase 102 - 创建用户 API
// POST /api/v1/users — 创建用户

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use regex::Regex;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::database::user_store::SqliteUserRepository;
use crate::models::rbac::RbacRepository;
use crate::models::user::UserRepository;
use crate::services::jwt_service::JwtService;

/// 创建用户请求
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: Option<String>,
}

/// 用户信息
#[derive(Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 创建用户响应
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
    pub data: UserInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 验证邮箱格式（Bug #19 修复）
/// 使用正则表达式验证标准邮箱格式
fn validate_email(email: &str) -> bool {
    // 长度检查（RFC 5321 最大 254 字符）
    if email.len() > 254 || email.len() < 5 {
        return false;
    }
    
    // 正则表达式验证邮箱格式
    // 规则：local@domain.tld
    // - local: 字母、数字、._%+-（不能以.开头或结尾）
    // - domain: 字母、数字、.-（必须包含至少一个.）
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9]([a-zA-Z0-9._%+-]*[a-zA-Z0-9])?@[a-zA-Z0-9]([a-zA-Z0-9.-]*[a-zA-Z0-9])?\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    email_regex.is_match(email)
}

/// 创建用户（Phase 102）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证用户名唯一性（409 Conflict）
/// - 验证邮箱格式（400 Bad Request）
/// - 验证密码强度（400 Bad Request）
/// - 创建成功返回 201 Created
pub async fn create_user(
    req: HttpRequest,
    payload: web::Json<CreateUserRequest>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    user_store: web::Data<SqliteUserRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 仅 admin 角色可创建用户
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create users".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 验证必要参数
    let username = &payload.username;
    let password = &payload.password;
    let email = &payload.email;
    let role = payload.role.as_deref().unwrap_or("user");

    if username.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "username is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "password is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if email.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "email is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 验证密码强度（至少 8 位）
    if password.len() < 8 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "password must be at least 8 characters long".to_string(),
            code: "WEAK_PASSWORD".to_string(),
        }));
    }

    // 5. 验证邮箱格式（Bug #19 修复：使用正则验证）
    if !validate_email(email) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "invalid email format. expected format: user@domain.tld".to_string(),
            code: "INVALID_EMAIL".to_string(),
        }));
    }

    // 6. 验证角色
    let valid_roles = vec!["admin", "user", "guest"];
    if !valid_roles.contains(&role) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Invalid role '{}'. Valid roles: {}", role, valid_roles.join(", ")),
            code: "INVALID_ROLE".to_string(),
        }));
    }

    // 7. 用户名唯一性检查（Bug #20 修复：查询数据库验证）
    match user_store.find_by_username(username) {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("Username '{}' already exists", username),
                code: "CONFLICT".to_string(),
            }));
        }
        Ok(None) => {
            // 用户名可用，继续创建
        }
        Err(e) => {
            log::error!("Database error checking username: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to check username availability".to_string(),
                code: "DATABASE_ERROR".to_string(),
            }));
        }
    }

    // 8. 模拟创建用户
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
        .as_secs();

    let new_user = UserInfo {
        id: 100,
        username: username.clone(),
        email: email.clone(),
        role: role.to_string(),
        created_at: now,
        updated_at: now,
    };

    Ok(HttpResponse::Created().json(CreateUserResponse {
        success: true,
        message: "User created successfully".to_string(),
        data: new_user,
    }))
}
