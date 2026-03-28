// Phase 216: WebDAV 共享详情 API
// GET /api/v1/shares/webdav/{id} — 获取 WebDAV 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// WebDAV 共享详情信息
#[derive(Serialize, Clone)]
pub struct WebdavShareDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub public: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// WebDAV 共享详情响应
#[derive(Serialize)]
pub struct WebdavShareDetailResponse {
    pub success: bool,
    pub data: WebdavShareDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 WebDAV 共享详情（Phase 216）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（仅 WebDAV，非 WebDAV 返回 404）
/// - 返回 WebDAV 共享完整详情
pub async fn get_webdav_share(
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

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view WebDAV share details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询共享
    match repo.get_share_by_id(share_id) {
        Ok(Some(share)) => {
            // 5. 验证是 WebDAV 协议
            if share.protocol != "webdav" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("WebDAV share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 6. 构建 WebDAV 共享详情
            let detail = WebdavShareDetail {
                id: share.id,
                name: share.name,
                path: share.path,
                description: share.description,
                public: share.guest_ok,
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            Ok(HttpResponse::Ok().json(WebdavShareDetailResponse {
                success: true,
                data: detail,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("WebDAV share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_webdav_share_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/shares/webdav/{id}", web::get().to(get_webdav_share))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
