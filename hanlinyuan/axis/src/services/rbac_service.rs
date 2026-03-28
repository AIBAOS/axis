use std::sync::Arc;
use crate::models::rbac::{Role, Permission, RbacRepository};
use crate::database::rbac_store::SqliteRbacRepository;

/// RBAC 服务层（整合数据库存储）
pub struct RbacService {
    repository: Arc<SqliteRbacRepository>,
}

impl RbacService {
    pub fn new(repository: Arc<SqliteRbacRepository>) -> Self {
        Self { repository }
    }

    /// 创建新角色
    pub fn create_role(&self, name: &str, description: &str) -> Result<u64, String> {
        let role = Role {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            permissions: Vec::new(),
            created_at: 0,
            updated_at: 0,
        };

        self.repository.create_role(&role)
    }

    /// 根据 ID 获取角色
    pub fn get_role(&self, role_id: u64) -> Option<Role> {
        self.repository.get_role(role_id)
    }

    /// 获取所有角色列表（优化：单次 SQL 查询）
    pub fn list_roles(&self) -> Vec<Role> {
        match self.repository.list_all_roles() {
            Ok(roles) => roles,
            Err(_) => Vec::new(),
        }
    }

    /// 根据用户 ID 获取角色列表
    pub fn get_roles_by_user(&self, user_id: u64) -> Vec<Role> {
        self.repository.get_roles_by_user(user_id)
    }

    /// 检查用户权限
    pub fn check_permission(&self, user_id: u64, resource: &str, action: &str) -> bool {
        self.repository.check_permission(user_id, resource, action)
    }

    /// 创建新权限
    pub fn create_permission(&self, name: &str, description: &str, resource: &str, action: &str) -> Result<u64, String> {
        self.repository.create_permission(name, description, resource, action)
    }

    /// 给角色分配权限
    pub fn assign_permission_to_role(&self, role_id: u64, permission_id: u64) -> Result<(), String> {
        self.repository.assign_permission_to_role(role_id, permission_id)
    }

    /// 获取权限列表
    pub fn list_permissions(&self) -> Vec<Permission> {
        match self.repository.list_all_permissions() {
            Ok(perms) => perms,
            Err(_) => Vec::new(),
        }
    }
}
