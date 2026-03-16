// Users handlers
use actix_web::{web, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: crate::models::user::User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<crate::models::user::User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuotaRequest {
    pub quota_mb: u64,
}

// GET /api/users/{user_id}
pub async fn get_user(
    path: web::Path<(u64,)>,
) -> Result<HttpResponse, Error> {
    let _user_id = path.into_inner().0;
    // 暂时返回占位数据
    let user = crate::models::user::User {
        user_id: 1,
        username: "demo".to_string(),
        email: "demo@example.com".to_string(),
        quota_mb: 1024,
        used_mb: 0,
        created_at: 0,
    };
    Ok(HttpResponse::Ok().json(UserResponse { user }))
}

// GET /api/users
pub async fn list_users() -> Result<HttpResponse, Error> {
    let users = vec![crate::models::user::User {
        user_id: 1,
        username: "demo".to_string(),
        email: "demo@example.com".to_string(),
        quota_mb: 1024,
        used_mb: 0,
        created_at: 0,
    }];
    Ok(HttpResponse::Ok().json(ListUsersResponse { users }))
}

// PUT /api/users/{user_id}/quota
pub async fn update_user_quota(
    path: web::Path<(u64,)>,
    req: web::Json<UpdateQuotaRequest>,
) -> Result<HttpResponse, Error> {
    let _user_id = path.into_inner().0;
    let _quota_mb = req.quota_mb;
    
    let user = crate::models::user::User {
        user_id: 1,
        username: "demo".to_string(),
        email: "demo@example.com".to_string(),
        quota_mb: _quota_mb,
        used_mb: 0,
        created_at: 0,
    };
    Ok(HttpResponse::Ok().json(UserResponse { user }))
}

// DELETE /api/users/{user_id}
pub async fn delete_user(path: web::Path<(u64,)>) -> Result<HttpResponse, Error> {
    let _user_id = path.into_inner().0;
    Ok(HttpResponse::Ok().json(serde_json::json!({"deleted": true})))
}
