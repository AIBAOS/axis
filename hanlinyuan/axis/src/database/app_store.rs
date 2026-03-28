// 应用/插件存储模块 — SQLite 持久化
// 包含：建表、分页查询、创建、更新、删除

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 应用信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppRow {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub description: String,
    pub status: String,
    pub icon_url: Option<String>,
    pub category: String,
    pub installed_at: Option<i64>,
    pub updated_at: i64,
    pub size_bytes: i64,
}

/// SQLite 应用存储
pub struct SqliteAppRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteAppRepository {
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

    /// 初始化应用表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS apps (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                version TEXT NOT NULL DEFAULT '0.0.0',
                description TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'available',
                icon_url TEXT,
                category TEXT NOT NULL DEFAULT 'other',
                installed_at INTEGER,
                updated_at INTEGER NOT NULL,
                size_bytes INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_apps_name ON apps(name);
            CREATE INDEX IF NOT EXISTS idx_apps_status ON apps(status);
            CREATE INDEX IF NOT EXISTS idx_apps_category ON apps(category);
        "#).map_err(|e| format!("Init apps table failed: {}", e))
    }

    /// 分页查询应用列表
    pub fn get_apps(
        &self,
        status: Option<&str>,
        category: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<AppRow>, u64), String> {
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
        if let Some(cat) = category {
            conditions.push(format!("category = ?{}", param_values.len() + 1));
            param_values.push(Box::new(cat.to_string()));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 总数
        let count_sql = format!("SELECT COUNT(*) FROM apps {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;

        // 数据
        let data_sql = format!(
            "SELECT id, name, version, description, status, icon_url, category, installed_at, updated_at, size_bytes \
             FROM apps {} ORDER BY name ASC LIMIT ?{} OFFSET ?{}",
            where_clause,
            param_values.len() + 1,
            param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| {
            Ok(AppRow {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                icon_url: row.get(5)?,
                category: row.get(6)?,
                installed_at: row.get(7)?,
                updated_at: row.get(8)?,
                size_bytes: row.get(9)?,
            })
        }).map_err(|e| format!("Query failed: {}", e))?;

        let apps: Vec<AppRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((apps, total))
    }

    /// 根据 ID 查询
    pub fn get_app_by_id(&self, id: i64) -> Result<Option<AppRow>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, version, description, status, icon_url, category, installed_at, updated_at, size_bytes \
             FROM apps WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(AppRow {
                id: row.get(0)?,
                name: row.get(1)?,
                version: row.get(2)?,
                description: row.get(3)?,
                status: row.get(4)?,
                icon_url: row.get(5)?,
                category: row.get(6)?,
                installed_at: row.get(7)?,
                updated_at: row.get(8)?,
                size_bytes: row.get(9)?,
            })
        });

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 安装应用
    pub fn install_app(
        &self,
        name: &str,
        version: &str,
        description: &str,
        icon_url: Option<&str>,
        category: &str,
        size_bytes: i64,
    ) -> Result<AppRow, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO apps (name, version, description, status, icon_url, category, installed_at, updated_at, size_bytes) \
             VALUES (?1, ?2, ?3, 'installed', ?4, ?5, ?6, ?7, ?8)",
            params![name, version, description, icon_url, category, now, now, size_bytes],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(AppRow {
            id,
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            status: "installed".to_string(),
            icon_url: icon_url.map(|s| s.to_string()),
            category: category.to_string(),
            installed_at: Some(now),
            updated_at: now,
            size_bytes,
        })
    }

    /// 卸载应用
    pub fn uninstall_app(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM apps WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Delete failed: {}", e))?;
        Ok(affected > 0)
    }
}
