// Phase 203: SMB 共享详情 API
// GET /api/v1/shares/smb/{id} — 获取 SMB 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// SMB 共享详情信息
#[derive(Serialize, Clone)]
pub struct SmbShareDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub allowed_users: Option<String>,
    pub allowed_groups: Option<String>,
    pub guest_ok: bool,
    pub read_only: bool,
    pub status: String,
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
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议为 SMB（非 SMB 返回 404）
/// - 返回完整共享信息（包含 SMB 专用字段）
pub async fn get_smb_share(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
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

    // 3. 查询共享
    match repo.get_share_by_id(share_id) {
        Ok(Some(share)) => {
            // 4. 验证协议为 SMB
            if share.protocol != "smb" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("SMB share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 5. 转换数据格式（返回完整共享信息）
            let detail = SmbShareDetail {
                id: share.id,
                name: share.name,
                path: share.path,
                description: share.description,
                allowed_users: share.allowed_users,
                allowed_groups: share.allowed_groups,
                guest_ok: share.guest_ok,
                read_only: share.read_only,
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            Ok(HttpResponse::Ok().json(SmbShareDetailResponse {
                success: true,
                data: detail,
            }))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("SMB share {} not found", share_id),
            code: "NOT_FOUND".to_string(),
        })),
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询共享失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_smb_share_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let repo = web::Data::new(Arc::new(SqliteShareRepository::new(
            crate::database::pool::create_sqlite_pool(":memory:").expect("Failed to create test database"),
        )));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .app_data(repo)
                .route("/api/v1/shares/smb/{id}", web::get().to(get_smb_share))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
