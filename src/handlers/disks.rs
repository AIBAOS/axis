// 磁盘管理处理器（SQLite 持久化版）
// 包含：列表、详情、健康状态、使用量汇总

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use crate::database::disk_store::SqliteDiskRepository;
use crate::services::jwt_service::JwtService;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct DiskQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub smart_status: Option<String>,
    pub disk_type: Option<String>,
}

/// GET /api/v1/disks — 磁盘列表（分页 + smart_status/disk_type 筛选）
/// 需要登录用户访问
pub async fn list_disks(
    req: HttpRequest,
    query: web::Query<DiskQuery>,
    repo: web::Data<Arc<SqliteDiskRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    let _claims = jwt_service.validate_token(&token.expect("Token should exist"))
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).max(1).min(100);

    match repo.get_disks(
        query.smart_status.as_deref(),
        query.disk_type.as_deref(),
        page,
        per_page,
    ) {
        Ok((disks, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": disks,
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
            "message": format!("查询磁盘列表失败: {}", e)
        }))),
    }
}

/// GET /api/v1/disks/{id} — 磁盘详情
pub async fn get_disk(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteDiskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_disk_by_id(id) {
        Ok(Some(disk)) => Ok(HttpResponse::Ok().json(json!({ "success": true, "data": disk }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Disk '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询磁盘失败: {}", e)
        }))),
    }
}

/// GET /api/v1/disks/{id}/health — 磁盘健康状态
pub async fn get_disk_health(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteDiskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_disk_health(id) {
        Ok(Some((smart_status, health_score, temperature))) => {
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": {
                    "id": id,
                    "smart_status": smart_status,
                    "health_score": health_score,
                    "temperature": temperature
                }
            })))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Disk '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询磁盘健康状态失败: {}", e)
        }))),
    }
}

/// GET /api/v1/disks/usage — 磁盘使用量汇总
pub async fn get_disk_usage(
    repo: web::Data<Arc<SqliteDiskRepository>>,
) -> Result<HttpResponse> {
    match repo.get_disk_usage_summary() {
        Ok((total_bytes, used_bytes)) => {
            let free_bytes = total_bytes - used_bytes;
            let usage_percent = if total_bytes > 0 {
                (used_bytes as f64 / total_bytes as f64 * 100.0) as u32
            } else {
                0
            };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": {
                    "total_bytes": total_bytes,
                    "used_bytes": used_bytes,
                    "free_bytes": free_bytes,
                    "usage_percent": usage_percent
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询磁盘使用量失败: {}", e)
        }))),
    }
}
