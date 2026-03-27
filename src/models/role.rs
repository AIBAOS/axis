// 角色模型
// 包含：角色结构体、角色存储接口

use serde::{Deserialize, Serialize};

/// 角色结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_default: bool,
}

/// 角色创建请求
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub is_default: bool,
}

/// 角色响应结构
#[derive(Serialize, Deserialize, Clone)]
pub struct RoleResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Role>,
}

/// 角色存储接口（预留 PostgreSQL 迁移）
pub trait RoleRepository {
    fn find_by_name(&self, name: &str) -> Option<Role>;
    fn find_by_id(&self, id: u64) -> Option<Role>;
    fn create(&self, role: &mut Role) -> Result<(), String>;
    fn update(&self, role: &Role) -> Result<(), String>;
    fn delete(&self, id: u64) -> Result<(), String>;
    fn list_all(&self) -> Vec<Role>;
}
