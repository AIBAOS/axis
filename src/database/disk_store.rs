// 磁盘存储模块 — SQLite 持久化
// 包含：建表、分页查询（含 SMART/类型筛选）、详情、健康状态

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 磁盘信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskRow {
    pub id: i64,
    pub name: String,
    pub model: String,
    pub serial: String,
    pub capacity_bytes: i64,
    pub used_bytes: i64,
    pub temperature: i32,
    pub smart_status: String,
    pub health_score: i32,
    pub disk_type: String,
    pub mount_point: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// SQLite 磁盘存储
pub struct SqliteDiskRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteDiskRepository {
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

    /// 初始化磁盘表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS disks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                model TEXT NOT NULL DEFAULT '',
                serial TEXT NOT NULL DEFAULT '',
                capacity_bytes INTEGER NOT NULL DEFAULT 0,
                used_bytes INTEGER NOT NULL DEFAULT 0,
                temperature INTEGER NOT NULL DEFAULT 0,
                smart_status TEXT NOT NULL DEFAULT 'healthy',
                health_score INTEGER NOT NULL DEFAULT 100,
                disk_type TEXT NOT NULL DEFAULT 'hdd',
                mount_point TEXT NOT NULL DEFAULT '',
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_disks_smart_status ON disks(smart_status);
            CREATE INDEX IF NOT EXISTS idx_disks_disk_type ON disks(disk_type);
            CREATE INDEX IF NOT EXISTS idx_disks_name ON disks(name);
        "#).map_err(|e| format!("Init disks table failed: {}", e))
    }

    const SELECT_COLS: &'static str = "id, name, model, serial, capacity_bytes, used_bytes, \
        temperature, smart_status, health_score, disk_type, mount_point, created_at, updated_at";

    fn row_to_disk(row: &rusqlite::Row<'_>) -> Result<DiskRow, rusqlite::Error> {
        Ok(DiskRow {
            id: row.get(0)?,
            name: row.get(1)?,
            model: row.get(2)?,
            serial: row.get(3)?,
            capacity_bytes: row.get(4)?,
            used_bytes: row.get(5)?,
            temperature: row.get(6)?,
            smart_status: row.get(7)?,
            health_score: row.get(8)?,
            disk_type: row.get(9)?,
            mount_point: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
        })
    }

    /// 分页查询磁盘列表
    pub fn get_disks(
        &self,
        smart_status: Option<&str>,
        disk_type: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<DiskRow>, u64), String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * per_page;

        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(st) = smart_status {
            if st != "all" {
                conditions.push(format!("smart_status = ?{}", param_values.len() + 1));
                param_values.push(Box::new(st.to_string()));
            }
        }
        if let Some(dt) = disk_type {
            if dt != "all" {
                conditions.push(format!("disk_type = ?{}", param_values.len() + 1));
                param_values.push(Box::new(dt.to_string()));
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM disks {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        let data_sql = format!(
            "SELECT {} FROM disks {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            Self::SELECT_COLS, where_clause, param_values.len() + 1, param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| Self::row_to_disk(row))
            .map_err(|e| format!("Query failed: {}", e))?;

        let disks: Vec<DiskRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((disks, total))
    }

    /// 根据 ID 查询磁盘详情
    pub fn get_disk_by_id(&self, id: i64) -> Result<Option<DiskRow>, String> {
        let conn = self.get_connection()?;
        let sql = format!("SELECT {} FROM disks WHERE id = ?1", Self::SELECT_COLS);
        let mut stmt = conn.prepare(&sql).map_err(|e| format!("Prepare failed: {}", e))?;

        match stmt.query_row(params![id], |row| Self::row_to_disk(row)) {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 获取磁盘健康状态
    pub fn get_disk_health(&self, id: i64) -> Result<Option<(String, i32, i32)>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT smart_status, health_score, temperature FROM disks WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        match stmt.query_row(params![id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        }) {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 获取磁盘使用量汇总
    pub fn get_disk_usage_summary(&self) -> Result<(i64, i64), String> {
        let conn = self.get_connection()?;
        let (total, used): (i64, i64) = conn.query_row(
            "SELECT COALESCE(SUM(capacity_bytes), 0), COALESCE(SUM(used_bytes), 0) FROM disks",
            params![],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).map_err(|e| format!("Query failed: {}", e))?;
        Ok((total, used))
    }
}
