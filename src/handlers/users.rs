// 用户管理处理器（Phase 47+）
// 包含：用户列表、详情、创建、更新、删除接口
// Phase 47: 实现创建用户接口，支持 bcrypt 密码加密、角色校验、admin 权限校验

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::database::user_store::SqliteUserRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::jwt::JwtClaims;
use crate::models::user::UserRepository;
use crate::models::rbac::RbacRepository;

/// 用户状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Locked,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Locked => write!(f, "locked"),
        }
    }
}

/// 用户角色
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
            UserRole::Guest => write!(f, "guest"),
        }
    }
}

/// 用户信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub last_login_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub storage_quota: Option<u64>,
}

/// 分页参数
#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
    pub role: Option<String>,
}

impl Default for UserQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            status: None,
            role: None,
        }
    }
}

/// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub success: bool,
    pub data: Vec<User>,
    pub pagination: UserPagination,
}

#[derive(Debug, Serialize)]
pub struct UserPagination {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 获取用户列表
pub async fn get_users(
    query: web::Query<UserQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1); // Bug #72 修复：防止整数下溢
    let limit = query.limit.unwrap_or(20).max(1) as u64; // Bug #72 修复：防止空结果
    let status_filter = query.status.as_deref();
    let role_filter = query.role.as_deref();

    // 模拟数据
    let mut all_users = vec![
        User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@axis.local".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
            last_login_at: Some("2026-03-19T01:00:00Z".to_string()),
            created_at: "2026-03-15T10:00:00Z".to_string(),
            updated_at: "2026-03-18T15:30:00Z".to_string(),
            storage_quota: Some(100_000_000_000u64),
        },
        User {
            id: 2,
            username: "user1".to_string(),
            email: "user1@axis.local".to_string(),
            role: "user".to_string(),
            status: "active".to_string(),
            last_login_at: Some("2026-03-18T20:15:00Z".to_string()),
            created_at: "2026-03-16T14:30:00Z".to_string(),
            updated_at: "2026-03-18T16:00:00Z".to_string(),
            storage_quota: Some(50_000_000_000u64),
        },
        User {
            id: 3,
            username: "guest".to_string(),
            email: "guest@axis.local".to_string(),
            role: "guest".to_string(),
            status: "inactive".to_string(),
            last_login_at: Some("2026-03-10T09:00:00Z".to_string()),
            created_at: "2026-03-17T09:15:00Z".to_string(),
            updated_at: "2026-03-17T09:15:00Z".to_string(),
            storage_quota: None,
        },
        User {
            id: 4,
            username: "user2".to_string(),
            email: "user2@axis.local".to_string(),
            role: "user".to_string(),
            status: "locked".to_string(),
            last_login_at: Some("2026-03-17T11:00:00Z".to_string()),
            created_at: "2026-03-17T10:00:00Z".to_string(),
            updated_at: "2026-03-18T17:00:00Z".to_string(),
            storage_quota: Some(75_000_000_000u64),
        },
        User {
            id: 5,
            username: "developer".to_string(),
            email: "dev@axis.local".to_string(),
            role: "user".to_string(),
            status: "active".to_string(),
            last_login_at: Some("2026-03-19T00:30:00Z".to_string()),
            created_at: "2026-03-18T08:00:00Z".to_string(),
            updated_at: "2026-03-18T08:00:00Z".to_string(),
            storage_quota: Some(50_000_000_000u64),
        },
    ];

    // 状态过滤
    if let Some(status) = status_filter {
        all_users.retain(|u| u.status == status);
    }

    // 角色过滤
    if let Some(role) = role_filter {
        all_users.retain(|u| u.role == role);
    }

    let total = all_users.len() as u64;
    let start = (page - 1) * limit;
    let end = start + limit;

    let paginated_users: Vec<User> = all_users
        .into_iter()
        .enumerate()
        .filter_map(|(i, u)| {
            let idx = i as u64;
            if idx >= start && idx < end {
                Some(u)
            } else {
                None
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(UserListResponse {
        success: true,
        data: paginated_users,
        pagination: UserPagination {
            page,
            limit,
            total,
            total_pages: (total + limit - 1) / limit,
        },
    }))
}

/// 获取用户详情
pub async fn get_user(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    let mock_users = vec![
        User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@axis.local".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
            last_login_at: Some("2026-03-19T01:00:00Z".to_string()),
            created_at: "2026-03-15T10:00:00Z".to_string(),
            updated_at: "2026-03-18T15:30:00Z".to_string(),
            storage_quota: Some(100_000_000_000u64),
        },
        User {
            id: 2,
            username: "user1".to_string(),
            email: "user1@axis.local".to_string(),
            role: "user".to_string(),
            status: "active".to_string(),
            last_login_at: Some("2026-03-18T20:15:00Z".to_string()),
            created_at: "2026-03-16T14:30:00Z".to_string(),
            updated_at: "2026-03-18T16:00:00Z".to_string(),
            storage_quota: Some(50_000_000_000u64),
        },
    ];

    match mock_users.iter().find(|u| u.id == id) {
        Some(user) => Ok(HttpResponse::Ok().json(user.clone())),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("User {} not found", id)
        }))),
    }
}

/// 创建用户请求（Phase 47）
#[derive(Debug, Deserialize)]
pub struct CreateUserRequestV2 {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role_id: u64,
    pub storage_quota: Option<u64>,
}

