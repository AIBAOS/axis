// 定时任务存储实现（基于 SQLite）
use crate::database::pool::DbConnectionType;
use rusqlite::{params, OptionalExtension};
use std::sync::{Arc, Mutex};

/// 定时任务信息
#[derive(Debug, Clone)]
pub struct CronJob {
    pub id: u64,
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub status: String,
    pub last_run: Option<i64>,
    pub next_run: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// SQLite 定时任务存储
pub struct SqliteCronJobRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteCronJobRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

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

    /// 初始化定时任务表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS cron_jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                schedule TEXT NOT NULL,
                command TEXT NOT NULL,
                description TEXT,
                enabled INTEGER NOT NULL DEFAULT 1,
                status TEXT NOT NULL DEFAULT 'active',
                last_run INTEGER,
                next_run INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_cron_jobs_name ON cron_jobs(name);
            CREATE INDEX IF NOT EXISTS idx_cron_jobs_status ON cron_jobs(status);
            CREATE INDEX IF NOT EXISTS idx_cron_jobs_enabled ON cron_jobs(enabled);
        "#).map_err(|e| format!("Init cron_jobs table failed: {}", e))
    }

    /// 获取所有定时任务
    pub fn get_all_jobs(&self) -> Result<Vec<CronJob>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, schedule, command, description, enabled, status, last_run, next_run, created_at, updated_at 
             FROM cron_jobs ORDER BY created_at DESC"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let jobs = stmt
            .query_map(params![], |row| {
                Ok(CronJob {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    schedule: row.get(2)?,
                    command: row.get(3)?,
                    description: row.get(4)?,
                    enabled: row.get::<_, i32>(5)? != 0,
                    status: row.get(6)?,
                    last_run: row.get(7)?,
                    next_run: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .map_err(|e| format!("Query failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(jobs)
    }

    /// 根据 ID 获取定时任务
    pub fn get_job_by_id(&self, id: u64) -> Result<Option<CronJob>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, schedule, command, description, enabled, status, last_run, next_run, created_at, updated_at 
             FROM cron_jobs WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt
            .query_row(params![id], |row| {
                Ok(CronJob {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    schedule: row.get(2)?,
                    command: row.get(3)?,
                    description: row.get(4)?,
                    enabled: row.get::<_, i32>(5)? != 0,
                    status: row.get(6)?,
                    last_run: row.get(7)?,
                    next_run: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;

        Ok(result)
    }

    /// 根据名称获取定时任务
    pub fn get_job_by_name(&self, name: &str) -> Result<Option<CronJob>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, schedule, command, description, enabled, status, last_run, next_run, created_at, updated_at 
             FROM cron_jobs WHERE name = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt
            .query_row(params![name], |row| {
                Ok(CronJob {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    schedule: row.get(2)?,
                    command: row.get(3)?,
                    description: row.get(4)?,
                    enabled: row.get::<_, i32>(5)? != 0,
                    status: row.get(6)?,
                    last_run: row.get(7)?,
                    next_run: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .optional()
            .map_err(|e| format!("Query failed: {}", e))?;

        Ok(result)
    }

    /// 创建定时任务
    pub fn create_job(&self, name: &str, schedule: &str, command: &str, description: Option<&str>, enabled: bool) -> Result<CronJob, String> {
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let status = if enabled { "active" } else { "inactive" };

        conn.execute(
            "INSERT INTO cron_jobs (name, schedule, command, description, enabled, status, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![name, schedule, command, description, enabled as i32, status, now, now],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                format!("Job name '{}' already exists", name)
            } else {
                format!("Insert failed: {}", e)
            }
        })?;

        let id = conn.last_insert_rowid() as u64;

        Ok(CronJob {
            id,
            name: name.to_string(),
            schedule: schedule.to_string(),
            command: command.to_string(),
            description: description.map(|s| s.to_string()),
            enabled,
            status: status.to_string(),
            last_run: None,
            next_run: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// 更新定时任务
    pub fn update_job(&self, id: u64, name: Option<&str>, schedule: Option<&str>, command: Option<&str>, description: Option<&str>, enabled: Option<bool>) -> Result<CronJob, String> {
        let job = self.get_job_by_id(id)?.ok_or_else(|| format!("Job {} not found", id))?;
        
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs() as i64;

        let new_name = name.unwrap_or(&job.name);
        let new_schedule = schedule.unwrap_or(&job.schedule);
        let new_command = command.unwrap_or(&job.command);
        let new_description = description.or(job.description.as_deref());
        let new_enabled = enabled.unwrap_or(job.enabled);
        let new_status = if new_enabled { "active" } else { "inactive" };

        conn.execute(
            "UPDATE cron_jobs SET name = ?1, schedule = ?2, command = ?3, description = ?4, enabled = ?5, status = ?6, updated_at = ?7 WHERE id = ?8",
            params![new_name, new_schedule, new_command, new_description, new_enabled as i32, new_status, now, id],
        ).map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                format!("Job name '{}' already exists", new_name)
            } else {
                format!("Update failed: {}", e)
            }
        })?;

        self.get_job_by_id(id)
            .map_err(|e| format!("Failed to get updated job: {}", e))?
            .ok_or_else(|| "Job not found after update".to_string())
    }

    /// 删除定时任务
    pub fn delete_job(&self, id: u64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "DELETE FROM cron_jobs WHERE id = ?1",
            params![id],
        ).map_err(|e| format!("Delete failed: {}", e))?;

        Ok(affected > 0)
    }
}

impl Default for SqliteCronJobRepository {
    fn default() -> Self {
        panic!("SqliteCronJobRepository requires database connection");
    }
}
