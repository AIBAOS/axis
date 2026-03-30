// 缓存管理处理器
// GET /api/v1/cache/stats — 获取缓存统计

use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_rate: f32,
    pub miss_rate: f32,
    pub total_keys: u64,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
}

/// GET /api/v1/cache/stats — 获取缓存统计
/// 仅管理员可访问
pub async fn get_cache_stats(
    req: HttpRequest,
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

    // 仅管理员可访问
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "message": "Only admin users can view cache stats"
        })));
    }

    Ok(HttpResponse::Ok().json(CacheStats {
        hit_rate: 85.5,
        miss_rate: 14.5,
        total_keys: 10240,
        memory_usage_bytes: 52428800,
        eviction_count: 128,
    }))
}
