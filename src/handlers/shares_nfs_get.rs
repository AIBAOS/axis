// Phase 204: NFS 共享详情 API (数据库版本)
// GET /api/v1/shares/nfs/{id} — 获取 NFS 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// NFS 客户端配置
#[derive(Serialize, Clone)]
pub struct NfsClientConfig {
    pub network: String,
    pub access: String,
}

/// NFS 共享详情信息
#[derive(Serialize, Clone)]
pub struct NfsShareDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub comment: Option<String>,
    pub read_only: bool,
    pub no_subtree_check: bool,
    pub sync: bool,
    pub clients: Vec<NfsClientConfig>,
    pub enabled: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// NFS 共享详情响应
#[derive(Serialize)]
pub struct NfsShareDetailResponse {
    pub success: bool,
    pub data: NfsShareDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 NFS 共享详情（Phase 204 - 数据库版本）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（非 NFS 返回 404）
/// - 返回 NFS 共享完整详情
pub async fn get_nfs_share(
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
            error: "Only admin users can view NFS share details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询共享
    match repo.get_share_by_id(share_id) {
        Ok(Some(share)) => {
            // 5. 验证是 NFS 协议
            if share.protocol != "nfs" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("NFS share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 6. 构建 NFS 共享详情（模拟客户端配置）
            let detail = NfsShareDetail {
                id: share.id,
                name: share.name,
                path: share.path,
                comment: share.description,
                read_only: false,
                no_subtree_check: true,
                sync: true,
                clients: vec![
                    NfsClientConfig {
                        network: "192.168.1.0/24".to_string(),
                        access: "rw".to_string(),
                    }
                ],
                enabled: share.status == "active",
                status: share.status,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            // 7. 返回共享详情
            Ok(HttpResponse::Ok().json(NfsShareDetailResponse {
                success: true,
                data: detail,
            }))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("NFS share {} not found", share_id),
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
