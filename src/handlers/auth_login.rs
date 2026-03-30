// Phase 52 - 用户登录 API
// POST /api/v1/auth/login — 用户名/密码认证获取 JWT token

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::database::user_store::SqliteUserRepository;
use crate::services::jwt_service::JwtService;
use crate::models::user::UserRepository;

/// 登录请求
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<LoginData>,
}

#[derive(Serialize)]
pub struct LoginData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 登录处理器（Phase 52）
/// - 用户名/密码认证
/// - 密码使用 bcrypt 验证
/// - 成功后返回 JWT token 和用户信息
/// - 失败返回 401 Unauthorized（不泄露具体错误）
pub async fn login(
    payload: web::Json<LoginRequest>,
    user_repo: web::Data<SqliteUserRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let username = &payload.username;
    let password = &payload.password;

    // 1. 验证必要参数
    if username.is_empty() || password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "username and password are required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 2. 查询用户
    let user = match user_repo.find_by_username(username) {
        Ok(Some(u)) => u,
        Ok(None) => {
            // 不泄露用户是否存在
            return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "Invalid username or password".to_string(),
                code: "UNAUTHORIZED".to_string(),
            }));
        }
        Err(e) => {
            log::error!("Database error during login: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Login failed".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            }));
        }
    };

    // 3. 验证密码（bcrypt）
    let password_valid = if user.password_hash.starts_with("$2") {
        // bcrypt 哈希（$2a$, $2b$, $2y$）
        bcrypt::verify(password, &user.password_hash).unwrap_or(false)
    } else {
        // PBKDF2 哈希（向后兼容旧数据）
        let password_hash = crate::services::jwt_service::hash_password(password, &user.password_salt);
        password_hash == user.password_hash
    };

    if !password_valid {
        return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
            success: false,
            error: "Invalid username or password".to_string(),
            code: "UNAUTHORIZED".to_string(),
        }));
    }

    // 4. 检查用户状态
    if !user.is_active {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "User account is inactive".to_string(),
            code: "ACCOUNT_INACTIVE".to_string(),
        }));
    }

    // 5. 生成 JWT Token
    let token_response = match jwt_service.generate_token(
        user.id,
        &user.username,
        user.roles.clone(),
        user.permissions.clone(),
    ) {
        Ok(response) => response,
        Err(e) => {
            log::error!("Token generation failed: {}", e);
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Token generation failed".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            }));
        }
    };

    // 6. 返回登录成功响应
    let token_data = match token_response.data {
        Some(data) => data,
        None => {
            log::error!("Token generation returned no data");
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Token generation failed".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            }));
        }
    };
    
    Ok(HttpResponse::Ok().json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        data: Some(LoginData {
            access_token: token_data.access_token,
            token_type: token_data.token_type,
            expires_in: token_data.expires_in,
            user: UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
                roles: user.roles,
            },
        }),
    }))
}
