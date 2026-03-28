// 系统电源管理处理器（SQLite 持久化版）
// 包含：执行电源操作、查询操作历史

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::power_store::SqlitePowerRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerActionRequest {
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct PowerLogQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub action: Option<String>,
}

/// POST /api/v1/system/power — 执行电源操作
pub async fn execute_power_action(
    payload: web::Json<PowerActionRequest>,
    repo: web::Data<Arc<SqlitePowerRepository>>,
) -> Result<HttpResponse> {
    let action = payload.action.trim().to_lowercase();

    // 校验操作类型
    if !["shutdown", "reboot", "sleep"].contains(&action.as_str()) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": format!("无效的电源操作: '{}'. 支持: shutdown, reboot, sleep", action)
        })));
    }

    // 记录操作日志
    match repo.log_power_action(&action, "admin", "executed") {
        Ok(log) => {
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "message": format!("电源操作 '{}' 已执行", action),
                "data": {
                    "id": log.id,
                    "action": log.action,
                    "status": log.status,
                    "created_at": log.created_at
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("执行电源操作失败: {}", e)
        }))),
    }
}

/// GET /api/v1/system/power/logs — 电源操作历史
pub async fn get_power_logs(
    query: web::Query<PowerLogQuery>,
    repo: web::Data<Arc<SqlitePowerRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

    match repo.get_power_logs(query.action.as_deref(), page, per_page) {
        Ok((logs, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": logs,
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
            "message": format!("查询电源操作历史失败: {}", e)
        }))),
    }
}
