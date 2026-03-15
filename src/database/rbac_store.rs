use crate::models::rbac::{Role, RbacRepository};
use std::sync::Mutex;

/// SQLite RBAC 存储实现
pub struct SqliteRbacRepository {
    pool: Mutex<()>, // 占位，实际实现需集成数据库连接池
}

impl SqliteRbacRepository {
    pub fn new() -> Self {
        Self {
            pool: Mutex::new(()),
        }
    }

    /// 初始化 RBAC 表
    pub fn init_tables(&self) -> Result<(), String> {
        // 预留：执行 CREATE TABLE IF NOT EXISTS
        // users, roles, permissions, user_roles
        Ok(())
    }
}

impl Default for SqliteRbacRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl RbacRepository for SqliteRbacRepository {
    fn create_role(&self, _role: &Role) -> Result<u64, String> {
        Err("Database not implemented".to_string())
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
