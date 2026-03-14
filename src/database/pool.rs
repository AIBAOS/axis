// 数据库连接池管理
use r2d2::{Pool, PooledConnection};
use r2d2_rusqlite::rusqlite::Connection;
use crate::config::database::DatabaseConfig;

pub type DbPool = Pool<Connection>;

pub fn create_pool(config: &DatabaseConfig) -> Result<DbPool, String> {
    match config.r#type.as_str() {
        "sqlite" => {
            let path = config.path.as_ref()
                .ok_or_else(|| "SQLite path not configured".to_string())?;
            
            let manager = r2d2_rusqlite::SqliteConnectionManager::file(path);
            Pool::builder()
                .max_size(5)
                .build(manager)
                .map_err(|e| format!("Failed to create SQLite pool: {}", e))
        }
        "postgres" => {
            let url = config.postgres_url.as_ref()
                .ok_or_else(|| "PostgreSQL URL not configured".to_string())?;
            
            let manager = r2d2_postgres::PostgresConnectionManager::new(url);
            Pool::builder()
                .max_size(10)
                .build(manager)
                .map_err(|e| format!("Failed to create PostgreSQL pool: {}", e))
        }
        _ => Err("Invalid database type".to_string())
    }
}
