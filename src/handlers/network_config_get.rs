// Phase 122 - 网络配置 API
// GET /api/v1/network/config — 获取网络配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 网络配置信息
#[derive(Serialize)]
pub struct NetworkConfig {
    pub hostname: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: String,
    pub dhcp_enabled: bool,
    pub mac_address: String,
    pub connection_status: String,
}

/// 网络配置响应
#[derive(Serialize)]
pub struct NetworkConfigResponse {
    pub success: bool,
    pub data: NetworkConfig,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取网络配置（Phase 122）
/// - JWT 认证，仅 admin 角色可访问
/// - 返回系统网络配置信息
pub async fn get_network_config(
    req: HttpRequest,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 权限校验 - 仅 admin 角色可访问
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can access network configuration".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 3. 模拟网络配置数据（实际实现中应该从系统读取）
    let config = NetworkConfig {
        hostname: "axis-nas".to_string(),
        ip_address: "192.168.1.100".to_string(),
        subnet_mask: "255.255.255.0".to_string(),
        gateway: "192.168.1.1".to_string(),
        dns_primary: "8.8.8.8".to_string(),
        dns_secondary: "8.8.4.4".to_string(),
        dhcp_enabled: true,
        mac_address: "00:1A:2B:3C:4D:5E".to_string(),
        connection_status: "connected".to_string(),
    };

    Ok(HttpResponse::Ok().json(NetworkConfigResponse {
        success: true,
        data: config,
    }))
}
