// Phase 125: DNS 配置 API
// GET /api/v1/network/dns — 获取 DNS 配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// DNS 配置信息
#[derive(Serialize, Clone)]
pub struct DnsConfig {
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dns_search_domains: Vec<String>,
    pub dns_mode: String, // auto 或 manual
}

/// DNS 配置响应
#[derive(Serialize)]
pub struct DnsConfigResponse {
    pub success: bool,
    pub data: DnsConfig,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 DNS 配置（Phase 125）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回：dns_primary/dns_secondary/dns_search_domains/dns_mode
pub async fn get_dns_config(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性并获取用户角色
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access DNS configuration".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟 DNS 配置数据（实际应从系统读取）
    let dns_config = DnsConfig {
        dns_primary: "8.8.8.8".to_string(),
        dns_secondary: "8.8.4.4".to_string(),
        dns_search_domains: vec!["local".to_string(), "lan".to_string()],
        dns_mode: "manual".to_string(), // auto 或 manual
    };

    Ok(HttpResponse::Ok().json(DnsConfigResponse {
        success: true,
        data: dns_config,
    }))
}
