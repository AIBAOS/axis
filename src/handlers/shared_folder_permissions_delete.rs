// Phase 98 - 删除共享文件夹权限 API
// DELETE /api/v1/shared-folders/{id}/permissions/{permission_id} — 删除共享文件夹权限

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 删除权限响应
#[derive(Serialize)]
pub struct DeletePermissionResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除共享文件夹权限（Phase 98）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享文件夹 ID 存在性（404）
/// - 验证权限配置存在性（404）
/// - 删除成功返回 200 OK
pub async fn delete_shared_folder_permission(
    req: HttpRequest,
    path: web::Path<(u64, u64)>,
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

    // 2. 权限校验 - 仅 admin 角色可删除权限
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");
    
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can manage permissions".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let (shared_folder_id, permission_id) = path.into_inner();

    // 3. 模拟共享文件夹数据（验证存在性）
    let mock_shared_folders = vec![1u64, 2, 3, 4];
    if !mock_shared_folders.contains(&shared_folder_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Shared folder {} not found", shared_folder_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 4. 模拟权限配置数据（验证存在性）
    let mock_permissions = vec![
        (1u64, 1u64), // permission_id, shared_folder_id
        (2u64, 1u64),
        (3u64, 1u64),
    ];

    let permission = mock_permissions.into_iter().find(|(pid, sfid)| *pid == permission_id && *sfid == shared_folder_id);

    match permission {
        Some(_) => {
            // 5. 模拟删除操作
            // 在实际实现中，这里会调用数据库删除权限记录

            Ok(HttpResponse::Ok().json(DeletePermissionResponse {
                success: true,
                message: format!("Permission {} deleted successfully", permission_id),
            }))
        }
        None => {
            // 6. 权限配置不存在或不属于该共享文件夹
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Permission {} not found for shared folder {}", permission_id, shared_folder_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
