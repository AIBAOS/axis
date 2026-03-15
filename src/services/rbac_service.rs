use crate::models::rbac::{Role, Permission, UserRole, RbacRepository};
use std::collections::HashMap;

/// 本地内存 RBAC 存储（待改为数据库持久化）
pub struct MemoryRbacRepository {
    roles: HashMap<u64, Role>,
    permissions: HashMap<u64, Permission>,
    user_roles: HashMap<u64, Vec<u64>>, // user_id -> Vec<role_ids>
}

impl MemoryRbacRepository {
    pub fn new() -> Self {
        Self {
            roles: HashMap::new(),
            permissions: HashMap::new(),
            user_roles: HashMap::new(),
        }
    }

    pub fn init_builtin_roles(&mut self) {
        // 默认管理员角色
        let admin_role = Role {
            id: 1,
            name: "admin".to_string(),
            description: "系统管理员，拥有所有权限".to_string(),
            permissions: vec!["*".to_string()],
            created_at: 0,
            updated_at: 0,
        };
        self.roles.insert(1, admin_role);
    }
}

impl Default for MemoryRbacRepository {
    fn default() -> Self {
        let mut repo = Self::new();
        repo.init_builtin_roles();
        repo
    }
}

impl RbacRepository for MemoryRbacRepository {
    fn create_role(&self, _role: &Role) -> Result<u64, String> {
        Err("Memory storage not implemented".to_string())
    }

    fn get_role(&self, _role_id: u64) -> Option<Role> {
        None
    }

    fn get_roles_by_user(&self, _user_id: u64) -> Vec<Role> {
        Vec::new()
    }

    fn check_permission(&self, _user_id: u64, _resource: &str, _action: &str) -> bool {
        false
    }
}
