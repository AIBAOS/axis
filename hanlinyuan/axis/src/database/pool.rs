// 数据库连接管理（简化版，支持 SQLite/PostgreSQL 切换）
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

// 简单连接包装（无 r2d2 依赖）
pub struct DbPool {
    pub path: String,
    conn: Arc<Mutex<Connection>>,
}

impl DbPool {
    pub fn new(path: &str, conn: Connection) -> Self {
        Self {
            path: path.to_string(),
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

// 数据库类型
pub enum DbConnectionType {
    Sqlite(DbPool),
    #[cfg(feature = "postgres")]
    Postgres(PostgresConnection),
}

/// 创建 SQLite 连接池
pub fn create_sqlite_pool(path: &str) -> Result<DbConnectionType, String> {
    let conn = Connection::open(path)
        .map_err(|e| format!("Failed to open SQLite: {}", e))?;
    Ok(DbConnectionType::Sqlite(DbPool::new(path, conn)))
}

#[cfg(feature = "postgres")]
/// 创建 PostgreSQL 连接（feature flag）
pub fn create_postgres_connection(url: &str) -> Result<DbConnectionType, String> {
    Ok(DbConnectionType::Postgres(PostgresConnection::new(url)?))
}

// 初始化 RBAC 表
pub fn init_rbac_tables(conn: &DbConnectionType) -> Result<(), String> {
    match conn {
        DbConnectionType::Sqlite(c) => {
            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    username TEXT UNIQUE NOT NULL,
                    email TEXT NOT NULL,
                    password_hash TEXT NOT NULL,
                    password_salt TEXT NOT NULL,
                    roles TEXT,
                    permissions TEXT,
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

            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS roles_permissions (
                    role_id INTEGER NOT NULL,
                    permission_id INTEGER NOT NULL,
                    assigned_at INTEGER NOT NULL,
                    PRIMARY KEY (role_id, permission_id),
                    FOREIGN KEY (role_id) REFERENCES roles(id),
                    FOREIGN KEY (permission_id) REFERENCES permissions(id)
                )
            "#)?;

            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS sessions (
                    session_id TEXT PRIMARY KEY,
                    user_id INTEGER NOT NULL,
                    created_at INTEGER NOT NULL,
                    last_active INTEGER NOT NULL,
                    ip_address TEXT,
                    user_agent TEXT
                )
            "#)?;

            c.execute_batch(r#"
                CREATE TABLE IF NOT EXISTS user_quotas (
                    user_id INTEGER PRIMARY KEY,
                    quota_bytes INTEGER NOT NULL DEFAULT 0,
                    used_bytes INTEGER NOT NULL DEFAULT 0,
                    updated_at INTEGER NOT NULL
                )
            "#)?;

            // 预置系统角色
            if let Ok(conn) = Connection::open(&c.path) {
                crate::database::seed_roles::init_system_roles(&conn);
            }
            
            Ok(())
        }
        #[cfg(feature = "postgres")]
        DbConnectionType::Postgres(_) => {
            // TODO: PostgreSQL 初始化逻辑
            Ok(())
        }
    }
}
