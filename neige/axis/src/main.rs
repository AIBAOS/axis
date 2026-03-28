//! Axis Project - Main Entry Point
//! Phase 226: 用户详情 API

use actix_web::{web, App, HttpServer, middleware};
use actix_web::http::header::ContentType;
use std::env;

mod handlers;
mod middleware as app_middleware;
mod database;
mod services;
mod models;

use handlers::{auth, files, files_search, sessions, rbac, users_update, users_get, network_interfaces, shares_smb_list};
use database::user_store::SqliteUserRepository;
use database::file_store::SqliteFileRepository;
use database::network_store::SqliteNetworkRepository;
use database::share_store::SqliteShareRepository;
use services::jwt_service::JwtService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 环境变量配置
    let jwt_secret = env::var("JWT_SECRET_KEY")
        .unwrap_or_else(|_| "default_secret_key_change_in_production".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // 初始化服务
    let jwt_service = web::Data::new(JwtService::new(&jwt_secret));
    let user_repo = web::Data::new(SqliteUserRepository::new().await.expect("Failed to create user repository"));
    let file_repo = web::Data::new(SqliteFileRepository::new().await.expect("Failed to create file repository"));
    let network_repo = web::Data::new(SqliteNetworkRepository::new().await.expect("Failed to create network repository"));
    let share_repo = web::Data::new(SqliteShareRepository::new().await.expect("Failed to create share repository"));
    
    log::info!("Starting Axis server on {}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(app_middleware::jwt_auth::jwt_middleware())
            .app_data(jwt_service.clone())
            .app_data(user_repo.clone())
            .app_data(file_repo.clone())
            .app_data(network_repo.clone())
            .app_data(share_repo.clone())
            // Phase 2: JWT 认证路由
            .service(
                web::scope("/api/v1/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/refresh", web::post().to(auth::refresh))
            )
            // Phase 3: 文件管理路由
            .service(
                web::scope("/api/v1/files")
                    .route("/upload", web::post().to(files::upload))
                    .route("/download/{id}", web::get().to(files::download))
                    .route("/{id}", web::delete().to(files::delete))
                    .route("/list", web::get().to(files::list))
                    .route("/rename", web::put().to(files::rename))
                    .route("/move", web::put().to(files::move_file))
                    .route("/copy", web::post().to(files::copy))
                    // Phase 46: 文件搜索
                    .route("/search", web::get().to(files_search::search_files))
            )
            // Phase 3.2: 会话管理路由
            .service(
                web::scope("/api/v1/sessions")
                    .route("/current", web::get().to(sessions::current))
                    .route("/list", web::get().to(sessions::list))
                    .route("/{id}", web::delete().to(sessions::delete))
            )
            // Phase 3.3: RBAC 路由
            .service(
                web::scope("/api/v1")
                    .route("/roles", web::get().to(rbac::list_roles))
                    .route("/roles", web::post().to(rbac::create_role))
                    .route("/roles/{id}", web::get().to(rbac::get_role))
                    .route("/roles/{id}", web::put().to(rbac::update_role))
                    .route("/roles/{id}", web::delete().to(rbac::delete_role))
                    .route("/permissions", web::get().to(rbac::list_permissions))
                    .route("/permissions", web::post().to(rbac::create_permission))
                    .route("/users/{id}/roles", web::post().to(rbac::assign_role))
            )
            // Phase 102-104, 225-226: 用户管理路由
            .service(
                web::scope("/api/v1/users")
                    .route("", web::get().to(users_get::list_users))
                    .route("", web::post().to(users_update::create_user))
                    .route("/{id}", web::get().to(users_get::get_user))
                    .route("/{id}", web::put().to(users_update::update_user))
                    .route("/{id}", web::delete().to(users_update::delete_user))
            )
            // Phase 129-132: 网络管理路由
            .service(
                web::scope("/api/v1/network")
                    .route("/interfaces", web::get().to(network_interfaces::list_interfaces))
                    .route("/interfaces", web::post().to(network_interfaces::create_interface))
                    .route("/interfaces/{id}", web::get().to(network_interfaces::get_interface))
                    .route("/interfaces/{id}", web::put().to(network_interfaces::update_interface))
                    .route("/interfaces/{id}", web::delete().to(network_interfaces::delete_interface))
            )
            // Phase 201-202: SMB 共享管理路由
            .service(
                web::scope("/api/v1/shares")
                    .route("/smb", web::get().to(shares_smb_list::list_smb_shares))
                    .route("/smb", web::post().to(shares_smb_list::create_smb_share))
            )
    })
    .bind((host.as_str(), port.parse::<u16>().unwrap_or(8080)))?
    .run()
    .await
}
