use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::rbac_service::RbacService;

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

/// 创建新角色
pub async fn create_role(
    req: web::Json<CreateRoleRequest>,
    rbac_service: web::Data<Arc<RbacService>>,
) -> Result<HttpResponse> {
    match rbac_service.create_role(&req.name, &req.description) {
        Ok(role_id) => {
            Ok(HttpResponse::Created().json(serde_json::json!({
                "id": role_id,
                "name": req.name,
                "description": req.description
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            })))
        }
    }
}

/// 获取所有角色列表
pub async fn list_roles(
    rbac_service: web::Data<Arc<RbacService>>,
) -> Result<HttpResponse> {
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

/// 获取权限列表
pub async fn list_permissions(
    rbac_service: web::Data<Arc<RbacService>>,
) -> Result<HttpResponse> {
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

/// 给角色分配权限
pub async fn assign_permission_to_role(
    path: web::Path<u64>,
    req: web::Json<AssignPermissionRequest>,
    rbac_service: web::Data<Arc<RbacService>>,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();
    
    match rbac_service.assign_permission_to_role(role_id, req.permission_id) {
        Ok(()) => {
            Ok(HttpResponse::Created().json(serde_json::json!({
                "message": "permission assigned successfully",
                "role_id": role_id,
                "permission_id": req.permission_id
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e
            })))
        }
    }
}

/// 查询用户权限
pub async fn get_user_permissions(
    path: web::Path<u64>,
    rbac_service: web::Data<Arc<RbacService>>,
) -> Result<HttpResponse> {
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