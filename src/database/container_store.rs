// Docker 容器存储模块 — SQLite 持久化
// 包含：建表、分页查询、详情、状态更新

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 容器信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerRow {
    pub id: i64,
    pub container_id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub volumes: String,
    pub env_vars: String,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub stopped_at: Option<i64>,
    pub cpu_usage: f64,
    pub memory_usage: i64,
    pub memory_limit: i64,
}

/// SQLite 容器存储
pub struct SqliteContainerRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteContainerRepository {
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

    /// 初始化容器表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS containers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                container_id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                image TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'created',
                ports TEXT NOT NULL DEFAULT '[]',
                volumes TEXT NOT NULL DEFAULT '[]',
                env_vars TEXT NOT NULL DEFAULT '[]',
                created_at INTEGER NOT NULL,
                started_at INTEGER,
                stopped_at INTEGER,
                cpu_usage REAL NOT NULL DEFAULT 0.0,
                memory_usage INTEGER NOT NULL DEFAULT 0,
                memory_limit INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_containers_name ON containers(name);
            CREATE INDEX IF NOT EXISTS idx_containers_status ON containers(status);
            CREATE INDEX IF NOT EXISTS idx_containers_image ON containers(image);
        "#).map_err(|e| format!("Init containers table failed: {}", e))
    }

    /// 分页查询容器列表
    pub fn get_containers(
        &self,
        status: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ContainerRow>, u64), String> {
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

        // 总数
        let count_sql = format!("SELECT COUNT(*) FROM containers {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;

        // 数据
        let data_sql = format!(
            "SELECT id, container_id, name, image, status, ports, volumes, env_vars, \
             created_at, started_at, stopped_at, cpu_usage, memory_usage, memory_limit \
             FROM containers {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            where_clause,
            param_values.len() + 1,
            param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| {
            Ok(ContainerRow {
                id: row.get(0)?,
                container_id: row.get(1)?,
                name: row.get(2)?,
                image: row.get(3)?,
                status: row.get(4)?,
                ports: row.get(5)?,
                volumes: row.get(6)?,
                env_vars: row.get(7)?,
                created_at: row.get(8)?,
                started_at: row.get(9)?,
                stopped_at: row.get(10)?,
                cpu_usage: row.get(11)?,
                memory_usage: row.get(12)?,
                memory_limit: row.get(13)?,
            })
        }).map_err(|e| format!("Query failed: {}", e))?;

        let containers: Vec<ContainerRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((containers, total))
    }

    /// 根据 ID 查询容器详情
    pub fn get_container_by_id(&self, id: i64) -> Result<Option<ContainerRow>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, container_id, name, image, status, ports, volumes, env_vars, \
             created_at, started_at, stopped_at, cpu_usage, memory_usage, memory_limit \
             FROM containers WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(ContainerRow {
                id: row.get(0)?,
                container_id: row.get(1)?,
                name: row.get(2)?,
                image: row.get(3)?,
                status: row.get(4)?,
                ports: row.get(5)?,
                volumes: row.get(6)?,
                env_vars: row.get(7)?,
                created_at: row.get(8)?,
                started_at: row.get(9)?,
                stopped_at: row.get(10)?,
                cpu_usage: row.get(11)?,
                memory_usage: row.get(12)?,
                memory_limit: row.get(13)?,
            })
        });

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 更新容器状态
    pub fn update_container_status(&self, id: i64, new_status: &str) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let (started_at_sql, stopped_at_sql) = match new_status {
            "running" => ("started_at = ?3, stopped_at = NULL,", ""),
            "stopped" | "exited" => ("stopped_at = ?3,", ""),
            _ => ("", ""),
        };

        let sql = if new_status == "running" {
            format!(
                "UPDATE containers SET status = ?1, started_at = ?3, stopped_at = NULL WHERE id = ?2"
            )
        } else if new_status == "stopped" || new_status == "exited" {
            format!(
                "UPDATE containers SET status = ?1, stopped_at = ?3 WHERE id = ?2"
            )
        } else {
            format!(
                "UPDATE containers SET status = ?1 WHERE id = ?2"
            )
        };

        let affected = if new_status == "running" || new_status == "stopped" || new_status == "exited" {
            conn.execute(&sql, params![new_status, id, now])
                .map_err(|e| format!("Update failed: {}", e))?
        } else {
            conn.execute(&sql, params![new_status, id])
                .map_err(|e| format!("Update failed: {}", e))?
        };

        Ok(affected > 0)
    }
}
