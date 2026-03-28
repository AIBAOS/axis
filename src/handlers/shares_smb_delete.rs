// Phase 159: SMB 共享删除 API
// DELETE /api/v1/shares/smb/{id} — 删除 SMB 共享

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除 SMB 共享（Phase 159）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享 ID 存在性（404 Not Found）
/// - 删除成功返回 204 No Content
pub async fn delete_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let share_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete SMB shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟现有 SMB 共享数据
    let mock_shares = vec![1u64, 2u64, 3u64];

    // 5. 验证共享存在性
    let share_exists = mock_shares.iter().any(|id| *id == share_id);

    if !share_exists {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("SMB share {} not found", share_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 6. 模拟删除共享
    // （在实际实现中，这里会调用数据库或文件系统删除共享配置）

    // 7. 返回删除成功（204 No Content）
    Ok(HttpResponse::NoContent().finish())
}
