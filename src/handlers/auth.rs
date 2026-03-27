// JWT 认证处理器
// Phase 52: 增强登录接口（bcrypt 密码验证）
// 包含：登录、登出、Token 刷新接口

use actix_web::{web, HttpResponse, Responder, HttpMessage};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;
use crate::models::user::{UserRepository, LoginRequest, LoginResponse, LoginData};
use crate::database::user_store::SqliteUserRepository;

/// 刷新 Token 请求
#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: Option<String>,
}

/// 登录响应（Phase 52 格式）
#[derive(Serialize)]
pub struct LoginResponseV2 {
    pub success: bool,
    pub message: String,
    pub data: Option<LoginDataV2>,
}

#[derive(Serialize)]
pub struct LoginDataV2 {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: Option<UserInfo>,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 登录处理器（Phase 52 增强）
/// - 支持 bcrypt 密码验证
/// - 返回用户基本信息
/// - 失败时不泄露具体错误（安全考虑）
pub async fn login(
    jwt_service: web::Data<JwtService>,
    user_repo: web::Data<SqliteUserRepository>,
    req: actix_web::web::Json<LoginRequest>,
) -> impl Responder {
    let username = &req.username;
    let password = &req.password;

    // 1. 查询用户
    let user_opt = match user_repo.find_by_username(username) {
        Ok(Some(user)) => Some(user),
        Ok(None) => None,
        Err(e) => {
            log::error!("Database error: {}", e);
            let response = LoginResponseV2 {
                success: false,
                message: "登录失败".to_string(),
                data: None,
            };
            return HttpResponse::InternalServerError().json(response);
        }
    };

    let user = match user_opt {
        Some(u) => u,
        None => {
            // 不泄露用户是否存在
            let response = LoginResponseV2 {
                success: false,
                message: "用户名或密码错误".to_string(),
                data: None,
            };
            return HttpResponse::Unauthorized().json(response);
        }
    };

    // 2. 验证密码（bcrypt）
    let password_valid = if user.password_hash.starts_with("$2") {
        // bcrypt 哈希
        bcrypt::verify(password, &user.password_hash).unwrap_or(false)
    } else {
        // PBKDF2 哈希（向后兼容）
        let password_hash = crate::services::jwt_service::hash_password(password, &user.password_salt);
        password_hash == user.password_hash
    };

    if !password_valid {
        let response = LoginResponseV2 {
            success: false,
            message: "用户名或密码错误".to_string(),
            data: None,
        };
        return HttpResponse::Unauthorized().json(response);
    }

    // 3. 生成 JWT Token
    match jwt_service.generate_token(user.id, &user.username, user.roles.clone(), user.permissions.clone()) {
        Ok(token_response) => {
            let token_data = token_response.data.unwrap();
            let response = LoginResponseV2 {
                success: true,
                message: "登录成功".to_string(),
                data: Some(LoginDataV2 {
                    access_token: token_data.access_token,
                    token_type: token_data.token_type,
                    expires_in: token_data.expires_in,
                    user: Some(UserInfo {
                        id: user.id,
                        username: user.username.clone(),
                        email: user.email.clone(),
                        role: user.roles.get(0).cloned().unwrap_or_else(|| "user".to_string()),
                    }),
                }),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Token generation failed: {}", e);
            let response = LoginResponseV2 {
                success: false,
                message: "Token 生成失败".to_string(),
                data: None,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/// 登出处理器
pub async fn logout(
    _jwt_service: web::Data<JwtService>,
) -> impl Responder {
    let response = LoginResponseV2 {
        success: true,
        message: "登出成功".to_string(),
        data: None,
    };
    
    HttpResponse::Ok().json(response)
}

/// Token 刷新处理器
pub async fn refresh_token(
    jwt_service: web::Data<JwtService>,
    _req: actix_web::web::Json<RefreshTokenRequest>,
    route: actix_web::HttpRequest,
) -> impl Responder {
    let claims_opt = route.extensions()
        .get::<crate::models::jwt::JwtClaims>()
        .cloned();

    if let Some(claims) = claims_opt {
        let new_token = jwt_service.generate_token(
            claims.sub.parse().unwrap_or(0),
            &claims.sub,
            claims.roles.clone(),
            claims.permissions.clone(),
        );

        match new_token {
            Ok(token_response) => {
                let token_data = token_response.data.unwrap();
                let response = LoginResponseV2 {
                    success: true,
                    message: "Token 刷新成功".to_string(),
                    data: Some(LoginDataV2 {
                        access_token: token_data.access_token,
                        token_type: token_data.token_type,
                        expires_in: token_data.expires_in,
                        user: None,
                    }),
                };
                HttpResponse::Ok().json(response)
            }
            Err(e) => {
                log::error!("Token refresh failed: {}", e);
                let response = LoginResponseV2 {
                    success: false,
                    message: "Token 刷新失败".to_string(),
                    data: None,
                };
                HttpResponse::InternalServerError().json(response)
            }
        }
    } else {
        let response = LoginResponseV2 {
            success: false,
            message: "未认证".to_string(),
            data: None,
        };
        HttpResponse::Unauthorized().json(response)
    }
}
