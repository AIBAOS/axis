use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

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

    // 初始化数据库连接（预留 SQLite）
    let sqlite = SqliteConn::new("NAS.db");
    let state = Arc::new(AppState {
        db_pool: Some(Box::new(sqlite)),
        request_count: Arc::new(AtomicU64::new(0)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/api/v1/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
