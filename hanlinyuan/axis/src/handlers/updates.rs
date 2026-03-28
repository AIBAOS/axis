// 系统更新管理处理器（SQLite 持久化版）
// 包含：检查更新、更新历史

use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::database::update_store::SqliteUpdateRepository;

const CURRENT_VERSION: &str = "0.1.0";
const LATEST_VERSION: &str = "0.2.0";

#[derive(Debug, Deserialize)]
pub struct UpdateHistoryQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

/// GET /api/v1/system/updates/check — 检查系统更新
pub async fn check_updates(
    repo: web::Data<Arc<SqliteUpdateRepository>>,
) -> Result<HttpResponse> {
    let installed_version = repo.get_current_version()
        .unwrap_or(None)
        .unwrap_or_else(|| CURRENT_VERSION.to_string());

    let has_update = installed_version != LATEST_VERSION;

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "current_version": installed_version,
            "latest_version": LATEST_VERSION,
            "has_update": has_update,
            "release_notes": if has_update {
                "Axis v0.2.0: 新增磁盘健康监控、电源管理、计划任务等功能"
            } else {
                "系统已是最新版本"
            }
        }
    })))
}

/// GET /api/v1/system/updates/history — 更新历史列表（分页 + 状态筛选）
pub async fn get_update_history(
    query: web::Query<UpdateHistoryQuery>,
    repo: web::Data<Arc<SqliteUpdateRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

    match repo.get_update_history(query.status.as_deref(), page, per_page) {
        Ok((records, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": records,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": total_pages
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询更新历史失败: {}", e)
        }))),
    }
}
