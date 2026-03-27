// 应用/插件管理处理器（SQLite 持久化版）
// 包含：列表、详情、安装、卸载

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::app_store::SqliteAppRepository;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct AppQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub category: Option<String>,
}

/// 安装应用请求
#[derive(Debug, Serialize, Deserialize)]
pub struct InstallAppRequest {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    pub icon_url: Option<String>,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default)]
    pub size_bytes: i64,
}

fn default_category() -> String { "other".to_string() }

/// GET /api/v1/apps — 应用列表（分页 + 筛选）
pub async fn get_apps(
    query: web::Query<AppQuery>,
    repo: web::Data<Arc<SqliteAppRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

    match repo.get_apps(
        query.status.as_deref(),
        query.category.as_deref(),
        page,
        per_page,
    ) {
        Ok((apps, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": apps,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": total_pages
                }
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("查询应用列表失败: {}", e)
            })))
        }
    }
}

/// GET /api/v1/apps/{id} — 应用详情
pub async fn get_app(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteAppRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_app_by_id(id) {
        Ok(Some(app)) => Ok(HttpResponse::Ok().json(app)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("App '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询应用失败: {}", e)
        }))),
    }
}

/// POST /api/v1/apps — 安装应用
pub async fn install_app(
    payload: web::Json<InstallAppRequest>,
    repo: web::Data<Arc<SqliteAppRepository>>,
) -> Result<HttpResponse> {
    match repo.install_app(
        &payload.name,
        &payload.version,
        &payload.description,
        payload.icon_url.as_deref(),
        &payload.category,
        payload.size_bytes,
    ) {
        Ok(app) => Ok(HttpResponse::Created().json(app)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("安装应用失败: {}", e)
        }))),
    }
}

/// DELETE /api/v1/apps/{id} — 卸载应用
pub async fn uninstall_app(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteAppRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.uninstall_app(id) {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("App '{}' uninstalled", id)
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("App '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("卸载应用失败: {}", e)
        }))),
    }
}
