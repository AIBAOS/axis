use serde::{Deserialize, Serialize};

/// RBAC 角色定义
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// RBAC 权限定义
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Permission {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

/// 用户角色映射
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRole {
    pub user_id: u64,
    pub role_id: u64,
    pub assigned_at: u64,
}

/// RBAC 存储接口
pub trait RbacRepository: Send + Sync + 'static {
    fn create_role(&self, role: &Role) -> Result<u64, String>;
    fn get_role(&self, role_id: u64) -> Option<Role>;
    /// 根据角色名称获取角色
    fn get_role_by_name(&self, name: &str) -> Option<Role>;
    fn get_roles_by_user(&self, user_id: u64) -> Vec<Role>;
    fn check_permission(&self, user_id: u64, resource: &str, action: &str) -> bool;
    /// 删除用户的所有角色关联
    fn remove_user_roles(&self, user_id: u64) -> Result<(), String>;
}
