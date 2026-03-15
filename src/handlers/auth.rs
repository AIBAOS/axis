// JWT 认证处理器
// 包含：登录、登出、会话管理

use actix_web::{web, HttpResponse, Responder, Result};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use crate::services::jwt_service::{JwtService, generate_salt, hash_password, verify_password};
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

// ==================== 安全密码验证 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let salt = generate_salt();
        let hash = hash_password("test_password", &salt);
        assert!(verify_password("test_password", &salt, &hash));
        assert!(!verify_password("wrong_password", &salt, &hash));
    }
}

/// 验证密码（使用 PBKDF2）
pub fn verify_password_with_hash(password: &str, stored_hash: &str) -> bool {
    verify_password(password, "default_salt_placeholder", stored_hash)
}

// ==================== 示例用户存储 ====================
// 注意：生产环境应从数据库查询，此处仅为开发演示

/// 用户数据结构
pub struct User {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub roles: Vec<String>,
}

// 示例用户（实际应从数据库加载）
fn get_demo_users() -> HashMap<String, User> {
    HashMap::new()  // 空用户池，实际应从 DB 加载
}

// ==================== 登录处理器 ====================

/// 登录处理器
pub async fn login(
    jwt_service: web::Data<JwtService>,
    req: Json<LoginRequest>,
) -> impl Responder {
    let username = req.username.clone();
    let password = req.password.clone();
    
    // TODO: 从数据库查询用户
    // current implementation: placeholder for database lookup
    
    // 示例：从内存用户池查找（仅用于开发测试）
    let users = get_demo_users();
    
    match users.get(&username) {
        Some(user) => {
            // 使用 PBKDF2 验证密码
            if verify_password(&password, &user.salt, &user.password_hash) {
                match jwt_service.generate_token(user.id, &user.username, user.roles.clone(), vec![]) {
                    Ok(response) => {
                        if let Some(token_data) = response.data {
                            let response = LoginResponse {
                                success: true,
                                message: "登录成功".to_string(),
                                data: Some(LoginData {
                                    access_token: token_data.access_token,
                                    token_type: token_data.token_type,
                                    expires_in: token_data.expires_in,
                                    user_id: user.id,
                                    username: user.username.clone(),
                                    roles: user.roles.clone(),
                                }),
                            };
                            HttpResponse::Ok().json(response)
                        } else {
                            HttpResponse::InternalServerError().json(LoginResponse {
                                success: false,
                                message: "Token 生成失败".to_string(),
                                data: None,
                            })
                        }
                    }
                    Err(e) => {
                        HttpResponse::InternalServerError().json(LoginResponse {
                            success: false,
                            message: format!("登录失败: {}", e),
                            data: None,
                        })
                    }
                }
            } else {
                HttpResponse::Unauthorized().json(LoginResponse {
                    success: false,
                    message: "用户名或密码错误".to_string(),
                    data: None,
                })
            }
        }
        None => {
            // 实际应使用固定消息防止用户枚举
            HttpResponse::Unauthorized().json(LoginResponse {
                success: false,
                message: "用户名或密码错误".to_string(),
                data: None,
            })
        }
    }
}

// ==================== 登出处理器 ====================

use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use actix_web::web::Data;
use actix_web::http::header::AUTHORIZATION;

// 全局 Token 黑名单（简化实现）
static BLACKLIST: once_cell::sync::Lazy<Arc<Mutex<HashSet<String>>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(HashSet::new())));

/// 从 Authorization 头提取 token
fn extract_token_from_header(auth_header: &str) -> Result<String, String> {
    let parts: Vec<&str> = auth_header.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err("Invalid Authorization header format".to_string());
    }
    Ok(parts[1].to_string())
}

/// 登出处理器（将 token 加入黑名单）
pub async fn logout(
    jwt_service: Data<JwtService>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // 从 Authorization 头提取 token
    let auth_header = match req.headers().get(AUTHORIZATION) {
        Some(h) => match h.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return HttpResponse::Unauthorized().json(LoginResponse {
                success: false,
                message: "Invalid header encoding".to_string(),
                data: None,
            }),
        },
        None => return HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Missing Authorization header".to_string(),
            data: None,
        }),
    };
    
    let token = match extract_token_from_header(&auth_header) {
        Ok(t) => t,
        Err(e) => return HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: format!("Invalid token: {}", e),
            data: None,
        }),
    };
    
    // 验证 token 并加入黑名单
    if jwt_service.validate_token(&token).is_ok() {
        if let Ok(mut blacklist) = BLACKLIST.lock() {
            blacklist.insert(token.clone());
        }
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "登出成功".to_string(),
            data: None,
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid or expired token".to_string(),
            data: None,
        })
    }
}

// ==================== Token 刷新处理器 ====================

/// Token 刷新处理器
pub async fn refresh_token(
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // TODO: 从 Authorization 头解析 refresh_token
    // 验证 refresh_token 并签发新的 access_token
    
    // 当前返回待实现
    HttpResponse::NotImplemented().json(LoginResponse {
        success: false,
        message: "Token 刷新功能待实现".to_string(),
        data: None,
    })
}
