#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use std::env;
use std::sync::Mutex;

mod middleware {
    pub mod jwt_auth;
    pub mod rate_limiter;
    pub mod request_logging;
}
mod handlers;
mod routes {
    pub mod media;
}

mod services {
    pub mod file_service;
    pub mod jwt_service;
    pub mod rbac_service;
    pub mod session_service;
    pub mod quota_service;
}
mod models {
    pub mod jwt;
    pub mod user;
    pub mod role;
    pub mod rbac;
    pub mod session;
    pub mod quota;
    pub mod file_audit;
    pub mod printer;
    pub mod media;
    pub mod task;
    pub mod share;
}
mod database {
    pub mod pool;
    pub mod rbac_store;
    pub mod user_store;
    pub mod session_store;
    pub mod seed_roles;
    pub mod quota_store;
    pub mod notification_store;
    pub mod app_store;
    pub mod container_store;
    pub mod backup_store;
    pub mod scheduled_task_store;
    pub mod disk_store;
    pub mod power_store;
    pub mod update_store;
    pub mod usb_device_store;
    pub mod share_store;
}
mod config;

use handlers::rbac::{create_role, list_roles, list_permissions, assign_permission_to_role, get_user_permissions};
use handlers::downloads::{
    get_downloads, get_download, create_download, cancel_download,
};
use handlers::tasks::{
    get_tasks,
    create_task,
    get_task,
    update_task,
    delete_task,
};
use handlers::settings::{get_all_settings, get_setting, update_setting};
use handlers::notifications::{
    get_notifications,
    get_notification,
    create_notification,
    mark_as_read,
    delete_notification,
    delete_read_notifications,
};
use handlers::system_notifications_list::list_notifications;
use handlers::system_notifications_mark_read::mark_system_notification_as_read;
use handlers::system_notifications_mark_read::mark_notification_as_read;
use handlers::system_notifications_delete::delete_notification as delete_system_notification;
use handlers::system_notifications_detail::get_system_notification_detail;

