// Phase 261: 备份任务路由注册

use actix_web::{web, dev::ServiceConfig};

use crate::handlers::backups;

/// 注册备份相关路由
pub fn register_routes(config: &mut ServiceConfig) {
    config.service(
        web::resource("/api/v1/backups")
            .route(web::get().to(backups::list_backup_tasks))
            .route(web::post().to(backups::create_backup_task))
    );
}
