use crate::models::rbac::{Role, Permission, RbacRepository};
use crate::database::pool::{DbConnectionType, init_rbac_tables};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// SQLite RBAC 存储实现
pub struct SqliteRbacRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteRbacRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 初始化 RBAC 表（委托给 pool::init_rbac_tables）
    pub fn init_tables(&self) -> Result<(), String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        init_rbac_tables(&*guard)
    }

    /// 从数据库连接获取连接
    fn get_connection(&self) -> Result<rusqlite::Connection, String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        match &*guard {
            DbConnectionType::Sqlite(pool) => {
                Connection::open(&pool.path)
                    .map_err(|e| format!("Open failed: {}", e))
            }
            #[cfg(feature = "postgres")]
            DbConnectionType::Postgres(_) => Err("PostgreSQL not implemented".to_string()),
        }
    }

    /// 从 row 构建 Role
    fn row_to_role(row: &rusqlite::Row<'_>) -> Result<Role, rusqlite::Error> {
        Ok(Role {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            permissions: Vec::new(),
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }

    /// 从 row 构建 Permission
    fn row_to_permission(row: &rusqlite::Row<'_>) -> Result<Permission, rusqlite::Error> {
        Ok(Permission {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            resource: row.get(3)?,
            action: row.get(4)?,
        })
    }

    /// 获取所有角色（避免 N+1 查询）
    pub fn list_all_roles(&self) -> Result<Vec<Role>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM roles"
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let roles: Vec<Role> = stmt
            .query_map(params![], |row| Self::row_to_role(row))
            .map_err(|e| format!("Query map failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(roles)
    }

    /// 创建新权限，返回权限 ID
    pub fn create_permission(&self, name: &str, description: &str, resource: &str, action: &str) -> Result<u64, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            r#"
            INSERT INTO permissions (name, description, resource, action, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![name, description, resource, action, now, now],
        ).map_err(|e| format!("Insert permission failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(id as u64)
    }

    /// 给角色分配权限
    pub fn assign_permission_to_role(&self, role_id: u64, permission_id: u64) -> Result<(), String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            r#"
            INSERT INTO roles_permissions (role_id, permission_id, assigned_at)
            VALUES (?1, ?2, ?3)
            "#,
            params![role_id, permission_id, now],
        ).map_err(|e| format!("Insert role_permission failed: {}", e))?;

        Ok(())
    }

    /// 获取角色的所有权限
    pub fn get_permissions_by_role(&self, role_id: u64) -> Result<Vec<Permission>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT p.id, p.name, p.description, p.resource, p.action
            FROM permissions p
            JOIN roles_permissions rp ON p.id = rp.permission_id
            WHERE rp.role_id = ?1
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let permissions: Vec<Permission> = stmt
            .query_map(params![role_id], |row| Self::row_to_permission(row))
            .map_err(|e| format!("Query map failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(permissions)
    }

    /// 获取所有权限
    pub fn list_all_permissions(&self) -> Result<Vec<Permission>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, resource, action FROM permissions"
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let permissions: Vec<Permission> = stmt
            .query_map(params![], |row| Self::row_to_permission(row))
            .map_err(|e| format!("Query map failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(permissions)
    }
}

impl Default for SqliteRbacRepository {
    fn default() -> Self {
        panic!("SqliteRbacRepository requires database connection");
    }
}

impl RbacRepository for SqliteRbacRepository {
    fn create_role(&self, role: &Role) -> Result<u64, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            r#"
            INSERT INTO roles (name, description, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![role.name, role.description, now, now],
        ).map_err(|e| format!("Insert role failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(id as u64)
    }

    /// 删除用户的所有角色关联 (Bug #45 修复)
    /// 在删除用户时调用，确保数据一致性
    pub fn remove_user_roles(&self, user_id: u64) -> Result<(), String> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "DELETE FROM user_roles WHERE user_id = ?1",
            params![user_id],
        ).map_err(|e| format!("Delete user_roles failed: {}", e))?;

        log::info!("Removed all role assignments for user {}", user_id);
        Ok(())
    }

    fn get_role(&self, role_id: u64) -> Option<Role> {
        let conn = self.get_connection().ok()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM roles WHERE id = ?1"
        ).ok()?;
        
        stmt.query_row(params![role_id], |row| Self::row_to_role(row)).ok()
    }

    /// 根据角色名称获取角色
    fn get_role_by_name(&self, name: &str) -> Option<Role> {
        let conn = self.get_connection().ok()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM roles WHERE name = ?1"
        ).ok()?;
        
        stmt.query_row(params![name], |row| Self::row_to_role(row)).ok()
    }

    fn get_roles_by_user(&self, user_id: u64) -> Vec<Role> {
        let conn_result = self.get_connection();
        let conn = match conn_result {
            Ok(conn) => conn,
            Err(_) => return Vec::new(),
        };
        
        let mut stmt = match conn.prepare(
            r#"
            SELECT r.id, r.name, r.description, r.created_at, r.updated_at
            FROM roles r
            JOIN user_roles ur ON r.id = ur.role_id
            WHERE ur.user_id = ?1
            "#,
        ) {
            Ok(stmt) => stmt,
            Err(_) => return Vec::new(),
        };

        let mut roles = Vec::new();
        let mut rows = match stmt.query([user_id]) {
            Ok(rows) => rows,
            Err(_) => return Vec::new(),
        };

        while let Ok(Some(row)) = rows.next() {
            if let Ok(role) = Self::row_to_role(row) {
                roles.push(role);
            }
        }

        roles
    }

    fn check_permission(&self, user_id: u64, resource: &str, action: &str) -> bool {
        let conn = match self.get_connection() {
            Ok(conn) => conn,
            Err(_) => return false,
        };
        
        let mut stmt = match conn.prepare(
            r#"
            SELECT COUNT(*) FROM permissions p
            JOIN roles_permissions rp ON p.id = rp.permission_id
            JOIN user_roles ur ON rp.role_id = ur.role_id
            WHERE ur.user_id = ?1 AND p.resource = ?2 AND p.action = ?3
            "#,
        ) {
            Ok(stmt) => stmt,
            Err(_) => return false,
        };

        let count: i64 = match stmt.query_row(params![user_id, resource, action], |row| row.get(0)) {
            Ok(count) => count,
            Err(_) => return false,
        };
        
        count > 0
    }
}