use handlers::system_update::{
    check_update,
    get_update_info,
    download_update,
    install_update,
    get_update_status,
    cancel_update,
};
use handlers::system_logs::get_system_logs;
use handlers::system_logs_detail::get_system_log_detail;
use handlers::system_logs_export::export_system_logs;
use handlers::system_alerts_list::list_system_alerts;
use handlers::system_alerts_detail::get_system_alert_detail;
use handlers::system_alerts_acknowledge::acknowledge_system_alert;
use handlers::system_alerts_resolve::resolve_system_alert;
use handlers::system_alerts_delete::delete_system_alert;
use handlers::network_config_list::list_network_config;
use handlers::storage_disks_list::list_storage_disks;
use handlers::storage_disk_detail::get_storage_disk_detail;
use handlers::network_interfaces_list::list_network_interfaces;
use handlers::network_interfaces_create::create_network_interface;
use handlers::network_interface_update::update_network_interface;
use handlers::network_interface_delete::delete_network_interface;
use handlers::storage_volumes_list::list_storage_volumes;
use handlers::storage_volume_detail::get_storage_volume_detail;
use handlers::disk_smart::get_disk_smart_info;
use handlers::printers_list::list_printers;
use handlers::printers_jobs_list::list_printer_jobs as list_print_jobs;
use handlers::printers_jobs_detail::get_print_job_detail as get_job_detail;
use handlers::printers_create_job::create_print_job as create_print_job_2;
use handlers::printers_update_job::update_job;
use handlers::printers_delete_job::delete_job;
use handlers::printers_stats::get_printer_stats;
use handlers::printers_create::create_printer;
use handlers::printers_update::update_printer;
use handlers::printers_get::get_printer_detail as get_printer_2;
use handlers::printers_delete::delete_printer;
use handlers::system_health::get_system_health;
use handlers::storage_volume_snapshot_create::create_volume_snapshot as create_volume_snapshot;
use handlers::storage_volume_snapshots_list::list_volume_snapshots as list_volume_snapshots;
use handlers::storage_volume_snapshot_detail::get_volume_snapshot as get_volume_snapshot;
use handlers::storage_volume_snapshot_delete::delete_snapshot as delete_volume_snapshot;
use handlers::storage_volume_snapshot_update::update_snapshot as update_volume_snapshot;
use handlers::storage_volume_snapshot_restore::restore_volume_snapshot as restore_volume_snapshot;
use handlers::storage_volume_snapshot_clone::clone_volume_snapshot as clone_volume_snapshot;
use handlers::shared_folder_create::create_shared_folder as create_shared_folder;
use handlers::shared_folder_list::list_shared_folders as list_shared_folders;
use handlers::shared_folder_detail::get_shared_folder as get_shared_folder;
use handlers::shared_folder_update::update_shared_folder as update_shared_folder;
use handlers::shared_folder_delete::delete_shared_folder as delete_shared_folder;
use handlers::shared_folder_permissions_list::list_permissions as list_shared_folder_permissions;
use handlers::shared_folder_permissions_add::add_shared_folder_permission as add_shared_folder_permission;
use handlers::shared_folder_permissions_update::update_shared_folder_permission as update_shared_folder_permission;
use handlers::shared_folder_permissions_delete::delete_shared_folder_permission as delete_shared_folder_permission;
use handlers::firewall_rules_create::create_firewall_rule;
use handlers::firewall_rule_delete::delete_firewall_rule as delete_firewall_rule_by_id;
use handlers::firewall_rules::list_firewall_rules;
use handlers::firewall_rule_detail::get_firewall_rule_detail;
use handlers::firewall_rule_update::update_firewall_rule;
use handlers::storage_pools_list::list_pools;
use handlers::storage_pool_detail::get_pool;
use handlers::storage_pools_create::create_storage_pool;
use handlers::storage_pool_update::update_pool;
use handlers::storage_pools_delete::delete_storage_pool;
use handlers::storage_pools_volumes_list::list_pool_volumes;
use handlers::storage_usage::get_storage_usage;
use handlers::wifi::{
    scan_wifi, connect_wifi, disconnect_wifi, get_wifi_status,
    list_wifi_interfaces, forget_wifi, list_saved_wifi,
};
use handlers::apps::get_apps;
use handlers::apps::get_app;
use handlers::apps::install_app;
use handlers::apps::uninstall_app;
use handlers::containers_list::list_containers;
use handlers::containers_detail::get_container_detail;
use handlers::containers_create::create_container;
use handlers::containers_update::update_container;
use handlers::containers_delete::delete_container;
use handlers::containers_start::start_container;
use handlers::containers_stop::stop_container;
use handlers::containers_restart::restart_container;
use handlers::containers_logs::get_container_logs;
use handlers::containers_stats::get_container_stats;
use handlers::backups_list::list_backup_tasks;
use handlers::backups_create::create_backup;
use handlers::backups_detail::get_backup_task_detail;
use handlers::backups_execute::execute_backup_task;
use handlers::backups_update::update_backup;
use handlers::backups_delete::delete_backup;
use handlers::backups_restore::restore_backup;
use handlers::backups_archive::archive_backup;
use handlers::backups_execution_history::get_backup_execution_history;
use handlers::backups_stats::get_backup_stats;
use handlers::backups::run_backup;
use handlers::files_ex::{
    create_folder,
    delete_files,
};
use handlers::files_search::search_files;
use handlers::logs_ex::{
    get_logs,
    export_logs,
    delete_logs,
};
use handlers::scheduled_tasks::{
    list_scheduled_tasks,
    get_scheduled_task,
    create_scheduled_task,
    update_scheduled_task,
    delete_scheduled_task,
};
use handlers::power::{
    execute_power_action,
    get_power_logs,
};
use handlers::updates::{
    check_updates,
    get_update_history,
};
use handlers::usb_devices::{
    list_usb_devices,
    get_usb_device,
    eject_usb_device,
};
use handlers::shares_create::create_share;
use handlers::shares_list::list_shares;
use handlers::shares_detail::get_share;
use handlers::shares_update::update_share;
use handlers::shares_delete::delete_share;
use handlers::shares_smb_list_v2::list_smb_shares_v2 as list_smb_shares;
use handlers::shares_smb_create::create_smb_share;
use handlers::shares_smb_get::get_smb_share;
use handlers::shares_smb_update::update_smb_share;
use handlers::shares_smb_delete::delete_smb_share;
use handlers::shares_nfs_list::list_nfs_shares;
use handlers::shares_nfs_create::create_nfs_share;
use handlers::shares_nfs_get::get_nfs_share;
use handlers::shares_nfs_update::update_nfs_share;
use handlers::shares_nfs_delete::delete_nfs_share;
use handlers::shares_webdav_list::list_webdav_shares;
use handlers::shares_webdav_get::get_webdav_share;
use handlers::shares_webdav_create::create_webdav_share;
use handlers::shares_webdav_update::update_webdav_share;
use handlers::shares_webdav_delete::delete_webdav_share;
use handlers::shares_ftp_list::list_ftp_shares;
use handlers::shares_ftp_get::get_ftp_share;
use handlers::shares_ftp_create::create_ftp_share;
use handlers::shares_ftp_update::update_ftp_share;
use handlers::shares_ftp_delete::delete_ftp_share;
use handlers::users_list::list_users;
//     create_share,
//     get_share,
//     update_share,
//     delete_share,
//     toggle_share,
// };
use handlers::users_list::list_users;
use handlers::users_get_by_id::get_user_by_id;
use handlers::users_create::create_user;
use handlers::users_update::update_user;
use handlers::files_list::list_files;
use handlers::files_detail::get_file_detail;
use handlers::files_upload::upload_file;
use handlers::files_download::download_file;
use handlers::files_delete::delete_file;
use handlers::files_update::update_file;
use handlers::files_rename::{rename_file, move_file};
use handlers::files_copy::copy_file;
use handlers::files_browse::browse_files;
use handlers::network_config_get::get_network_config;
use handlers::network_config_update::update_network_config;
use handlers::dns_config_get::get_dns_config;
use handlers::dns_config_update::update_dns_config;
use middleware::jwt_auth::JwtAuth;
use services::rbac_service::RbacService;
use crate::config::AppSettings;

// 数据库连接池预留接口（支持 SQLite/PostgreSQL 切换）
pub trait DbConnectionTrait: Send + Sync + 'static {
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

