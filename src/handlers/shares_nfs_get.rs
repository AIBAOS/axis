// Phase 204: NFS 共享详情 API (数据库版本)
// GET /api/v1/shares/nfs/{id} — 获取 NFS 共享详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// NFS 共享详情信息（7 字段简化版）
#[derive(Serialize, Clone)]
pub struct NfsShareDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub public: bool,
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
/// - JWT 认证，登录用户可访问
/// - 归属验证：admin 可查看任意，普通用户暂受限
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 验证共享 ID 存在性（404 Not Found）
/// - 验证协议类型（非 NFS 返回 404）
/// - 返回完整共享信息（id/name/path/description/public/created_at/updated_at）
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

    // 4. 从数据库查询共享
    match repo.get_share_by_id(share_id as i64) {
        Ok(Some(share)) => {
            // 5. 验证是 NFS 协议
            if share.protocol != "nfs" {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("NFS share {} not found", share_id),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 6. 归属验证：非 admin 用户暂受限
            if !is_admin {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "普通用户暂不支持查看 NFS 共享详情".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 7. 返回共享详情（7 字段）
            let detail = NfsShareDetail {
                id: share.id,
                name: share.name,
                path: share.path,
                description: share.description,
                public: share.status == "active",
                created_at: share.created_at,
                updated_at: share.updated_at,
            };

            return Ok(HttpResponse::Ok().json(NfsShareDetailResponse {
                success: true,
                data: detail,
            }));
        }
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("NFS share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询共享失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }));
        }
    }
}
        NfsShareDetail {
            id: 1,
            name: "Data".to_string(),
            path: "/srv/nfs/data".to_string(),
            comment: "Data shared folder".to_string(),
            read_only: false,
            no_subtree_check: true,
            sync: true,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.1.0/24".to_string(),
                    access: "rw".to_string(),
                },
            ],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareDetail {
            id: 2,
            name: "Backup".to_string(),
            path: "/srv/nfs/backup".to_string(),
            comment: "Backup shared folder".to_string(),
            read_only: true,
            no_subtree_check: true,
            sync: true,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.1.0/24".to_string(),
                    access: "ro".to_string(),
                },
            ],
            enabled: true,
            status: "active".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareDetail {
            id: 3,
            name: "Media".to_string(),
            path: "/srv/nfs/media".to_string(),
            comment: "Media shared folder".to_string(),
            read_only: true,
            no_subtree_check: true,
            sync: false,
            clients: vec![
                NfsClientConfig {
                    network: "192.168.0.0/16".to_string(),
                    access: "ro".to_string(),
                },
            ],
            enabled: false,
            status: "inactive".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 查找共享
    let share = mock_shares.into_iter().find(|s| s.id == share_id);

    // 6. 验证共享存在性
    match share {
        Some(share) => {
            // 7. 返回共享详情
            Ok(HttpResponse::Ok().json(NfsShareDetailResponse {
                success: true,
                data: share,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("NFS share {} not found", share_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
