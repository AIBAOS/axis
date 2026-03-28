// 会话存储实现（基于 SQLite）
use crate::database::pool::{DbConnectionType};
use rusqlite::{params, OptionalExtension};
use std::sync::{Arc, Mutex};
use crate::services::session_service::Session;

/// SQLite 会话存储实现
pub struct SqliteSessionRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteSessionRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 初始化会话表
    pub fn init_table(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                user_id INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                last_active INTEGER NOT NULL,
                ip_address TEXT,
                user_agent TEXT
            )
        "#).map_err(|e| format!("Create table failed: {}", e))?;
        Ok(())
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

    /// 从 row 构建 Session
    fn row_to_session(row: &rusqlite::Row<'_>) -> Result<Session, rusqlite::Error> {
        Ok(Session {
            id: row.get(0)?,
            user_id: row.get(1)?,
            username: row.get(2).unwrap_or_default(), // username 需要从 users 表 JOIN 获取
            created_at: row.get(3)?,
            last_activity: row.get(4)?,
        })
    }
}

impl Default for SqliteSessionRepository {
    fn default() -> Self {
        panic!("SqliteSessionRepository requires database connection");
    }
}

impl SqliteSessionRepository {
    /// 创建新会话
    pub fn create_session(&self, session: &Session) -> Result<(), String> {
        let conn = self.get_connection()?;
        
        conn.execute(
            r#"
            INSERT INTO sessions (session_id, user_id, created_at, last_active, ip_address, user_agent)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                &session.id,
                session.user_id,
                session.created_at,
                session.last_activity,
                None::<&str>,  // ip_address
                None::<&str>,  // user_agent
            ],
        ).map_err(|e| format!("Insert failed: {}", e))?;
        
        Ok(())
    }

    /// 获取会话
    pub fn get_session(&self, session_id: &str) -> Result<Option<Session>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT session_id, user_id, created_at, last_active
            FROM sessions WHERE session_id = ?1
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let result = stmt
            .query_row(params![session_id], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    username: "".to_string(),
                    created_at: row.get(2)?,
                    last_activity: row.get(3)?,
                })
            })
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;
        
        Ok(result)
    }

    /// 根据用户ID获取会话列表
    pub fn get_sessions_by_user(&self, user_id: u64) -> Result<Vec<Session>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT session_id, user_id, created_at, last_active
            FROM sessions WHERE user_id = ?1
            ORDER BY last_active DESC
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let sessions: Vec<Session> = stmt
            .query_map(params![user_id], |row| {
                Ok(Session {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    username: "".to_string(),
                    created_at: row.get(2)?,
                    last_activity: row.get(3)?,
                })
            })
            .map_err(|e| format!("Query failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(sessions)
    }

    /// 更新最后活动时间
    pub fn update_activity(&self, session_id: &str) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();

        let affected = conn.execute(
            "UPDATE sessions SET last_active = ?1 WHERE session_id = ?2",
            params![now, session_id],
        ).map_err(|e| format!("Update failed: {}", e))?;

        Ok(affected > 0)
    }

    /// 删除会话
    pub fn delete_session(&self, session_id: &str) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM sessions WHERE session_id = ?1",
            params![session_id],
        ).map_err(|e| format!("Delete failed: {}", e))?;

        Ok(affected > 0)
    }
}
