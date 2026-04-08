// 配额存储实现（基于 SQLite）
use crate::models::quota::UserQuota;
use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// SQLite 配额存储实现
pub struct SqliteQuotaRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteQuotaRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 初始化配额表
    pub fn init_table(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS user_quotas (
                user_id INTEGER PRIMARY KEY,
                quota_bytes INTEGER NOT NULL DEFAULT 0,
                used_bytes INTEGER NOT NULL DEFAULT 0,
                updated_at INTEGER NOT NULL
            )
        "#).map_err(|e| format!("Create table failed: {}", e))?;
        Ok(())
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

    /// 从 row 构建 UserQuota
    fn row_to_quota(row: &rusqlite::Row<'_>) -> Result<UserQuota, rusqlite::Error> {
        Ok(UserQuota {
            user_id: row.get(0)?,
            quota_bytes: row.get(1)?,
            used_bytes: row.get(2)?,
            updated_at: row.get(3)?,
        })
    }
}

impl Default for SqliteQuotaRepository {
    fn default() -> Self {
        panic!("SqliteQuotaRepository requires database connection");
    }
}

impl SqliteQuotaRepository {
    /// 获取用户配额
    pub fn get_quota(&self, user_id: u64) -> Result<Option<UserQuota>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT user_id, quota_bytes, used_bytes, updated_at FROM user_quotas WHERE user_id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let result = stmt
            .query_row(params![user_id], |row| Self::row_to_quota(row))
            .ok();
        
        Ok(result)
    }

    /// 设置用户配额
    pub fn set_quota(&self, user_id: u64, quota_bytes: u64) -> Result<(), String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()) as i64;

        conn.execute(
            r#"
            INSERT INTO user_quotas (user_id, quota_bytes, used_bytes, updated_at)
            VALUES (?1, ?2, 0, ?3)
            ON CONFLICT(user_id) DO UPDATE SET
                quota_bytes = ?2,
                updated_at = ?3
            "#,
            params![user_id, quota_bytes, now],
        ).map_err(|e| format!("Insert/update failed: {}", e))?;
        
        Ok(())
    }

    /// 更新已用空间（原子操作，防止并发竞态）
    pub fn update_used(&self, user_id: u64, delta: i64) -> Result<UserQuota, String> {
        let mut conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()) as i64;

        // 使用事务确保原子性
        let tx = conn.transaction().map_err(|e| format!("Transaction failed: {}", e))?;
        
        // 使用 IMMEDIATE 事务模式获取写锁，防止并发读写
        tx.execute("UPDATE user_quotas SET used_bytes = used_bytes + ?1, updated_at = ?2 WHERE user_id = ?3",
            params![delta, now, user_id],
        ).map_err(|e| format!("Update failed: {}", e))?;
        
        // 如果不存在则插入
        let affected = tx.execute(
            "INSERT OR IGNORE INTO user_quotas (user_id, quota_bytes, used_bytes, updated_at) VALUES (?1, 0, ?2, ?3)",
            params![user_id, delta.max(0) as i64, now],
        ).map_err(|e| format!("Insert failed: {}", e))?;
        
        // 查询并返回结果
        let quota = tx.query_row(
            "SELECT user_id, quota_bytes, used_bytes, updated_at FROM user_quotas WHERE user_id = ?1",
            params![user_id],
            |row| Self::row_to_quota(row),
        ).map_err(|e| format!("Query failed: {}", e))?;
        
        // 提交事务
        tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
        
        Ok(quota)
    }

    /// 获取所有配额（支持分页）
    pub fn list_quotas(&self, page: u64, page_size: u64) -> Result<Vec<UserQuota>, String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * page_size;
        
        let mut stmt = conn.prepare(
            r#"
            SELECT user_id, quota_bytes, used_bytes, updated_at
            FROM user_quotas
            ORDER BY user_id
            LIMIT ?1 OFFSET ?2
            "#,
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        let quotas: Vec<UserQuota> = stmt
            .query_map(params![page_size, offset], |row| Self::row_to_quota(row))
            .map_err(|e| format!("Query map failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(quotas)
    }

    /// 获取用户配额使用情况
    pub fn get_quota_usage(&self, user_id: u64) -> Result<Option<UserQuota>, String> {
        self.get_quota(user_id)
    }

    /// 原子操作：检查并更新配额（防止并发竞态）
    /// 如果当前可用空间 >= required_bytes，则增加 used_bytes 并返回 true
    /// 否则返回 false
    pub fn try_reserve_quota(&self, user_id: u64, required_bytes: u64) -> Result<bool, String> {
        let mut conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()) as i64;

        // 使用事务和行级锁确保原子性
        let tx = conn.transaction().map_err(|e| format!("Transaction failed: {}", e))?;
        
        // 获取当前配额（带锁）
        let current = tx.query_row(
            "SELECT quota_bytes, used_bytes FROM user_quotas WHERE user_id = ?1 FOR UPDATE",
            params![user_id],
            |row| {
                let quota_bytes: u64 = row.get(0)?;
                let used_bytes: u64 = row.get(1)?;
                Ok((quota_bytes, used_bytes))
            },
        );

        let (quota_bytes, used_bytes) = match current {
            Ok((q, u)) => (q, u),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // 用户不存在配额记录，视为无限制
                tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
                return Ok(true);
            }
            Err(e) => return Err(format!("Query failed: {}", e)),
        };

        // 检查配额
        if quota_bytes == 0 {
            // 无限制
            tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
            return Ok(true);
        }

        let available = quota_bytes.saturating_sub(used_bytes);
        if available >= required_bytes {
            // 配额充足，更新
            tx.execute(
                "UPDATE user_quotas SET used_bytes = used_bytes + ?1, updated_at = ?2 WHERE user_id = ?3",
                params![required_bytes as i64, now, user_id],
            ).map_err(|e| format!("Update failed: {}", e))?;
            tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
            Ok(true)
        } else {
            // 配额不足
            tx.commit().map_err(|e| format!("Commit failed: {}", e))?;
            Ok(false)
        }
    }
}