impl DbConnectionTrait for SqliteConn {
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
impl DbConnectionTrait for PostgresConn {
    fn version(&self) -> &str {
        "PostgreSQL"
    }

    fn health_check(&self) -> bool {
        true
    }
}

// 全局连接池状态（单线程测试）
struct AppState {
    db_pool: Option<Box<dyn DbConnectionTrait>>,
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

    // 加载配置文件或使用默认配置
    let config = AppSettings::from_file("config.toml")
        .unwrap_or_else(|_| {
            log::warn!("config.toml not found, using defaults");
            AppSettings {
                jwt: config::SettingsJwtConfig {
                    secret_key: env::var("JWT_SECRET_KEY")
                        .unwrap_or_else(|_| "default_secret_key".to_string()),
                    issuer: "axis-nas".to_string(),
                    audience: "axis-nas-users".to_string(),
                    expiration_minutes: 60,
                    refresh_enabled: false,
                },
                database: config::DatabaseConfig {
                    path: "NAS.db".to_string(),
                    max_connections: 10,
                },
                server_host: "0.0.0.0".to_string(),
                server_port: 8080,
            }
        });

    log::info!("Configuration loaded");

    // 初始化 JWT 服务配置（从 config.toml 或环境变量）
    let jwt_config = models::jwt::JwtConfig {
        secret_key: config.jwt.secret_key,
        issuer: config.jwt.issuer,
        audience: config.jwt.audience,
        expiration_minutes: config.jwt.expiration_minutes,
        refresh_enabled: config.jwt.refresh_enabled,
    };
    let jwt_service = web::Data::new(services::jwt_service::JwtService::new(jwt_config));

    // 初始化数据库连接（预留 SQLite）
    let sqlite = SqliteConn::new("NAS.db");

    // 初始化 RBAC 存储
    use crate::database::rbac_store::SqliteRbacRepository;
    let db_pool = crate::database::pool::create_sqlite_pool("NAS.db")
        .expect("Failed to create DB pool");
    let db_arc = Arc::new(Mutex::new(db_pool));

    // 初始化 Session 服务
    let session_service = web::Data::new(services::session_service::SessionService::new(db_arc.clone()));

    // 初始化 RBAC 表（延迟执行，避免 panic 中断启动）
    let rbac_tables_init = SqliteRbacRepository::new(db_arc.clone());
    rbac_tables_init.init_tables().map_err(|e| {
        log::error!("Failed to init RBAC tables: {}", e);
    }).ok();

    // 初始化 User 存储
    use crate::database::user_store::SqliteUserRepository;
    let user_store = web::Data::new(Arc::new(SqliteUserRepository::new(db_arc.clone())));

    // 初始化 RBAC 服务
    let rbac_store = web::Data::new(Arc::new(SqliteRbacRepository::new(db_arc.clone())));
    let rbac_service = web::Data::new(Arc::new(RbacService::new(Arc::clone(&*rbac_store))));

    // 初始化配额服务
    let quota_service = web::Data::new(services::quota_service::QuotaService::new(db_arc.clone()));

    // 初始化通知存储
    use crate::database::notification_store::SqliteNotificationRepository;
    let notification_repo = Arc::new(SqliteNotificationRepository::new(db_arc.clone()));
    notification_repo.init_tables().map_err(|e| {
        log::error!("Failed to init notification tables: {}", e);
    }).ok();
    let notification_data = web::Data::new(notification_repo);

    // 初始化应用存储
    use crate::database::app_store::SqliteAppRepository;
    let app_repo = Arc::new(SqliteAppRepository::new(db_arc.clone()));
    app_repo.init_tables().map_err(|e| {
        log::error!("Failed to init app tables: {}", e);
    }).ok();
    let app_data = web::Data::new(app_repo);

    // 初始化容器存储
    use crate::database::container_store::SqliteContainerRepository;
    let container_repo = Arc::new(SqliteContainerRepository::new(db_arc.clone()));
    container_repo.init_tables().map_err(|e| {
        log::error!("Failed to init container tables: {}", e);
    }).ok();
    let container_data = web::Data::new(container_repo);

    // 初始化备份存储
    use crate::database::backup_store::SqliteBackupRepository;
    let backup_repo = Arc::new(SqliteBackupRepository::new(db_arc.clone()));
    backup_repo.init_tables().map_err(|e| {
        log::error!("Failed to init backup tables: {}", e);
    }).ok();
    let backup_data = web::Data::new(backup_repo);

    // 初始化计划任务存储
    use crate::database::scheduled_task_store::SqliteScheduledTaskRepository;
    let scheduled_task_repo = Arc::new(SqliteScheduledTaskRepository::new(db_arc.clone()));
    scheduled_task_repo.init_tables().map_err(|e| {
        log::error!("Failed to init scheduled_task tables: {}", e);
    }).ok();
    let scheduled_task_data = web::Data::new(scheduled_task_repo);

    // 初始化磁盘存储
    use crate::database::disk_store::SqliteDiskRepository;
    let disk_repo = Arc::new(SqliteDiskRepository::new(db_arc.clone()));
    disk_repo.init_tables().map_err(|e| {
        log::error!("Failed to init disk tables: {}", e);
    }).ok();
    let disk_data = web::Data::new(disk_repo);

