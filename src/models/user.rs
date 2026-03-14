// 用户模型
// 包含：用户结构体、用户存储接口

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 用户结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub last_login: Option<u64>,
    pub is_active: bool,
}

/// 用户创建请求
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// 用户更新请求
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub roles: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
}

/// 用户响应结构
#[derive(Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<User>,
}

/// 用户存储接口（预留 PostgreSQL 迁移）
pub trait UserRepository {
    fn find_by_username(&self, username: &str) -> Option<User>;
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn create(&self, user: &mut User) -> Result<(), String>;
    fn update(&self, user: &User) -> Result<(), String>;
    fn delete(&self, id: u64) -> Result<(), String>;
    fn list_all(&self) -> Vec<User>;
}
