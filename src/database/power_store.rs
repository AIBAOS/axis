// 电源操作日志存储模块 — SQLite 持久化

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 电源操作日志
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PowerLogRow {
    pub id: i64,
    pub action: String,
    pub initiated_by: String,
    pub status: String,
    pub created_at: i64,
}

pub struct SqlitePowerRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqlitePowerRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    fn get_connection(&self) -> Result<Connection, String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        match &*guard {
            DbConnectionType::Sqlite(pool) => {
                Connection::open(&pool.path).map_err(|e| format!("Open failed: {}", e))
            }
            #[cfg(feature = "postgres")]
            DbConnectionType::Postgres(_) => Err("PostgreSQL not implemented".to_string()),
        }
    }

    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS power_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action TEXT NOT NULL,
                initiated_by TEXT NOT NULL DEFAULT 'system',
                status TEXT NOT NULL DEFAULT 'pending',
                created_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_power_logs_action ON power_logs(action);
            CREATE INDEX IF NOT EXISTS idx_power_logs_created_at ON power_logs(created_at);
        "#).map_err(|e| format!("Init power_logs table failed: {}", e))
    }

    const SELECT_COLS: &'static str = "id, action, initiated_by, status, created_at";

    fn row_to_power_log(row: &rusqlite::Row<'_>) -> Result<PowerLogRow, rusqlite::Error> {
        Ok(PowerLogRow {
            id: row.get(0)?,
            action: row.get(1)?,
            initiated_by: row.get(2)?,
            status: row.get(3)?,
            created_at: row.get(4)?,
        })
    }

    /// 记录电源操作
    pub fn log_power_action(
        &self,
        action: &str,
        initiated_by: &str,
        status: &str,
    ) -> Result<PowerLogRow, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?.as_secs() as i64;

        conn.execute(
            "INSERT INTO power_logs (action, initiated_by, status, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![action, initiated_by, status, now],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(PowerLogRow {
            id, action: action.into(), initiated_by: initiated_by.into(),
            status: status.into(), created_at: now,
        })
    }

    /// 分页查询电源操作历史
    pub fn get_power_logs(
        &self,
        action: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<PowerLogRow>, u64), String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * per_page;

        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(a) = action {
            if a != "all" {
                conditions.push(format!("action = ?{}", param_values.len() + 1));
                param_values.push(Box::new(a.to_string()));
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM power_logs {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        let data_sql = format!(
            "SELECT {} FROM power_logs {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            Self::SELECT_COLS, where_clause, param_values.len() + 1, param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| Self::row_to_power_log(row))
            .map_err(|e| format!("Query failed: {}", e))?;

        let logs: Vec<PowerLogRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((logs, total))
    }
}
