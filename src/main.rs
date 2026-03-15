use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

mod middleware {
    pub mod jwt_auth;
}
mod handlers {
    pub mod auth;
    pub mod files;
    pub mod rbac;
}
mod services {
    pub mod file_service;
    pub mod jwt_service;
    pub mod rbac_service;
    pub mod session_service;
}
mod models {
    pub mod jwt;
    pub mod rbac;
    pub mod session;
}
mod database {
    pub mod pool;
    pub mod rbac_store;
}

// 数据库连接池预留接口（支持 SQLite/PostgreSQL 切换）
pub trait DbConnection: Send + Sync + 'static {
    fn version(&self) -> &str;
    fn health_check(&self) -> bool;
}

// SQLite 连接池实现（使用 r2d2）
pub struct SqliteConn {
    path: String,
}

impl SqliteConn {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}

impl DbConnection for SqliteConn {
    fn version(&self) -> &str {
        "SQLite 3"
    }

    fn health_check(&self) -> bool {
        true
    }
}

// PostgreSQL 连接预留（Feature Flag 控制）
#[cfg(feature = "postgres")]
pub struct PostgresConn {
    // 实际连接配置
}

#[cfg(feature = "postgres")]
impl DbConnection for PostgresConn {
    fn version(&self) -> &str {
        "PostgreSQL"
    }

    fn health_check(&self) -> bool {
        true
    }
}

// 全局连接池状态（单线程测试）
struct AppState {
    db_pool: Option<Box<dyn DbConnection>>,
    request_count: Arc<AtomicU64>,
}

impl AppState {
    fn incr_request(&self) -> u64 {
        self.request_count.fetch_add(1, Ordering::SeqCst) + 1
    }
}

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: u64,
    db_version: String,
    request_count: u64,
}

async fn health_check(state: web::Data<Arc<AppState>>) -> Result<HttpResponse> {
    let db_version = state
        .db_pool
        .as_ref()
        .map(|db| db.version().to_string())
        .unwrap_or_else(|| "uninitialized".to_string());

    let count = state.incr_request();

    Ok(HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
        timestamp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        db_version,
        request_count: count,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting Axis NAS API Server v0.1.0");

    // 初始化 JWT 服务配置
    let jwt_config = models::jwt::JwtConfig {
        secret_key: std::env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "default_secret_key".to_string()),
        issuer: "axis-nas".to_string(),
        audience: "axis-nas-users".to_string(),
        expiration_minutes: 60,
        refresh_enabled: false, // Phase 2.1: 未启用刷新
    };
    let jwt_service = web::Data::new(services::jwt_service::JwtService::new(jwt_config));

    // 初始化数据库连接（预留 SQLite）
    let sqlite = SqliteConn::new("NAS.db");
    let state = Arc::new(AppState {
        db_pool: Some(Box::new(sqlite)),
        request_count: Arc::new(AtomicU64::new(0)),
    });

    // 初始化 Session 服务
    let session_store = web::Data::new(services::session_service::SessionService::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(jwt_service.clone())
            .app_data(session_store.clone())
            .route("/api/v1/health", web::get().to(health_check))
            .route("/api/v1/auth/login", web::post().to(handlers::auth::login))
            .route("/api/v1/auth/logout", web::post().to(handlers::auth::logout))
            .route("/api/v1/auth/refresh", web::post().to(handlers::auth::refresh_token))
            // 文件 API routes
            .route("/api/v1/files/upload/{filename}", web::post().to(handlers::files::upload_file))
            .route("/api/v1/files/download/{filename}", web::get().to(handlers::files::download_file))
            .route("/api/v1/files/delete/{filename}", web::delete().to(handlers::files::delete_file))
            .route("/api/v1/files/list", web::get().to(handlers::files::list_files))
            // 会话管理 API routes
            .route("/api/v1/sessions/current", web::get().to(handlers::sessions::get_current_session))
            .route("/api/v1/sessions/list", web::get().to(handlers::sessions::list_sessions))
            .route("/api/v1/sessions/{session_id}", web::delete().to(handlers::sessions::delete_session))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
