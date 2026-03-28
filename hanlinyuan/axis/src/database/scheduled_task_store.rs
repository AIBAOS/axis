// 计划任务存储模块 — SQLite 持久化
// 包含：建表、CRUD、启用/禁用、执行状态更新

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 计划任务信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledTaskRow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub task_type: String,
    pub cron_expression: String,
    pub command: String,
    pub enabled: bool,
    pub status: String,
    pub last_run_at: Option<i64>,
    pub last_run_status: Option<String>,
    pub last_run_duration_ms: Option<i64>,
    pub next_run_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// SQLite 计划任务存储
pub struct SqliteScheduledTaskRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteScheduledTaskRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

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

    /// 初始化计划任务表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS scheduled_tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                task_type TEXT NOT NULL DEFAULT 'system',
                cron_expression TEXT NOT NULL,
                command TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 1,
                status TEXT NOT NULL DEFAULT 'idle',
                last_run_at INTEGER,
                last_run_status TEXT,
                last_run_duration_ms INTEGER,
                next_run_at INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_name ON scheduled_tasks(name);
            CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_enabled ON scheduled_tasks(enabled);
            CREATE INDEX IF NOT EXISTS idx_scheduled_tasks_status ON scheduled_tasks(status);
        "#).map_err(|e| format!("Init scheduled_tasks table failed: {}", e))
    }

    fn row_to_task(row: &rusqlite::Row<'_>) -> Result<ScheduledTaskRow, rusqlite::Error> {
        Ok(ScheduledTaskRow {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            task_type: row.get(3)?,
            cron_expression: row.get(4)?,
            command: row.get(5)?,
            enabled: row.get::<_, i32>(6)? != 0,
            status: row.get(7)?,
            last_run_at: row.get(8)?,
            last_run_status: row.get(9)?,
            last_run_duration_ms: row.get(10)?,
            next_run_at: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    }

    const SELECT_COLS: &'static str = "id, name, description, task_type, cron_expression, command, \
        enabled, status, last_run_at, last_run_status, last_run_duration_ms, next_run_at, created_at, updated_at";

    /// 分页查询
    pub fn get_tasks(
        &self,
        status: Option<&str>,
        enabled: Option<bool>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ScheduledTaskRow>, u64), String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * per_page;

        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(st) = status {
            if st != "all" {
                conditions.push(format!("status = ?{}", param_values.len() + 1));
                param_values.push(Box::new(st.to_string()));
            }
        }
        if let Some(en) = enabled {
            conditions.push(format!("enabled = ?{}", param_values.len() + 1));
            param_values.push(Box::new(if en { 1i32 } else { 0i32 }));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM scheduled_tasks {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        let data_sql = format!(
            "SELECT {} FROM scheduled_tasks {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            Self::SELECT_COLS, where_clause, param_values.len() + 1, param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| Self::row_to_task(row))
            .map_err(|e| format!("Query failed: {}", e))?;

        let tasks: Vec<ScheduledTaskRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((tasks, total))
    }

    /// 根据 ID 查询
    pub fn get_task_by_id(&self, id: i64) -> Result<Option<ScheduledTaskRow>, String> {
        let conn = self.get_connection()?;
        let sql = format!("SELECT {} FROM scheduled_tasks WHERE id = ?1", Self::SELECT_COLS);
        let mut stmt = conn.prepare(&sql).map_err(|e| format!("Prepare failed: {}", e))?;

        match stmt.query_row(params![id], |row| Self::row_to_task(row)) {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 创建计划任务
    pub fn create_task(
        &self,
        name: &str, description: &str, task_type: &str,
        cron_expression: &str, command: &str,
    ) -> Result<ScheduledTaskRow, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?.as_secs() as i64;

        conn.execute(
            "INSERT INTO scheduled_tasks (name, description, task_type, cron_expression, command, enabled, status, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, 1, 'idle', ?6, ?7)",
            params![name, description, task_type, cron_expression, command, now, now],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(ScheduledTaskRow {
            id, name: name.into(), description: description.into(),
            task_type: task_type.into(), cron_expression: cron_expression.into(),
            command: command.into(), enabled: true, status: "idle".into(),
            last_run_at: None, last_run_status: None, last_run_duration_ms: None,
            next_run_at: None, created_at: now, updated_at: now,
        })
    }

    /// 更新计划任务
    pub fn update_task(
        &self, id: i64, name: Option<&str>, description: Option<&str>,
        cron_expression: Option<&str>, command: Option<&str>,
    ) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?.as_secs() as i64;

        let mut sets: Vec<String> = vec!["updated_at = ?1".into()];
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(now)];

        if let Some(v) = name {
            sets.push(format!("name = ?{}", param_values.len() + 1));
            param_values.push(Box::new(v.to_string()));
        }
        if let Some(v) = description {
            sets.push(format!("description = ?{}", param_values.len() + 1));
            param_values.push(Box::new(v.to_string()));
        }
        if let Some(v) = cron_expression {
            sets.push(format!("cron_expression = ?{}", param_values.len() + 1));
            param_values.push(Box::new(v.to_string()));
        }
        if let Some(v) = command {
            sets.push(format!("command = ?{}", param_values.len() + 1));
            param_values.push(Box::new(v.to_string()));
        }

        let sql = format!(
            "UPDATE scheduled_tasks SET {} WHERE id = ?{}",
            sets.join(", "), param_values.len() + 1
        );
        param_values.push(Box::new(id));
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let affected = conn.execute(&sql, params_ref.as_slice())
            .map_err(|e| format!("Update failed: {}", e))?;
        Ok(affected > 0)
    }

    /// 删除计划任务
    pub fn delete_task(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute("DELETE FROM scheduled_tasks WHERE id = ?1", params![id])
            .map_err(|e| format!("Delete failed: {}", e))?;
        Ok(affected > 0)
    }

    /// 启用/禁用切换
    pub fn toggle_task(&self, id: i64) -> Result<Option<bool>, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?.as_secs() as i64;

        // 先读当前状态
        let current: Option<i32> = conn.query_row(
            "SELECT enabled FROM scheduled_tasks WHERE id = ?1", params![id],
            |row| row.get(0),
        ).ok();

        match current {
            Some(en) => {
                let new_val = if en != 0 { 0 } else { 1 };
                conn.execute(
                    "UPDATE scheduled_tasks SET enabled = ?1, updated_at = ?2 WHERE id = ?3",
                    params![new_val, now, id],
                ).map_err(|e| format!("Toggle failed: {}", e))?;
                Ok(Some(new_val != 0))
            }
            None => Ok(None),
        }
    }

    /// 手动执行（标记为 running）
    pub fn run_task(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?.as_secs() as i64;

        let affected = conn.execute(
            "UPDATE scheduled_tasks SET status = 'running', last_run_at = ?1, updated_at = ?2 \
             WHERE id = ?3 AND status != 'running'",
            params![now, now, id],
        ).map_err(|e| format!("Run failed: {}", e))?;
        Ok(affected > 0)
    }
}
