// Users handlers - 管理员权限验证版本
use actix_web::{web, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuotaRequest {
    pub quota_mb: u64,
}

// 验证用户是否为管理员
fn require_admin(claims: &crate::models::jwt::JwtClaims) -> Result<(), Error> {
    if !claims.roles.contains(&"admin".to_string()) {
        return Err(Error::from(actix_web::error::ErrorForbidden("Admin access required")));
    }
    Ok(())
}

// GET /api/users/{user_id} - 需要管理员权限
pub async fn get_user(
    claims: web::Data<crate::models::jwt::JwtClaims>,
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, Error> {
    require_admin(claims.get_ref())?;
    
    let _user_id = path.into_inner().0;
    // 暂时返回占位数据（Phase 3-8 集成数据库后完善）
    let user = User {
        user_id: 1,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        quota_mb: 1024,
        used_mb: 0,
        created_at: 0,
    };
    Ok(HttpResponse::Ok().json(UserResponse { user }))
}

// GET /api/users - 列出所有用户（需管理员权限）
pub async fn list_users(claims: web::Data<crate::models::jwt::JwtClaims>) -> Result<HttpResponse, Error> {
    require_admin(claims.get_ref())?;
    
    let users = vec![User {
        user_id: 1,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        quota_mb: 1024,
        used_mb: 0,
        created_at: 0,
    }];
    Ok(HttpResponse::Ok().json(ListUsersResponse { users }))
}

// PUT /api/users/{user_id}/quota - 更新用户配额（需管理员权限）
pub async fn update_user_quota(
    claims: web::Data<crate::models::jwt::JwtClaims>,
    path: web::Path<(u64,)>,
    req: web::Json<UpdateQuotaRequest>,
) -> Result<HttpResponse, Error> {
    require_admin(claims.get_ref())?;
    
    let _user_id = path.into_inner().0;
    // 暂时返回占位数据（Phase 3-8 集成数据库后完善）
    let user = User {
        user_id: 1,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        quota_mb: req.quota_mb,
        used_mb: 0,
        created_at: 0,
    };
    Ok(HttpResponse::Ok().json(UserResponse { user }))
}

// DELETE /api/users/{user_id} - 删除用户（需管理员权限）
pub async fn delete_user(
    claims: web::Data<crate::models::jwt::JwtClaims>,
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, Error> {
    require_admin(claims.get_ref())?;
    
    let _user_id = path.into_inner().0;
    Ok(HttpResponse::Ok().json(serde_json::json!({"deleted": true})))
}