    // 初始化电源操作日志存储
    use crate::database::power_store::SqlitePowerRepository;
    let power_repo = Arc::new(SqlitePowerRepository::new(db_arc.clone()));
    power_repo.init_tables().map_err(|e| {
        log::error!("Failed to init power_logs tables: {}", e);
    }).ok();
    let power_data = web::Data::new(power_repo);

    // 初始化更新记录存储
    use crate::database::update_store::SqliteUpdateRepository;
    let update_repo = Arc::new(SqliteUpdateRepository::new(db_arc.clone()));
    update_repo.init_tables().map_err(|e| {
        log::error!("Failed to init update_records tables: {}", e);
    }).ok();
    let update_data = web::Data::new(update_repo);

    // 初始化 USB 设备存储
    use crate::database::usb_device_store::SqliteUsbDeviceRepository;
    let usb_repo = Arc::new(SqliteUsbDeviceRepository::new(db_arc.clone()));
    usb_repo.init_tables().map_err(|e| {
        log::error!("Failed to init usb_devices tables: {}", e);
    }).ok();
    let usb_data = web::Data::new(usb_repo);

    // 初始化网络共享存储 - temporarily disabled
    let state = Arc::new(AppState {
        db_pool: Some(Box::new(sqlite)),
        request_count: Arc::new(AtomicU64::new(0)),
    });

