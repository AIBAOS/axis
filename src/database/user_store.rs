// 用户存储实现（基于 SQLite）
use crate::database::pool::{DbConnectionType};
use rusqlite::{params, OptionalExtension};
use std::sync::{Arc, Mutex};
use serde_json;

/// SQLite 用户存储实现
pub struct SqliteUserRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteUserRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 从数据库连接获取连接
    fn get_connection(&self) -> Result<rusqlite::Connection, String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        match &*guard {
            DbConnectionType::Sqlite(pool) => {
                rusqlite::Connection::open(&pool.path)
                    .map_err(|e| format!("Open failed: {}", e))
            }
            #[cfg(feature = "postgres")]
            DbConnectionType::Postgres(_) => Err("PostgreSQL not implemented".to_string()),
        }
    }

    /// 从 row 构建 User
    fn row_to_user(row: &rusqlite::Row<'_>) -> Result<crate::models::user::User, rusqlite::Error> {
        let roles_json: String = row.get(5)?;
        let permissions_json: String = row.get(6)?;
        
        Ok(crate::models::user::User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            password_salt: row.get(4)?,
            roles: serde_json::from_str(&roles_json).unwrap_or_default(),
            permissions: serde_json::from_str(&permissions_json).unwrap_or_default(),
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            last_login: None,
            is_active: true,
            storage_quota: None,
        })
    }

    /// 将 Vec<String> 转换为 JSON 字符串
    fn vec_to_json<T: serde::Serialize>(v: &Vec<T>) -> String {
        serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string())
    }
}

impl Default for SqliteUserRepository {
    fn default() -> Self {
        panic!("SqliteUserRepository requires database connection");
    }
}

impl crate::models::user::UserRepository for SqliteUserRepository {
    fn find_by_username(&self, username: &str) -> Result<Option<crate::models::user::User>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, username, email, password_hash, password_salt, 
                   roles, permissions, created_at, updated_at
            FROM users WHERE username = ?1
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let result = stmt
            .query_row(params![username], |row| Self::row_to_user(row))
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;
        
        Ok(result)
    }

    fn find_by_id(&self, id: u64) -> Result<Option<crate::models::user::User>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, username, email, password_hash, password_salt,
                   roles, permissions, created_at, updated_at
            FROM users WHERE id = ?1
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let result = stmt
            .query_row(params![id], |row| Self::row_to_user(row))
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;
        
        Ok(result)
    }

    fn find_by_email(&self, email: &str) -> Result<Option<crate::models::user::User>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, username, email, password_hash, password_salt,
                   roles, permissions, created_at, updated_at
            FROM users WHERE email = ?1
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let result = stmt
            .query_row(params![email], |row| Self::row_to_user(row))
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;
        
        Ok(result)
    }

    fn create(&self, user: &mut crate::models::user::User) -> Result<(), String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let roles_json = Self::vec_to_json(&user.roles);
        let permissions_json = Self::vec_to_json(&user.permissions);

        conn.execute(
            r#"
            INSERT INTO users (username, email, password_hash, password_salt, 
                              roles, permissions, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            "#,
            params![
                &user.username,
                &user.email,
                &user.password_hash,
                &user.password_salt,
                &roles_json,
                &permissions_json,
                now,
                now,
            ],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        user.id = conn.last_insert_rowid() as u64;
        Ok(())
    }

    fn update(&self, user: &crate::models::user::User) -> Result<(), String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let roles_json = Self::vec_to_json(&user.roles);
        let permissions_json = Self::vec_to_json(&user.permissions);

        let affected = conn.execute(
            r#"
            UPDATE users SET
                email = ?1,
                password_hash = ?2,
                password_salt = ?3,
                roles = ?4,
                permissions = ?5,
                updated_at = ?6
            WHERE id = ?7
            "#,
            params![
                &user.email,
                &user.password_hash,
                &user.password_salt,
                &roles_json,
                &permissions_json,
                now,
                user.id,
            ],
        ).map_err(|e| format!("Update failed: {}", e))?;

        if affected == 0 {
            return Err("User not found".to_string());
        }
        Ok(())
    }

    fn delete(&self, id: u64) -> Result<(), String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM users WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Delete failed: {}", e))?;

        if affected == 0 {
            return Err("User not found".to_string());
        }
        Ok(())
    }

    fn list_all(&self) -> Result<Vec<crate::models::user::User>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT id, username, email, password_hash, password_salt,
                   roles, permissions, created_at, updated_at
            FROM users
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let users: Vec<crate::models::user::User> = stmt
            .query_map(params![], |row| Self::row_to_user(row))
            .map_err(|e| format!("Query failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(users)
    }
}
