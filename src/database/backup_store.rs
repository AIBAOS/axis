// 备份任务存储模块 — SQLite 持久化
// 包含：建表、分页查询、创建、删除、执行状态更新

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// 备份任务信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupRow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
    pub status: String,
    pub last_run_at: Option<i64>,
    pub last_run_status: Option<String>,
    pub last_run_size_bytes: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// SQLite 备份存储
pub struct SqliteBackupRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteBackupRepository {
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

    /// 初始化备份表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS backups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                backup_type TEXT NOT NULL DEFAULT 'full',
                source_path TEXT NOT NULL,
                destination_path TEXT NOT NULL,
                schedule TEXT,
                status TEXT NOT NULL DEFAULT 'idle',
                last_run_at INTEGER,
                last_run_status TEXT,
                last_run_size_bytes INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_backups_name ON backups(name);
            CREATE INDEX IF NOT EXISTS idx_backups_status ON backups(status);
            CREATE INDEX IF NOT EXISTS idx_backups_type ON backups(backup_type);
        "#).map_err(|e| format!("Init backups table failed: {}", e))
    }

    /// 分页查询备份列表
    pub fn get_backups(
        &self,
        status: Option<&str>,
        backup_type: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<BackupRow>, u64), String> {
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
        if let Some(bt) = backup_type {
            conditions.push(format!("backup_type = ?{}", param_values.len() + 1));
            param_values.push(Box::new(bt.to_string()));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM backups {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;

        let data_sql = format!(
            "SELECT id, name, description, backup_type, source_path, destination_path, \
             schedule, status, last_run_at, last_run_status, last_run_size_bytes, created_at, updated_at \
             FROM backups {} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            where_clause,
            param_values.len() + 1,
            param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| {
            Ok(BackupRow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                backup_type: row.get(3)?,
                source_path: row.get(4)?,
                destination_path: row.get(5)?,
                schedule: row.get(6)?,
                status: row.get(7)?,
                last_run_at: row.get(8)?,
                last_run_status: row.get(9)?,
                last_run_size_bytes: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        }).map_err(|e| format!("Query failed: {}", e))?;

        let backups: Vec<BackupRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((backups, total))
    }

    /// 根据 ID 查询
    pub fn get_backup_by_id(&self, id: i64) -> Result<Option<BackupRow>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, backup_type, source_path, destination_path, \
             schedule, status, last_run_at, last_run_status, last_run_size_bytes, created_at, updated_at \
             FROM backups WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(BackupRow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                backup_type: row.get(3)?,
                source_path: row.get(4)?,
                destination_path: row.get(5)?,
                schedule: row.get(6)?,
                status: row.get(7)?,
                last_run_at: row.get(8)?,
                last_run_status: row.get(9)?,
                last_run_size_bytes: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        });

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 创建备份任务
    pub fn create_backup(
        &self,
        name: &str,
        description: &str,
        backup_type: &str,
        source_path: &str,
        destination_path: &str,
        schedule: Option<&str>,
    ) -> Result<BackupRow, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        conn.execute(
            "INSERT INTO backups (name, description, backup_type, source_path, destination_path, schedule, status, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'idle', ?7, ?8)",
            params![name, description, backup_type, source_path, destination_path, schedule, now, now],
        ).map_err(|e| format!("Insert failed: {}", e))?;

        let id = conn.last_insert_rowid();
        Ok(BackupRow {
            id,
            name: name.to_string(),
            description: description.to_string(),
            backup_type: backup_type.to_string(),
            source_path: source_path.to_string(),
            destination_path: destination_path.to_string(),
            schedule: schedule.map(|s| s.to_string()),
            status: "idle".to_string(),
            last_run_at: None,
            last_run_status: None,
            last_run_size_bytes: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// 删除备份任务
    pub fn delete_backup(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM backups WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Delete failed: {}", e))?;
        Ok(affected > 0)
    }

    /// 手动执行备份（更新状态为 running）
    pub fn run_backup(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let affected = conn.execute(
            "UPDATE backups SET status = 'running', last_run_at = ?1, updated_at = ?2 WHERE id = ?3 AND status != 'running'",
            params![now, now, id],
        ).map_err(|e| format!("Update failed: {}", e))?;
        Ok(affected > 0)
    }

    /// 恢复备份任务（状态从 archived → active）
    pub fn restore_backup(&self, id: i64) -> Result<Option<BackupRow>, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        // 先检查备份是否存在且状态为 archived
        let backup = self.get_backup_by_id(id)?;
        match &backup {
            Some(b) => {
                if b.status != "archived" {
                    return Err(format!("备份状态为 '{}'，仅 archived 状态的备份可恢复", b.status));
                }
            }
            None => return Ok(None),
        }

        // 更新状态为 active
        let affected = conn.execute(
            "UPDATE backups SET status = 'active', updated_at = ?1 WHERE id = ?2 AND status = 'archived'",
            params![now, id],
        ).map_err(|e| format!("Update failed: {}", e))?;

        if affected == 0 {
            return Ok(None);
        }

        // 返回恢复后的备份信息
        self.get_backup_by_id(id)
    }

    /// 归档备份任务（状态从 active/completed → archived）
    pub fn archive_backup(&self, id: i64) -> Result<Option<BackupRow>, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        // 先检查备份是否存在
        let backup = self.get_backup_by_id(id)?;
        match &backup {
            Some(b) => {
                // 允许归档 active 或 completed 状态的备份
                if b.status != "active" && b.status != "completed" {
                    if b.status == "archived" {
                        return Err("Backup is already archived".to_string());
                    }
                    return Err(format!("Backup status is '{}'. Only active or completed backups can be archived", b.status));
                }
            }
            None => return Ok(None),
        }

        // 更新状态为 archived
        let affected = conn.execute(
            "UPDATE backups SET status = 'archived', updated_at = ?1 WHERE id = ?2 AND (status = 'active' OR status = 'completed')",
            params![now, id],
        ).map_err(|e| format!("Update failed: {}", e))?;

        if affected == 0 {
            return Ok(None);
        }

        // 返回归档后的备份信息
        self.get_backup_by_id(id)
    }
}
