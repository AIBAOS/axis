// 系统更新记录存储模块 — SQLite 持久化

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 更新记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateRecordRow {
    pub id: i64,
    pub version: String,
    pub release_notes: String,
    pub status: String,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

pub struct SqliteUpdateRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteUpdateRepository {
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
            CREATE TABLE IF NOT EXISTS update_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL,
                release_notes TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'pending',
                created_at INTEGER NOT NULL,
                completed_at INTEGER
            );

            CREATE INDEX IF NOT EXISTS idx_update_records_status ON update_records(status);
            CREATE INDEX IF NOT EXISTS idx_update_records_version ON update_records(version);
        "#).map_err(|e| format!("Init update_records table failed: {}", e))
    }

    const SELECT_COLS: &'static str = "id, version, release_notes, status, created_at, completed_at";

    fn row_to_update_record(row: &rusqlite::Row<'_>) -> Result<UpdateRecordRow, rusqlite::Error> {
        Ok(UpdateRecordRow {
            id: row.get(0)?,
            version: row.get(1)?,
            release_notes: row.get(2)?,
            status: row.get(3)?,
            created_at: row.get(4)?,
            completed_at: row.get(5)?,
        })
    }

    /// 获取最新已安装版本
    pub fn get_current_version(&self) -> Result<Option<String>, String> {
        let conn = self.get_connection()?;
        let result = conn.query_row(
            "SELECT version FROM update_records WHERE status = 'success' ORDER BY completed_at DESC LIMIT 1",
            params![],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(v) => Ok(Some(v)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 分页查询更新历史
    pub fn get_update_history(
        &self,
        status: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<UpdateRecordRow>, u64), String> {
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

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM update_records {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        let data_sql = format!(
            "SELECT {} FROM update_records {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            Self::SELECT_COLS, where_clause, param_values.len() + 1, param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| Self::row_to_update_record(row))
            .map_err(|e| format!("Query failed: {}", e))?;

        let records: Vec<UpdateRecordRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((records, total))
    }
}
