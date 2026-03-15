use actix_web::{web, HttpResponse, Responder, Result};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::services::jwt_service::JwtService;
use crate::database::pool::{DbConnection, DbPool, create_sqlite_pool, init_rbac_tables};

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
    pub user_id: u64,
    pub username: String,
    pub roles: Vec<String>,
}

/// 实际的密码检查（示例实现）
fn check_credentials(username: &str, password: &str) -> Option<(u64, String, Vec<String>)> {
    // 示例用户数据（实际应查询数据库）
    let users: HashMap<&str, (&str, Vec<String>)> = HashMap::from([
        ("admin", ("admin123", vec!["admin".to_string()])),
        ("user", ("user123", vec!["user".to_string()])),
    ]);
    
    users.get(username)
        .and_then(|(stored_hash, roles)| {
            // 简化验证：直接比较（实际应使用 hash 比较）
            if *stored_hash == password {
                Some((1, username.to_string(), roles.clone()))
            } else {
                None
            }
        })
}

/// 登录处理器
pub async fn login(
    jwt_service: web::Data<JwtService>,
    req: Json<LoginRequest>,
) -> impl Responder {
    let username = req.username.clone();
    let password = req.password.clone();
    
    match check_credentials(&username, &password) {
        Some((user_id, username_str, roles)) => {
            match jwt_service.generate_token(user_id, &username_str, roles, vec![]) {
                Ok(response) => {
                    if let Some(token_data) = response.data {
                        let response = LoginResponse {
                            success: true,
                            message: "登录成功".to_string(),
                            data: Some(LoginData {
                                access_token: token_data.access_token,
                                token_type: token_data.token_type,
                                expires_in: token_data.expires_in,
                                user_id,
                                username: username_str,
                                roles: vec!["user".to_string()],
                            }),
                        };
                        HttpResponse::Ok().json(response)
                    } else {
                        let response = LoginResponse {
                            success: false,
                            message: "Token 生成失败".to_string(),
                            data: None,
                        };
                        HttpResponse::InternalServerError().json(response)
                    }
                }
                Err(e) => {
                    let response = LoginResponse {
                        success: false,
                        message: format!("登录失败: {}", e),
                        data: None,
                    };
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        None => {
            let response = LoginResponse {
                success: false,
                message: "用户名或密码错误".to_string(),
                data: None,
            };
            HttpResponse::Unauthorized().json(response)
        }
    }
}

/// 登出处理器（暂存处理，实际应实现 Token 黑名单）
pub async fn logout(
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // TODO: 实现 Token 黑名单或短 Token 失效逻辑
    
    let response = LoginResponse {
        success: true,
        message: "登出成功".to_string(),
        data: None,
    };
    
    HttpResponse::Ok().json(response)
}

/// Token 刷新处理器（暂存处理）
pub async fn refresh_token(
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // TODO: 实现 Token 刷新逻辑
    
    let response = LoginResponse {
        success: false,
        message: "Token 刷新功能待实现".to_string(),
        data: None,
    };
    
    HttpResponse::Ok().json(response)
}