/// 创建用户响应
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<UserInfo>,
}

/// 用户信息（不含密码）
#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role_id: u64,
    pub role_name: String,
    pub storage_quota: u64,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 创建用户（Phase 47 完整实现）
/// - JWT 认证，仅 admin 角色可创建用户
/// - 用户名唯一性校验（重复返回 409 Conflict）
/// - 角色 ID 校验（不存在返回 400 Bad Request）
/// - 密码 bcrypt 加密存储
pub async fn create_user(
    jwt_claims: web::Data<JwtClaims>,
    user_store: web::Data<Arc<SqliteUserRepository>>,
    rbac_store: web::Data<Arc<SqliteRbacRepository>>,
    payload: web::Json<CreateUserRequestV2>,
) -> Result<HttpResponse> {
    // 1. JWT 认证：检查当前用户是否为 admin
    let current_user_id = jwt_claims.sub.parse::<u64>()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid user ID"))?;
    
    // 获取当前用户信息，检查是否为 admin
    let current_user = user_store.get_ref().find_by_id(current_user_id)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("DB error: {}", e)))?;
    
    match current_user {
        Some(user) => {
            // 检查是否有 admin 角色
            let is_admin = user.roles.iter().any(|r| r.to_lowercase() == "admin" || r == "1");
            if !is_admin {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "Only admin users can create new users".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }
        }
        None => {
            return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
                success: false,
                error: "Current user not found".to_string(),
                code: "UNAUTHORIZED".to_string(),
            }));
        }
    }

    // 2. 验证请求参数
    if payload.username.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "username is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if payload.username.len() < 3 || payload.username.len() > 50 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "username must be between 3 and 50 characters".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if payload.password.len() < 6 {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "password must be at least 6 characters".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if payload.email.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "email is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 验证邮箱格式
    if !payload.email.contains('@') || !payload.email.contains('.') {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid email format".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 用户名唯一性校验
    let existing_user = user_store.get_ref().find_by_username(&payload.username)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("DB error: {}", e)))?;
    
    if existing_user.is_some() {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Username '{}' already exists", payload.username),
            code: "CONFLICT".to_string(),
        }));
    }

    // 4. 角色 ID 校验
    let role = rbac_store.get_ref().get_role(payload.role_id);
    if role.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Role ID {} does not exist", payload.role_id),
            code: "INVALID_ROLE".to_string(),
        }));
    }
    let role = role.expect("Role should exist after check");

    // 5. bcrypt 密码加密
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Hash error: {}", e)))?;

    // 生成 salt（bcrypt 已内置 salt，这里留空或存储 hash 中的 salt 部分）
    let password_salt = "".to_string();

    // 6. 创建用户记录
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut new_user = crate::models::user::User {
        id: 0, // 将由数据库设置
        username: payload.username.clone(),
        email: payload.email.clone(),
        password_hash,
        password_salt,
        roles: vec![role.name.clone()],
        permissions: vec![],
        created_at: now,
        updated_at: now,
        last_login: None,
        is_active: true,
        storage_quota: Some(payload.storage_quota.unwrap_or(100_000_000_000u64)),
    };

    user_store.get_ref().create(&mut new_user)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Create user error: {}", e)))?;

    // 7. 如果指定了存储配额，创建配额记录
    if let Some(_quota) = payload.storage_quota {
        // 配额服务调用（预留）
        // quota_service.set_quota(new_user.id, quota)?;
    }

    // 8. 返回用户信息（不含密码）
    let user_info = UserInfo {
        id: new_user.id,
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        role_id: payload.role_id,
        role_name: role.name.clone(),
        storage_quota: payload.storage_quota.unwrap_or(0),
        status: "active".to_string(),
        created_at: new_user.created_at,
        updated_at: new_user.updated_at,
    };

    Ok(HttpResponse::Created().json(CreateUserResponse {
        success: true,
        message: "User created successfully".to_string(),
        data: Some(user_info),
    }))
}

/// 更新用户
pub async fn update_user(
    path: web::Path<u64>,
    payload: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证用户存在
    let mock_users = vec![1, 2, 3, 4, 5];
    if !mock_users.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("User {} not found", id)
        })));
    }

    // 简化模拟：返回更新后的用户
    Ok(HttpResponse::Ok().json(User {
        id,
        username: payload.username.clone().unwrap_or_else(|| "user".to_string()),
        email: payload.email.clone().unwrap_or_else(|| "unknown@axis.local".to_string()),
        role: payload.role.as_deref().unwrap_or("user").to_string(),
        status: payload.status.as_deref().unwrap_or("active").to_string(),
        last_login_at: None,
        created_at: "2026-03-15T10:00:00Z".to_string(),
        updated_at: chrono::Utc::now().to_rfc3339(),
        storage_quota: payload.storage_quota,
    }))
}

/// 删除用户
pub async fn delete_user(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证用户存在
    let mock_users = vec![1, 2, 3, 4, 5];
    if !mock_users.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("User {} not found", id)
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("User {} deleted", id)
    })))
}

/// 更改用户密码
pub async fn change_password(
    path: web::Path<u64>,
    payload: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 验证必要参数
    if payload.old_password.is_empty() || payload.new_password.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "old_password and new_password are required"
        })));
    }

    // 简化模拟
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Password updated for user {}", id)
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub storage_quota: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
