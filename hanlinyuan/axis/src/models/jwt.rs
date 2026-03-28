// JWT 模型模块
// 包含：JWT 结构体、Token 结构体、配置文件结构

use serde::{Deserialize, Serialize};

/// JWT 配置结构
#[derive(Serialize, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret_key: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_minutes: u64,
    pub refresh_enabled: bool,
}

/// JWT Token 结构
#[derive(Serialize, Deserialize, Clone)]
pub struct JwtToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
}

/// JWT Claims（声明）
#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String,          // 用户 ID
    pub user_id: u64,         // 用户 ID（数字）
    pub username: String,     // 用户名
    pub issuer: String,
    pub audience: String,
    pub exp: u64,             // 过期时间戳
    pub iat: u64,             // 签发时间戳
    pub roles: Vec<String>,   // 用户角色
    pub permissions: Vec<String>, // 用户权限
}

/// JWT 响应结构
#[derive(Serialize, Deserialize, Clone)]
pub struct JwtResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<JwtToken>,
}

/// JWT 中间件需要的数据
#[derive(Clone)]
pub struct JwtMiddlewareData {
    pub config: JwtConfig,
}

impl JwtMiddlewareData {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }
}
