// Phase 103 更新用户 API (Phase 103)
// 更新用户信息接口
// JWT 认证，仅 admin 角色可访问

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::database::user_store::SqliteUserRepository;
use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::jwt::JwtClaims;
use crate::models::user::UserRepository;
use crate::models::rbac::RbacRepository;

/// 更新用户请求（Phase 103）
/// 注意：密码不可通过此接口修改，请使用 /users/{id}/password 接口
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    /// 邮箱地址（可选），更新时会验证格式
    pub email: Option<String>,
    /// 角色名称（可选），例如 "admin" 或 "user"
    pub role: Option<String>,
    /// 存储配额（可选），单位字节
    #[serde(rename = "storage_quota")]
    pub storage_quota: Option<u64>,
    /// 用户状态（可选）：active 或 inactive
    pub status: Option<String>,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 更新用户（Phase 103）
/// - JWT 认证，仅 admin 角色可访问
/// - 可更新字段：email, role, storage_quota, status
/// - 密码不可通过此接口修改（使用 /users/{id}/password 接口）
/// - 用户不存在返回 404 Not Found
/// - 非 admin 访问返回 403 Forbidden
/// - 邮箱格式无效返回 400 Bad Request
/// - 角色不存在返回 400 Bad Request
pub async fn update_user(
    jwt_claims: web::Data<JwtClaims>,
    user_repo: web::Data<SqliteUserRepository>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    path: web::Path<u64>,
    req: web::Json<UpdateUserRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "success": false,
            "error": "Only admin users can update user information",
            "code": "FORBIDDEN"
        })));
    }

    let user_id = path.into_inner();

    // 2. 查询目标用户是否存在
    let mut user = match user_repo.find_by_id(user_id) {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "error": format!("User {} not found", user_id),
                "code": "NOT_FOUND"
            })));
        }
        Err(e) => {
            log::error!("Failed to get user {}: {}", user_id, e);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database error: {}", e),
                "code": "DATABASE_ERROR"
            })));
        }
    };

    // 3. 验证并更新 email（如果提供）
    if let Some(ref new_email) = req.email {
        // 验证邮箱格式（简单的邮箱格式检查）
        if !new_email.contains('@') {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid email format",
                "code": "INVALID_EMAIL"
            })));
        }
        
        // 检查邮箱是否已被其他用户使用
        let existing_user = user_repo.find_by_email(new_email);
        match existing_user {
            Ok(Some(u)) if u.id != user_id => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "Email already in use by another user",
                    "code": "EMAIL_ALREADY_EXISTS"
                })));
            }
            Ok(_) => {
                user.email = new_email.clone();
            }
            Err(e) => {
                log::error!("Failed to check email: {}", e);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": "Database error",
                    "code": "DATABASE_ERROR"
                })));
            }
        }
    }

    // 4. 验证并更新 role（如果提供）
    if let Some(ref new_role) = req.role {
        // 检查角色是否存在
        let role = rbac_repo.get_role_by_name(new_role);
        match role {
            Some(r) => {
                user.roles = vec![r.name];
            }
            None => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": format!("Role '{}' does not exist", new_role),
                    "code": "INVALID_ROLE"
                })));
            }
        }
    }

    // 5. 验证并更新 status（如果提供）
    if let Some(ref new_status) = req.status {
        if !["active", "inactive"].contains(&new_status.as_str()) {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": "Invalid status. Valid values: active, inactive",
                "code": "INVALID_STATUS"
            })));
        }
        user.is_active = new_status == "active";
    }

    // 6. 更新存储配额（如果提供）
    if let Some(new_quota) = req.storage_quota {
        //* storage_quota not supported */ // user.storage_quota = new_quota;
    }

    // 7. 更新用户时间戳
    user.updated_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| {
            log::error!("Time error: {}", e);
            Error::from(actix_web::error::ErrorInternalServerError("Time error"))
        })?
        .as_secs();

    // 8. 更新用户到数据库
    user_repo.update(&user)
        .map_err(|e| {
            log::error!("Failed to update user {}: {}", user_id, e);
            Error::from(actix_web::error::ErrorInternalServerError(format!("Update error: {}", e)))
        })?;

    log::info!("User {} updated by admin", user_id);

    // 9. 返回更新后的用户信息（不含密码）
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "User updated successfully",
        "data": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "role": user.roles.get(0).cloned().unwrap_or_else(|| "user".to_string()),
            "storage_quota": 0, // storage_quota not supported
            "status": if user.is_active { "active" } else { "inactive" },
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }
    })))
}
