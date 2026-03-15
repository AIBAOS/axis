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

// 初始化 RBAC 表
pub fn init_rbac_tables(pool: &DbPool) -> Result<(), String> {
    let conn = pool.get().map_err(|e| format!("Failed to get connection: {}", e))?;
    
    // users 表
    conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            salt TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
    "#)?;
    
    // roles 表
    conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
    "#)?;
    
    // permissions 表
    conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS permissions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT NOT NULL,
            resource TEXT NOT NULL,
            action TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
    "#)?;
    
    // user_roles 表（用户角色关联）
    conn.execute_batch(r#"
        CREATE TABLE IF NOT EXISTS user_roles (
            user_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            assigned_at INTEGER NOT NULL,
            PRIMARY KEY (user_id, role_id),
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (role_id) REFERENCES roles(id)
        )
    "#)?;
    
    Ok(())
}
