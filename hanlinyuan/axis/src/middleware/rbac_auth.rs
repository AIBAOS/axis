// RBAC 权限中间件（简化版）
// 功能：校验用户是否有权限访问指定资源

use actix_web::Error;
use std::sync::Arc;

use crate::models::jwt::JwtClaims;
use crate::services::rbac_service::RbacService;

/// RBAC 权限校验辅助函数
pub async fn check_permission(
    claims: &JwtClaims,
    rbac_service: &RbacService,
    resource: &str,
    action: &str,
) -> Result<bool, Error> {
    let user_id = claims.sub.parse().unwrap_or(0);
    Ok(rbac_service.check_permission(user_id, resource, action))
}

/// RBAC 权限中间件（用于路由包装）
pub struct RbacAuth {
    pub resource: String,
    pub action: String,
}

impl RbacAuth {
    pub fn new(resource: &str, action: &str) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}
