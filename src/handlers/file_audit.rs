use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use crate::models::file_audit::{FileAuditLog, FileOperation};
use crate::services::jwt_service::JwtService;
use chrono;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub user_id: Option<u64>,
    pub operation: Option<FileOperation>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

// 使用静态变量存储日志（简化实现，实际应使用数据库）
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static LOGS: Lazy<Arc<Mutex<Vec<FileAuditLog>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Vec::new()))
});

/// JWT 认证辅助函数
fn validate_auth(req: &HttpRequest, jwt_service: &web::Data<JwtService>) -> Result<crate::models::jwt::JwtClaims, HttpResponse> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return Err(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        })));
    }

    jwt_service.validate_token(&token.expect("Token should exist after check"))
        .map_err(|_| HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Invalid token"
        })))
}

/// 获取文件审计日志列表
/// 需要登录用户访问
pub async fn get_file_audit_logs(
    http_req: HttpRequest,
    query: web::Query<QueryParams>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let logs = LOGS.lock().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to acquire lock")
    })?;
    
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let offset = (page - 1) * page_size;
    
    let filtered: Vec<FileAuditLog> = logs.iter()
        .skip(offset as usize)
        .take(page_size as usize)
        .cloned()
        .collect();
    
    Ok(HttpResponse::Ok().json(filtered))
}

/// 获取单个审计日志
/// 需要登录用户访问
pub async fn get_file_audit_log_by_id(
    http_req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let logs = LOGS.lock().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to acquire lock")
    })?;
    
    let id = path.into_inner();
    let log_option = logs.iter()
        .find(|l| l.id == id)
        .cloned();
        
    match log_option {
        Some(log) => Ok(HttpResponse::Ok().json(log)),
        None => Ok(HttpResponse::NotFound().json(format!("Log {} not found", id)))
    }
}

/// 获取审计统计
/// 需要登录用户访问
pub async fn get_file_audit_stats(
    http_req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let logs = LOGS.lock().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to acquire lock")
    })?;
    
    let mut stats = serde_json::Map::new();
    
    // 按操作类型统计
    let mut by_operation: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for log in logs.iter() {
        let op = match log.operation {
            FileOperation::CREATE => "CREATE",
            FileOperation::READ => "READ",
            FileOperation::UPDATE => "UPDATE",
            FileOperation::DELETE => "DELETE",
        };
        *by_operation.entry(op.to_string()).or_insert(0) += 1;
    }
    
    stats.insert("by_operation".to_string(), serde_json::to_value(by_operation).unwrap_or(serde_json::Value::Null));
    
    Ok(HttpResponse::Ok().json(serde_json::Value::Object(stats)))
}

/// 删除审计日志
/// 需要登录用户访问
pub async fn delete_file_audit_logs(
    http_req: HttpRequest,
    query: web::Query<QueryParams>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let mut logs = LOGS.lock().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Failed to acquire lock")
    })?;
    
    let before = logs.len();
    
    if let Some(end_time) = &query.end_time {
        logs.retain(|log| log.timestamp > *end_time);
    }
    
    let cleaned = before - logs.len();
    Ok(HttpResponse::Ok().json(format!("Cleaned {} logs", cleaned)))
}

// 用于记录文件操作的辅助函数
pub fn log_file_operation(
    user_id: u64,
    operation: FileOperation,
    file_path: &str,
    ip_address: &str,
    details: Option<&str>,
) {
    let mut logs = LOGS.lock().expect("LOGS lock poisoned");
    let id = logs.len() as u64 + 1;
    logs.push(FileAuditLog {
        id,
        user_id,
        operation,
        file_path: file_path.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        ip_address: ip_address.to_string(),
        details: details.map(|s| s.to_string()),
    });
}
