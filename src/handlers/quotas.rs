// 配额管理处理器
// 包含：获取/设置/查询用户配额

use actix_web::{web, HttpRequest, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;
use crate::services::quota_service::QuotaService;

/// 设置配额请求
#[derive(Deserialize)]
pub struct SetQuotaRequest {
    pub quota_bytes: u64,
}

/// 配额响应
#[derive(Serialize)]
pub struct QuotaResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<QuotaData>,
}

/// 配额数据
#[derive(Serialize)]
pub struct QuotaData {
    pub user_id: u64,
    pub quota_bytes: u64,
    pub used_bytes: u64,
    pub remaining_bytes: u64,
}

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

    jwt_service.validate_token(&token.expect("Token should exist"))
        .map_err(|_| HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Invalid token"
        })))
}

/// 获取用户配额
/// 需要登录用户访问
pub async fn get_quota(
    http_req: HttpRequest,
    path: web::Path<u64>,
    quota_service: web::Data<QuotaService>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let user_id = path.into_inner();
    
    match quota_service.get_quota(user_id) {
        Some(quota) => {
            Ok(HttpResponse::Ok().json(QuotaResponse {
                success: true,
                message: "配额查询成功".to_string(),
                data: Some(QuotaData {
                    user_id: quota.user_id,
                    quota_bytes: quota.quota_bytes,
                    used_bytes: quota.used_bytes,
                    remaining_bytes: quota.quota_bytes.saturating_sub(quota.used_bytes),
                }),
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(QuotaResponse {
                success: false,
                message: "未找到配额记录".to_string(),
                data: None,
            }))
        }
    }
}

/// 设置用户配额
/// 需要登录用户访问
pub async fn set_quota(
    http_req: HttpRequest,
    path: web::Path<u64>,
    req: web::Json<SetQuotaRequest>,
    quota_service: web::Data<QuotaService>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let user_id = path.into_inner();
    let quota_bytes = req.into_inner().quota_bytes;
    
    match quota_service.set_quota(user_id, quota_bytes) {
        Ok(()) => {
            let data = quota_service.get_quota(user_id).map(|q| QuotaData {
                user_id: q.user_id,
                quota_bytes: q.quota_bytes,
                used_bytes: q.used_bytes,
                remaining_bytes: q.quota_bytes.saturating_sub(q.used_bytes),
            });
            
            Ok(HttpResponse::Ok().json(QuotaResponse {
                success: true,
                message: "配额设置成功".to_string(),
                data,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(QuotaResponse {
                success: false,
                message: format!("设置失败: {}", e),
                data: None,
            }))
        }
    }
}

/// 配额列表（支持分页）
/// 需要登录用户访问
pub async fn list_quotas(
    http_req: HttpRequest,
    query: web::Query<serde_json::Value>,
    quota_service: web::Data<QuotaService>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let page = query.get("page").and_then(|v| v.as_u64()).unwrap_or(1);
    let page_size = query.get("page_size").and_then(|v| v.as_u64()).unwrap_or(10);
    
    let quotas = quota_service.list_quotas(page, page_size);
    let total = quotas.len() as u64;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "quotas": quotas,
        "total": total,
        "page": page,
        "page_size": page_size
    })))
}

/// 获取用户配额使用情况
/// 需要登录用户访问
pub async fn get_quota_usage(
    http_req: HttpRequest,
    path: web::Path<u64>,
    quota_service: web::Data<QuotaService>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // JWT 认证
    let _claims = match validate_auth(&http_req, &jwt_service) {
        Ok(c) => c,
        Err(e) => return Ok(e),
    };

    let user_id = path.into_inner();
    
 match quota_service.get_quota_usage(user_id) {
        Some(quota) => {
            Ok(HttpResponse::Ok().json(QuotaResponse {
                success: true,
                message: "配额使用情况查询成功".to_string(),
                data: Some(QuotaData {
                    user_id: quota.user_id,
                    quota_bytes: quota.quota_bytes,
                    used_bytes: quota.used_bytes,
                    remaining_bytes: quota.quota_bytes.saturating_sub(quota.used_bytes),
                }),
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(QuotaResponse {
                success: false,
                message: "未找到配额记录".to_string(),
                data: None,
            }))
        }
    }
}
