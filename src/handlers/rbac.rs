use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::rbac_service::RbacService;
use crate::services::jwt_service::JwtService;
use crate::models::jwt::JwtClaims;

#[derive(Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoleInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListRolesResponse {
    pub roles: Vec<RoleInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionInfo {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListPermissionsResponse {
    pub permissions: Vec<PermissionInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CheckPermissionRequest {
    pub user_id: u64,
    pub resource: String,
    pub action: String,
}

#[derive(Serialize, Deserialize)]
pub struct CheckPermissionResponse {
    pub allowed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserPermissionsResponse {
    pub permissions: Vec<String>,
}

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 验证 JWT token 并返回 claims
async fn validate_jwt(
    req: &HttpRequest,
    jwt_service: &web::Data<JwtService>,
) -> Result<crate::models::jwt::JwtClaims, actix_web::Error> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    jwt_service.validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))
}

/// 创建新角色 (仅 admin)
pub async fn create_role(
    req: HttpRequest,
    payload: web::Json<CreateRoleRequest>,
    rbac_service: web::Data<Arc<RbacService>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    let claims = validate_jwt(&req, &jwt_service).await?;
    
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "error": "Only admin users can create roles",
            "code": "FORBIDDEN"
        })));
    }

    match rbac_service.create_role(&payload.name, &payload.description) {
        Ok(role_id) => {
            Ok(HttpResponse::Created().json(serde_json::json!({
                "id": role_id,
                "name": payload.name,
                "description": payload.description
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            })))
        }
    }
}

/// 获取所有角色列表 (需要认证)
pub async fn list_roles(
    req: HttpRequest,
    rbac_service: web::Data<Arc<RbacService>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    validate_jwt(&req, &jwt_service).await?;

    let roles = rbac_service.list_roles();
    let role_infos: Vec<RoleInfo> = roles.into_iter().map(|r| RoleInfo {
        id: r.id,
        name: r.name,
        description: r.description,
    }).collect();
    
    Ok(HttpResponse::Ok().json(ListRolesResponse {
        roles: role_infos,
    }))
}

/// 获取权限列表 (需要认证)
pub async fn list_permissions(
    req: HttpRequest,
    rbac_service: web::Data<Arc<RbacService>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    validate_jwt(&req, &jwt_service).await?;

    let permissions = rbac_service.list_permissions();
    let permission_infos: Vec<PermissionInfo> = permissions.into_iter().map(|p| PermissionInfo {
        id: p.id,
        name: p.name,
        description: p.description,
        resource: p.resource,
        action: p.action,
    }).collect();
    
    Ok(HttpResponse::Ok().json(ListPermissionsResponse {
        permissions: permission_infos,
    }))
}

/// 给角色分配权限 (仅 admin)
pub async fn assign_permission_to_role(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<AssignPermissionRequest>,
    rbac_service: web::Data<Arc<RbacService>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    let claims = validate_jwt(&req, &jwt_service).await?;
    
    if !is_admin(&claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "error": "Only admin users can assign permissions",
            "code": "FORBIDDEN"
        })));
    }

    let role_id = path.into_inner();
    
    match rbac_service.assign_permission_to_role(role_id, payload.permission_id) {
        Ok(()) => {
            Ok(HttpResponse::Created().json(serde_json::json!({
                "message": "permission assigned successfully",
                "role_id": role_id,
                "permission_id": payload.permission_id
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            })))
        }
    }
}

/// 查询用户权限 (需要认证)
pub async fn get_user_permissions(
    req: HttpRequest,
    path: web::Path<u64>,
    rbac_service: web::Data<Arc<RbacService>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse> {
    validate_jwt(&req, &jwt_service).await?;

    let user_id = path.into_inner();
    let roles = rbac_service.get_roles_by_user(user_id);
    
    // Extract permissions from roles (simplified - in real scenario, join with permissions table)
    let permissions: Vec<String> = roles.iter()
        .flat_map(|role| &role.permissions)
        .cloned()
        .collect();
    
    Ok(HttpResponse::Ok().json(GetUserPermissionsResponse {
        permissions,
    }))
}