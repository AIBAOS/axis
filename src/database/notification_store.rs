// 通知存储模块 — SQLite 持久化
// 包含：建表、分页查询、创建、标记已读、删除

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 通知结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationRow {
    pub id: i64,
    pub title: String,
    pub message: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub priority: String,
    pub target_user_id: Option<i64>,
    pub is_read: bool,
    pub created_at: i64,
    pub read_at: Option<i64>,
    pub action_url: Option<String>,
}

/// SQLite 通知存储
pub struct SqliteNotificationRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteNotificationRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 获取数据库连接
    fn get_connection(&self) -> Result<Connection, String> {
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

    /// 初始化通知表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS notifications (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                message TEXT NOT NULL,
                type TEXT NOT NULL DEFAULT 'info',
                priority TEXT NOT NULL DEFAULT 'normal',
                target_user_id INTEGER,
                is_read INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                read_at INTEGER,
                action_url TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);
            CREATE INDEX IF NOT EXISTS idx_notifications_target_user ON notifications(target_user_id);
            CREATE INDEX IF NOT EXISTS idx_notifications_is_read ON notifications(is_read);
        "#).map_err(|e| format!("Init notifications table failed: {}", e))
    }

    /// 分页查询通知
    pub fn get_notifications(
        &self,
        notification_type: Option<&str>,
        status: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> Result<(Vec<NotificationRow>, u64), String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * page_size;

        // 构建 WHERE 条件
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(nt) = notification_type {
            conditions.push(format!("type = ?{}", param_values.len() + 1));
            param_values.push(Box::new(nt.to_string()));
        }
        if let Some(st) = status {
            let is_read = if st == "read" { 1 } else { 0 };
            conditions.push(format!("is_read = ?{}", param_values.len() + 1));
            param_values.push(Box::new(is_read));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 查询总数
        let count_sql = format!("SELECT COUNT(*) FROM notifications {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;

        // 查询数据
        let data_sql = format!(
            "SELECT id, title, message, type, priority, target_user_id, is_read, created_at, read_at, action_url \
             FROM notifications {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            where_clause,
            param_values.len() + 1,
            param_values.len() + 2,
        );
        param_values.push(Box::new(page_size as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| {
            Ok(NotificationRow {
                id: row.get(0)?,
                title: row.get(1)?,
                message: row.get(2)?,
                notification_type: row.get(3)?,
                priority: row.get(4)?,
                target_user_id: row.get(5)?,
                is_read: row.get::<_, i32>(6)? != 0,
                created_at: row.get(7)?,
                read_at: row.get(8)?,
                action_url: row.get(9)?,
            })
        }).map_err(|e| format!("Query failed: {}", e))?;

        let notifications: Vec<NotificationRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((notifications, total))
    }

    /// 根据 ID 查询单条通知
    pub fn get_notification_by_id(&self, id: i64) -> Result<Option<NotificationRow>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, title, message, type, priority, target_user_id, is_read, created_at, read_at, action_url \
             FROM notifications WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(NotificationRow {
                id: row.get(0)?,
                title: row.get(1)?,
                message: row.get(2)?,
                notification_type: row.get(3)?,
                priority: row.get(4)?,
                target_user_id: row.get(5)?,
                is_read: row.get::<_, i32>(6)? != 0,
                created_at: row.get(7)?,
                read_at: row.get(8)?,
                action_url: row.get(9)?,
            })
        });

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 创建通知
    pub fn create_notification(
        &self,
        title: &str,
        message: &str,
        notification_type: &str,
        priority: &str,
        target_user_id: Option<i64>,
        action_url: Option<&str>,
    ) -> Result<NotificationRow, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO notifications (title, message, type, priority, target_user_id, is_read, created_at, action_url) \
             VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7)",
            params![title, message, notification_type, priority, target_user_id, now, action_url],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(NotificationRow {
            id,
            title: title.to_string(),
            message: message.to_string(),
            notification_type: notification_type.to_string(),
            priority: priority.to_string(),
            target_user_id,
            is_read: false,
            created_at: now,
            read_at: None,
            action_url: action_url.map(|s| s.to_string()),
        })
    }

    /// 标记为已读
    pub fn mark_as_read(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let affected = conn.execute(
            "UPDATE notifications SET is_read = 1, read_at = ?1 WHERE id = ?2",
            params![now, id],
        ).map_err(|e| format!("Update failed: {}", e))?;

        Ok(affected > 0)
    }

    /// 删除通知
    pub fn delete_notification(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM notifications WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Delete failed: {}", e))?;

        Ok(affected > 0)
    }

    /// 删除所有已读通知
    pub fn delete_read_notifications(&self) -> Result<u64, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM notifications WHERE is_read = 1",
            params![],
        ).map_err(|e| format!("Delete failed: {}", e))?;

        Ok(affected as u64)
    }
}