    // Create share repository (commented out for now, similar to other repos)
    // let share_repo = Arc::new(SqliteShareRepository::new(db_arc.clone()));
    // share_repo.init_tables().map_err(|e| {
    //     log::error!("Failed to init shares tables: {}", e);
    // }).ok();
    // let share_data = web::Data::new(share_repo);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(jwt_service.clone())
            .app_data(session_service.clone())
            .app_data(rbac_store.clone())
            .app_data(rbac_service.clone())
            .app_data(user_store.clone())
            .app_data(quota_service.clone())
            .app_data(notification_data.clone())
            .app_data(app_data.clone())
            .app_data(container_data.clone())
            .app_data(backup_data.clone())
            .app_data(scheduled_task_data.clone())
            .app_data(disk_data.clone())
            .app_data(power_data.clone())
            .app_data(update_data.clone())
            .app_data(usb_data.clone())
            .wrap(JwtAuth)
            .route("/api/v1/health", web::get().to(health_check))
            .route("/api/v1/auth/login", web::post().to(handlers::auth_login::login))
            .route("/api/v1/auth/logout", web::post().to(handlers::auth_logout::logout))
            .route("/api/v1/auth/refresh", web::post().to(handlers::auth::refresh_token))
            // 文件 API routes (legacy)
            .route("/api/v1/files/upload/{filename}", web::post().to(handlers::files::upload_file))
            .route("/api/v1/files/download/{filename}", web::get().to(handlers::files::download_file))
            .route("/api/v1/files/delete/{filename}", web::delete().to(handlers::files::delete_file))
            .route("/api/v1/files/list", web::get().to(handlers::files::list_files))
            // 文件管理 API routes (Phase 27)
            .route("/api/v1/files", web::get().to(list_files))
            .route("/api/v1/files/download", web::get().to(download_file))
            .route("/api/v1/files/upload", web::post().to(upload_file))
            .route("/api/v1/files/folder", web::post().to(create_folder))
            .route("/api/v1/files/rename", web::put().to(rename_file))
            .route("/api/v1/files", web::delete().to(delete_files))
            .route("/api/v1/files/search", web::get().to(search_files))
            .route("/api/v1/files/browse", web::get().to(browse_files))
            .route("/api/v1/files/upload", web::post().to(upload_file))
            .route("/api/v1/files/{id}", web::get().to(get_file_detail))
            .route("/api/v1/files/{id}/download", web::get().to(download_file))
            .route("/api/v1/files/{id}", web::put().to(update_file))
            .route("/api/v1/files/{id}", web::delete().to(delete_file))
            // 会话管理 API routes
            .route("/api/v1/sessions/current", web::get().to(handlers::sessions::get_current_session))
            .route("/api/v1/sessions/list", web::get().to(handlers::sessions::list_sessions))
            .route("/api/v1/sessions/{session_id}", web::delete().to(handlers::sessions::delete_session))
            // 共享链接 API routes
            .route("/api/v1/share", web::post().to(handlers::share::create_share))
            .route("/api/v1/storage/pools", web::get().to(list_pools))
            .route("/api/v1/storage/pools", web::post().to(create_storage_pool))
            .route("/api/v1/storage/pools/{id}", web::get().to(get_pool))
            .route("/api/v1/storage/pools/{id}", web::put().to(update_pool))
            .route("/api/v1/storage/pools/{id}", web::delete().to(delete_storage_pool))
            .route("/api/v1/storage/pools/{id}/volumes", web::get().to(list_pool_volumes))
            .route("/api/v1/storage/usage", web::get().to(get_storage_usage))
            // 磁盘管理 API routes
            .route("/api/v1/disks", web::get().to(handlers::disks::list_disks))
            .route("/api/v1/disks/{id}", web::get().to(handlers::disks::get_disk))
            .route("/api/v1/disks/{id}/health", web::get().to(handlers::disks::get_disk_health))
            .route("/api/v1/disks/usage", web::get().to(handlers::disks::get_disk_usage))
            // 系统电源管理 API routes
            .route("/api/v1/system/power", web::post().to(execute_power_action))
            .route("/api/v1/system/power/logs", web::get().to(get_power_logs))
            // 系统更新管理 API routes
            .route("/api/v1/system/updates/check", web::get().to(check_updates))
            .route("/api/v1/system/updates/history", web::get().to(get_update_history))
            // USB/外部设备管理 API routes
            .route("/api/v1/usb-devices", web::get().to(list_usb_devices))
            .route("/api/v1/usb-devices/{id}", web::get().to(get_usb_device))
            .route("/api/v1/usb-devices/{id}/eject", web::post().to(eject_usb_device))
            // 网络共享管理 API routes (Phase 33) - temporarily disabled
            // Shared folders API routes (Phase 89-94)
            .route("/api/v1/shares", web::get().to(list_shares))
            .route("/api/v1/shares", web::post().to(create_share))
            .route("/api/v1/shares/{id}", web::get().to(get_share))
            .route("/api/v1/shares/{id}", web::put().to(update_share))
            .route("/api/v1/shares/{id}", web::delete().to(delete_share))
            // .route("/api/v1/shares/{id}/toggle", web::post().to(handlers::shares::toggle_share))
            // SMB 共享列表 API routes (Phase 202 - 增强版)
            .route("/api/v1/shares/smb", web::get().to(handlers::shares_smb_list_v2::list_smb_shares_v2))
            // SMB 共享创建 API routes (Phase 153)
            .route("/api/v1/shares/smb", web::post().to(create_smb_share))
            // SMB 共享详情 API routes (Phase 155)
            .route("/api/v1/shares/smb/{id}", web::get().to(get_smb_share))
            // SMB 共享更新 API routes (Phase 157)
            .route("/api/v1/shares/smb/{id}", web::put().to(update_smb_share))
            // SMB 共享删除 API routes (Phase 159)
            .route("/api/v1/shares/smb/{id}", web::delete().to(delete_smb_share))
            // NFS 共享列表 API routes (Phase 152)
            .route("/api/v1/shares/nfs", web::get().to(list_nfs_shares))
            // NFS 共享创建 API routes (Phase 154)
            .route("/api/v1/shares/nfs", web::post().to(create_nfs_share))
            // NFS 共享详情 API routes (Phase 156)
            .route("/api/v1/shares/nfs/{id}", web::get().to(get_nfs_share))
            // NFS 共享更新 API routes (Phase 158)
            .route("/api/v1/shares/nfs/{id}", web::put().to(update_nfs_share))
            // NFS 共享删除 API routes (Phase 160)
            .route("/api/v1/shares/nfs/{id}", web::delete().to(delete_nfs_share))
            // WebDAV 共享列表 API routes (Phase 215)
            .route("/api/v1/shares/webdav", web::get().to(list_webdav_shares))
            .route("/api/v1/shares/webdav/{id}", web::get().to(get_webdav_share))
            .route("/api/v1/shares/webdav", web::post().to(create_webdav_share))
            .route("/api/v1/shares/webdav/{id}", web::put().to(update_webdav_share))
            .route("/api/v1/shares/webdav/{id}", web::delete().to(delete_webdav_share))
            // FTP 共享列表 API routes (Phase 220)
            .route("/api/v1/shares/ftp", web::get().to(list_ftp_shares))
            // FTP 共享详情 API routes (Phase 221)
            .route("/api/v1/shares/ftp/{id}", web::get().to(get_ftp_share))
            // FTP 共享创建 API routes (Phase 222)
            .route("/api/v1/shares/ftp", web::post().to(create_ftp_share))
            // FTP 共享更新 API routes (Phase 223)
            .route("/api/v1/shares/ftp/{id}", web::put().to(update_ftp_share))
            // FTP 共享删除 API routes (Phase 224)
            .route("/api/v1/shares/ftp/{id}", web::delete().to(delete_ftp_share))
            // 用户列表 API routes (Phase 225)
            .route("/api/v1/users", web::get().to(list_users))
            // 文件列表 API routes (Phase 38)
            .route("/api/v1/files", web::get().to(list_files))
            // 文件详情 API routes (Phase 39)
            .route("/api/v1/files/{path}", web::get().to(get_file_detail))
            .route("/api/v1/files/download/{path:.*}", web::get().to(download_file))
            // 文件下载 API (Phase 109)
            .route("/api/v1/files/{id}/download", web::get().to(download_file))
            .route("/api/v1/files/delete/{path:.*}", web::delete().to(delete_file))
            // 文件删除 API (Phase 106)
            .route("/api/v1/files/{id}", web::delete().to(delete_file))
            .route("/api/v1/files/rename", web::put().to(rename_file))
            .route("/api/v1/files/move", web::put().to(move_file))
            .route("/api/v1/files/{id}/copy", web::post().to(copy_file))
            // 文件上传 API routes (Phase 40)
            .route("/api/v1/files/upload", web::post().to(upload_file))
            // 用户列表 API (Phase 34)
            .route("/api/v1/users", web::get().to(list_users))
            .route("/api/v1/users", web::post().to(create_user))
            .route("/api/v1/users/{id}", web::get().to(get_user_by_id))
            .route("/api/v1/storage/volumes/{id}/snapshots", web::get().to(list_volume_snapshots))
            .route("/api/v1/storage/volumes/{id}/snapshots", web::post().to(create_volume_snapshot))
            .route("/api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}", web::get().to(get_volume_snapshot))
            .route("/api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}", web::put().to(update_volume_snapshot))
            .route("/api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}", web::delete().to(delete_volume_snapshot))
            .route("/api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/restore", web::post().to(restore_volume_snapshot))
            .route("/api/v1/storage/volumes/{volume_id}/snapshots/{snapshot_id}/clone", web::post().to(clone_volume_snapshot))
            // 共享文件夹管理 API routes
            .route("/api/v1/shared-folders", web::get().to(list_shared_folders))
            .route("/api/v1/shared-folders", web::post().to(create_shared_folder))
            .route("/api/v1/shared-folders/{id}", web::get().to(get_shared_folder))
            .route("/api/v1/shared-folders/{id}", web::put().to(update_shared_folder))
            .route("/api/v1/shared-folders/{id}", web::delete().to(delete_shared_folder))
            .route("/api/v1/shared-folders/{id}/permissions", web::get().to(list_shared_folder_permissions))
            .route("/api/v1/shared-folders/{id}/permissions", web::post().to(add_shared_folder_permission))
            .route("/api/v1/shared-folders/{id}/permissions/{permission_id}", web::put().to(update_shared_folder_permission))
            .route("/api/v1/shared-folders/{id}/permissions/{permission_id}", web::delete().to(delete_shared_folder_permission))
            // 下载管理 API routes
            .route("/api/v1/downloads", web::get().to(get_downloads))
            .route("/api/v1/downloads", web::post().to(create_download))
            .route("/api/v1/downloads/{id}", web::get().to(get_download))
            .route("/api/v1/downloads/{id}", web::delete().to(cancel_download))
            // 打印机管理 API routes
            .route("/api/v1/printers", web::get().to(list_printers))
            .route("/api/v1/printers/stats", web::get().to(get_printer_stats))
            .route("/api/v1/printers", web::post().to(create_printer))
            .route("/api/v1/printers", web::post().to(create_printer))
            .route("/api/v1/printers/{id}", web::put().to(update_printer))
            .route("/api/v1/printers/{id}", web::get().to(get_printer_2))
            .route("/api/v1/printers/{id}", web::put().to(update_printer))
            .route("/api/v1/printers/{id}", web::delete().to(delete_printer))
            .route("/api/v1/printers/{id}/jobs", web::get().to(list_print_jobs))
            .route("/api/v1/printers/{id}/jobs", web::post().to(create_print_job_2))
            .route("/api/v1/printers/{id}/jobs/{job_id}", web::get().to(get_job_detail))
            .route("/api/v1/printers/{id}/jobs/{job_id}", web::put().to(update_job))
            .route("/api/v1/printers/{id}/jobs/{job_id}", web::delete().to(delete_job))
            // 打印机任务 API (Phase 112-113)
            .route("/api/v1/printers/{id}/jobs", web::get().to(list_print_jobs))
            .route("/api/v1/printers/{id}/jobs/{job_id}", web::get().to(get_job_detail))
            .route("/api/v1/printers/{id}", web::get().to(handlers::printers::get_printer_status))
            .route("/api/v1/printers/queue", web::get().to(handlers::printers::get_queue_status))
            // 防火墙管理 API routes
            .route("/api/v1/firewall/rules", web::get().to(list_firewall_rules))
            .route("/api/v1/firewall/rules", web::post().to(create_firewall_rule))
            .route("/api/v1/firewall/rules/{rule_id}", web::get().to(get_firewall_rule_detail))
            .route("/api/v1/firewall/rules/{rule_id}", web::put().to(update_firewall_rule))
            .route("/api/v1/firewall/rules/{rule_id}", web::delete().to(delete_firewall_rule_by_id))
            // RBAC API routes
            .route("/api/v1/roles", web::post().to(create_role))
            .route("/api/v1/roles", web::get().to(list_roles))
            .route("/api/v1/permissions", web::get().to(list_permissions))
            .route("/api/v1/roles/{role_id}/permissions", web::post().to(assign_permission_to_role))
            .route("/api/v1/users/{user_id}/permissions", web::get().to(get_user_permissions))
            // 配额管理 API routes
            .route("/api/v1/quotas/{user_id}", web::get().to(handlers::quotas::get_quota))
            .route("/api/v1/quotas/{user_id}", web::post().to(handlers::quotas::set_quota))
            .route("/api/v1/quotas", web::get().to(handlers::quotas::list_quotas))
            .route("/api/v1/quotas/{user_id}/usage", web::get().to(handlers::quotas::get_quota_usage))
            // 文件操作审计 API routes
            .route("/api/v1/file-audit/logs", web::get().to(handlers::file_audit::get_file_audit_logs))
            .route("/api/v1/file-audit/logs/{id}", web::get().to(handlers::file_audit::get_file_audit_log_by_id))
            .route("/api/v1/file-audit/stats", web::get().to(handlers::file_audit::get_file_audit_stats))
            .route("/api/v1/file-audit/logs", web::delete().to(handlers::file_audit::delete_file_audit_logs))
            // 打印机管理 API routes
            .route("/api/v1/printers", web::get().to(list_printers))
            .route("/api/v1/printers/{id}", web::get().to(handlers::printers::get_printer_status))
            .route("/api/v1/printers/{id}/jobs", web::post().to(handlers::printers::create_print_job))
            .route("/api/v1/printers/{id}/jobs/{job_id}", web::delete().to(handlers::printers::cancel_print_job))
            .route("/api/v1/printers/queue", web::get().to(handlers::printers::get_queue_status))
            // 网络配置 API (Phase 122)
            .route("/api/v1/network/config", web::get().to(get_network_config))
            // WiFi 管理 API routes
            .route("/api/v1/network/config", web::get().to(get_network_config))
            .route("/api/v1/network/config", web::put().to(update_network_config))
            .route("/api/v1/network/interfaces", web::get().to(list_network_interfaces))
            .route("/api/v1/network/dns", web::get().to(get_dns_config))
            .route("/api/v1/network/dns", web::put().to(update_dns_config))
            .route("/api/v1/network/wifi/scan", web::get().to(scan_wifi))
            .route("/api/v1/network/wifi/connect", web::post().to(connect_wifi))
            .route("/api/v1/network/wifi/disconnect", web::post().to(disconnect_wifi))
            .route("/api/v1/network/wifi/status", web::get().to(get_wifi_status))
            .route("/api/v1/network/wifi/interfaces", web::get().to(list_wifi_interfaces))
            .route("/api/v1/network/wifi/forget", web::post().to(forget_wifi))
            .route("/api/v1/network/wifi/saved", web::get().to(list_saved_wifi))
            // 媒体服务器 API routes
            .route("/api/v1/media/files", web::get().to(routes::media::list_media_files))
            .route("/api/v1/media/files/{id}", web::get().to(routes::media::get_media_file))
            .route("/api/v1/media/stats", web::get().to(routes::media::get_media_stats))
            // 后台任务管理 API routes
            .route("/api/v1/tasks", web::get().to(get_tasks))
            .route("/api/v1/tasks", web::post().to(create_task))
            .route("/api/v1/tasks/{id}", web::get().to(get_task))
            .route("/api/v1/tasks/{id}", web::put().to(update_task))
            .route("/api/v1/tasks/{id}", web::delete().to(delete_task))
            // 系统信息 API routes
            .route("/api/v1/system/info", web::get().to(handlers::system_info::get_system_info))
            .route("/api/v1/system/health", web::get().to(get_system_health))
            .route("/api/v1/system/logs", web::get().to(get_system_logs))
            .route("/api/v1/system/logs/{id}", web::get().to(get_system_log_detail))
            .route("/api/v1/system/logs/export", web::post().to(export_system_logs))
            .route("/api/v1/system/alerts", web::get().to(list_system_alerts))
            .route("/api/v1/system/alerts/{id}", web::get().to(get_system_alert_detail))
            .route("/api/v1/system/alerts/{id}/acknowledge", web::post().to(acknowledge_system_alert))
            .route("/api/v1/system/alerts/{id}/resolve", web::post().to(resolve_system_alert))
            .route("/api/v1/system/alerts/{id}", web::delete().to(delete_system_alert))
            // 网络配置 API routes
            .route("/api/v1/network/config", web::get().to(list_network_config))
            // 网络接口 API routes
            .route("/api/v1/network/interfaces", web::get().to(list_network_interfaces))
            .route("/api/v1/network/interfaces", web::post().to(create_network_interface))
            .route("/api/v1/network/interfaces/{id}", web::put().to(update_network_interface))
            .route("/api/v1/network/interfaces/{id}", web::delete().to(delete_network_interface))
            // 存储磁盘 API routes
            .route("/api/v1/storage/disks", web::get().to(list_storage_disks))
            .route("/api/v1/storage/disks/{id}", web::get().to(get_storage_disk_detail))
            .route("/api/v1/storage/disks/{id}/smart", web::get().to(get_disk_smart_info))
            // 存储卷 API routes
            .route("/api/v1/storage/volumes", web::get().to(list_storage_volumes))
            .route("/api/v1/storage/volumes/{id}", web::get().to(get_storage_volume_detail))
            // 缓存管理 API routes
            .route("/api/v1/cache/stats", web::get().to(handlers::cache::get_cache_stats))
            // 日志管理 API routes
            .route("/api/v1/logs", web::get().to(handlers::logs::get_logs))
            // 数据库优化 API routes
            .route("/api/v1/database/stats", web::get().to(handlers::database::get_database_stats))
            .route("/api/v1/database/vacuum", web::post().to(handlers::database::vacuum_database))
            .route("/api/v1/database/tables", web::get().to(handlers::database::get_database_tables))
            // 下载管理 API routes
            .route("/api/v1/downloads", web::get().to(handlers::downloads::get_downloads))
            .route("/api/v1/downloads", web::post().to(handlers::downloads::create_download))
            .route("/api/v1/downloads/{id}", web::get().to(handlers::downloads::get_download))
            .route("/api/v1/downloads/{id}", web::delete().to(handlers::downloads::cancel_download))
            // 系统设置 API routes
            .route("/api/v1/settings", web::get().to(get_all_settings))
            .route("/api/v1/settings/{key}", web::get().to(get_setting))
            .route("/api/v1/settings/{key}", web::put().to(update_setting))
            // 通知管理 API routes
            .route("/api/v1/notifications", web::get().to(get_notifications))
            .route("/api/v1/notifications", web::post().to(create_notification))
            .route("/api/v1/notifications/{id}", web::get().to(get_notification))
            .route("/api/v1/notifications/{id}/read", web::put().to(mark_as_read))
            .route("/api/v1/notifications/{id}", web::delete().to(delete_notification))
            .route("/api/v1/notifications/read", web::delete().to(delete_read_notifications))
            .route("/api/v1/system/notifications", web::get().to(list_notifications))
            .route("/api/v1/system/notifications/{id}/read", web::put().to(mark_system_notification_as_read))
            .route("/api/v1/system/notifications/{id}/mark-read", web::post().to(mark_notification_as_read))
            .route("/api/v1/system/notifications/{id}", web::delete().to(delete_system_notification))
            .route("/api/v1/system/notifications/{id}", web::get().to(get_system_notification_detail))
            // 应用/插件管理 API routes
            .route("/api/v1/apps", web::get().to(get_apps))
            .route("/api/v1/apps", web::post().to(install_app))
            .route("/api/v1/apps/{id}", web::get().to(get_app))
            .route("/api/v1/apps/{id}", web::delete().to(uninstall_app))
            // Docker 容器管理 API routes
            .route("/api/v1/containers", web::get().to(list_containers))
            // Container detail routes temporarily disabled
            .route("/api/v1/containers/{id}", web::get().to(get_container_detail))
            .route("/api/v1/containers", web::post().to(create_container))
            .route("/api/v1/containers/{id}", web::put().to(update_container))
            .route("/api/v1/containers/{id}", web::delete().to(delete_container))
            .route("/api/v1/containers/{id}/start", web::post().to(start_container))
            .route("/api/v1/containers/{id}/stop", web::post().to(stop_container))
            .route("/api/v1/containers/{id}/restart", web::post().to(restart_container))
            .route("/api/v1/containers/{id}/logs", web::get().to(get_container_logs))
            .route("/api/v1/containers/{id}/stats", web::get().to(get_container_stats))
            // 备份任务管理 API routes
            .route("/api/v1/backups", web::get().to(list_backup_tasks))
            .route("/api/v1/backups/stats", web::get().to(get_backup_stats))
            .route("/api/v1/backups", web::post().to(create_backup))
            .route("/api/v1/backups/{id}", web::get().to(get_backup_task_detail))
            .route("/api/v1/backups/{id}/execute", web::post().to(execute_backup_task))
            .route("/api/v1/backups/{id}", web::put().to(update_backup))
            .route("/api/v1/backups/{id}", web::delete().to(delete_backup))
            .route("/api/v1/backups/{id}/restore", web::post().to(restore_backup))
            .route("/api/v1/backups/{id}/archive", web::post().to(archive_backup))
            .route("/api/v1/backups/{id}/execution-history", web::get().to(get_backup_execution_history))
            .route("/api/v1/backups/{id}/run", web::post().to(run_backup))
            // 计划任务管理 API routes
            .route("/api/v1/scheduled-tasks", web::get().to(list_scheduled_tasks))
            .route("/api/v1/scheduled-tasks", web::post().to(create_scheduled_task))
            .route("/api/v1/scheduled-tasks/{id}", web::get().to(get_scheduled_task))
            .route("/api/v1/scheduled-tasks/{id}", web::put().to(update_scheduled_task))
            .route("/api/v1/scheduled-tasks/{id}", web::delete().to(delete_scheduled_task))
            // 系统更新/固件管理 API routes
            .route("/api/v1/system/update/check", web::get().to(check_update))
            .route("/api/v1/system/update/info", web::get().to(get_update_info))
            .route("/api/v1/system/update/download", web::post().to(download_update))
            .route("/api/v1/system/update/install", web::post().to(install_update))
            .route("/api/v1/system/update/status", web::get().to(get_update_status))
            .route("/api/v1/system/update/cancel", web::post().to(cancel_update))
            // 用户管理 API routes (Phase 34 & 55 & 101 & 102 & 103)
            .route("/api/v1/users", web::get().to(handlers::users_list::list_users))
            .route("/api/v1/users", web::post().to(create_user))
            .route("/api/v1/users/{id}", web::get().to(get_user_by_id))
            .route("/api/v1/users/{id}", web::put().to(update_user))
            .route("/api/v1/users/{id}", web::delete().to(handlers::users_delete::delete_user))
            .route("/api/v1/users/{id}/password", web::put().to(handlers::users::change_password))
            // 系统设置 API routes
            .route("/api/v1/settings/system", web::get().to(handlers::system_info::get_system_info))
            // 日志管理 API routes
            .route("/api/v1/logs", web::get().to(get_logs))
            .route("/api/v1/logs/export", web::get().to(export_logs))
            .route("/api/v1/logs", web::delete().to(delete_logs))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
