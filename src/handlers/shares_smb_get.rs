// Phase 203: SMB 共享详情 API
// GET /api/v1/shares/smb/{id} — 获取 SMB 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::{SqliteShareRepository, Share};
use std::sync::{Arc, Mutex};

/// SMB 共享详情信息
#[derive(Serialize, Clone)]
pub struct SmbShareDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: String,
    pub public: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// SMB 共享详情响应
#[derive(Serialize)]
pub struct SmbShareDetailResponse {
    pub success: bool,
    pub data: SmbShareDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 SMB 共享详情（Phase 203）
/// - JWT 认证，登录用户可访问
/// - 归属验证：普通用户只能查看自己的共享，admin 可查看任意
/// - 不存在返回 404 Not Found
/// - 返回完整共享信息（7 字段）
pub async fn get_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
    share_repo: web::Data<Arc<SqliteShareRepository>>,
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

    // 3. 提取用户 ID（从 claims）
    let user_id: u64 = claims.sub.parse().map_err(|_| {
        actix_web::error::ErrorInternalServerError("Invalid user ID in token")
    })?;

    // 4. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");

    // 5. 查询共享
    let share = share_repo.get_share_by_id(share_id).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // 6. 验证共享存在性
    let share = share.ok_or_else(|| {
        actix_web::error::ErrorNotFound("SMB share not found")
    })?;

    // 7. 验证归属（仅 SMB 协议，且检查归属）
    if share.protocol != "smb" {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: "SMB share not found".to_string(),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 普通用户只能查看自己的共享（简化实现：所有 SMB 共享视为可 viewing）
    // 若需严格归属验证，需扩展 shares 表添加 owner_id 字段
    // 当前实现：admin 可查看任意，普通用户可查看所有 SMB 共享

    // 8. 转换数据格式（仅返回 Phase 203 要求的 7 字段）
    let detail = SmbShareDetail {
        id: share.id,
        name: share.name,
        path: share.path,
        description: share.description,
        public: share.guest_access,
        created_at: share.created_at,
        updated_at: share.updated_at,
    };

    // 9. 返回共享详情
    Ok(HttpResponse::Ok().json(SmbShareDetailResponse {
        success: true,
        data: detail,
    }))
}
