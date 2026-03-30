// 系统电源管理处理器（SQLite 持久化版）
// 包含：执行电源操作、查询操作历史

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::power_store::SqlitePowerRepository;
use crate::services::jwt_service::JwtService;

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

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// POST /api/v1/system/power — 执行电源操作
/// 仅管理员可执行
pub async fn execute_power_action(
    req: HttpRequest,
    payload: web::Json<PowerActionRequest>,
    repo: web::Data<Arc<SqlitePowerRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let claims = jwt_service.validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // 仅管理员可执行电源操作
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only admin users can execute power actions"
        })));
    }

    let action = payload.action.trim().to_lowercase();

    // 校验操作类型
    if !["shutdown", "reboot", "sleep"].contains(&action.as_str()) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": format!("无效的电源操作: '{}'. 支持: shutdown, reboot, sleep", action)
        })));
    }

    // 记录操作日志
    match repo.log_power_action(&action, &claims.username, "executed") {
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
