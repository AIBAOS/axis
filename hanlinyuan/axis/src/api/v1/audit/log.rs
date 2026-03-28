use axum::{extract::State, http::StatusCode, response::Json};
use diesel::prelude::*;
use serde::Deserialize;

use crate::{AppState, models::audit_log::AuditLog};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub user_id: Option<i64>,
}

pub async fn get_audit_logs(
    State(state): State<AppState>,
    params: axum::extract::Query<QueryParams>,
) -> Result<Json<Vec<AuditLog>>, (StatusCode, String)> {
    let mut conn = state.db.get().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e)))?;
    
    let logs = if let Some(user_id) = params.user_id {
        audit_log::get_logs_by_user(&mut conn, user_id, params.page, params.page_size)
    } else {
        audit_log::get_all_logs(&mut conn, params.page, params.page_size)
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Query error: {}", e)))?;
    
    Ok(Json(logs))
}

pub async fn get_audit_log_by_id(
    State(state): State<AppState>,
    axum::extract::Path(_id): axum::extract::Path<i64>,
) -> Result<Json<AuditLog>, (StatusCode, String)> {
    // TODO: 实现单条查询逻辑
    Ok(Json(AuditLog {
        id: 0,
        user_id: 0,
        action: "".to_string(),
        resource: "".to_string(),
        details: None,
        ip_address: "".to_string(),
        created_at: chrono::Utc::now().naive_local(),
    }))
}

pub async fn create_audit_log(
    State(state): State<AppState>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<AuditLog>, (StatusCode, String)> {
    // TODO: 实现日志记录逻辑（通常由中间件自动调用）
    Ok(Json(AuditLog {
        id: 0,
        user_id: 0,
        action: "POST /api/v1/audit/logs".to_string(),
        resource: "audit_logs".to_string(),
        details: Some("内部调用 - 记录操作".to_string()),
        ip_address: "".to_string(),
        created_at: chrono::Utc::now().naive_local(),
    }))
}
