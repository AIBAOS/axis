// 数据库连接管理（简化版，支持 SQLite/PostgreSQL 切换）
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

// 简单连接包装（无 r2d2 依赖）
pub struct DbPool {
    conn: Arc<Mutex<Connection>>,
}

impl DbPool {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn get(&self) -> Result<ConnectionGuard<'_>, String> {
        self.conn.lock()
            .map_err(|e| format!("Lock failed: {}", e))
            .map(|g| ConnectionGuard { inner: g })
    }

    pub fn execute_batch(&self, sql: &str) -> Result<(), String> {
        self.conn.lock()
            .map_err(|e| format!("Lock failed: {}", e))?
            .execute_batch(sql)
            .map_err(|e| format!("Execute failed: {}", e))
    }
}

pub struct ConnectionGuard<'a> {
    inner: std::sync::MutexGuard<'a, Connection>,
}

impl<'a> std::ops::Deref for ConnectionGuard<'a> {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::ops::DerefMut for ConnectionGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// PostgreSQL 连接预留（Feature Flag 控制）
#[cfg(feature = "postgres")]
pub struct PostgresConnection {
    // 实际连接配置
}

#[cfg(feature = "postgres")]
impl PostgresConnection {
    pub fn new(_url: &str) -> Result<Self, String> {
        Ok(Self {})
    }
}

/// 数据库类型
pub enum DbConnection {
    Sqlite(DbPool),
    #[cfg(feature = "postgres")]
    Postgres(PostgresConnection),
}

/// 创建 SQLite 连接池
pub fn create_sqlite_pool(path: &str) -> Result<DbConnection, String> {
    let conn = Connection::open(path)
        .map_err(|e| format!("Failed to open SQLite: {}", e))?;
    Ok(DbConnection::Sqlite(DbPool::new(conn)))
}

#[cfg(feature = "postgres")]
/// 创建 PostgreSQL 连接（feature flag）
pub fn create_postgres_connection(url: &str) -> Result<DbConnection, String> {
    Ok(DbConnection::Postgres(PostgresConnection::new(url)?))
}

// 初始化 RBAC 表
pub fn init_rbac_tables(conn: &DbConnection) -> Result<(), String> {
    match conn {
        DbConnection::Sqlite(c) => {
            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT UNIQUE NOT NULL,
                    password_hash TEXT NOT NULL,
                    salt TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )
            "#)?;
            
            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS roles (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL,
                    description TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )
            "#)?;
            
            c.execute_batch(r#"
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
            
            c.execute_batch(r#"
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
        #[cfg(feature = "postgres")]
        DbConnection::Postgres(_) => {
            // TODO: PostgreSQL 初始化逻辑
            Ok(())
        }
    }
}
