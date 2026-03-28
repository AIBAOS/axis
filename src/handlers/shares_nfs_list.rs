// Phase 213: NFS 共享列表 API (增强版 - 数据库版本)
// GET /api/v1/shares/nfs — 获取 NFS 共享列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// NFS 共享查询参数
#[derive(Debug, Deserialize)]
pub struct NfsSharesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

/// NFS 客户端配置
#[derive(Serialize, Clone)]
pub struct NfsClientConfig {
    pub network: String,
    pub access: String,
}

/// NFS 共享信息
#[derive(Serialize, Clone)]
pub struct NfsShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub comment: String,
    pub read_only: bool,
    pub no_subtree_check: bool,
    pub sync: bool,
    pub clients: Vec<NfsClientConfig>,
    pub enabled: bool,
    pub created_at: String,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// NFS 共享列表响应
#[derive(Serialize)]
pub struct NfsShareListResponse {
    pub success: bool,
    pub data: Vec<NfsShareInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 NFS 共享列表（Phase 213 - 数据库版本）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：status(active/inactive)
/// - 返回字段：id/name/path/comment/read_only/no_subtree_check/sync/clients/enabled/created_at/updated_at
/// - 返回 NFS 共享列表 + 分页信息
pub async fn list_nfs_shares(
    req: HttpRequest,
    query: web::Query<NfsSharesQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let status_filter = query.status.clone();

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
            error: "Only admin users can list NFS shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库查询 NFS 共享列表
    match repo.get_shares(page, per_page, Some("nfs".to_string()), status_filter) {
        Ok(shares) => {
            // 5. 转换为响应格式
            let data: Vec<NfsShareInfo> = shares.into_iter().map(|s| NfsShareInfo {
                id: s.id,
                name: s.name,
                path: s.path,
                comment: s.description.unwrap_or_default(),
                read_only: false,
                no_subtree_check: true,
                sync: true,
                clients: vec![],
                enabled: s.status == "active",
                created_at: s.created_at.to_string(),
            }).collect();

            // 6. 计算分页信息
            let total = repo.count_shares(Some("nfs".to_string()), status_filter).unwrap_or(data.len() as u64);
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            return Ok(HttpResponse::Ok().json(NfsShareListResponse {
                success: true,
                data,
                pagination: PaginationInfo {
                    page,
                    per_page,
                    total,
                    total_pages: total_pages as u32,
                },
            }));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询 NFS 共享列表失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }));
        }
    }
}
        NfsShareInfo {
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
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareInfo {
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
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        NfsShareInfo {
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
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 应用状态过滤
    let filtered_shares: Vec<NfsShareInfo> = if let Some(ref status) = status_filter {
        let status_bool = status == "active";
        all_shares.into_iter().filter(|s| s.enabled == status_bool).collect()
    } else {
        all_shares
    };

    // 6. 应用分页
    let total = filtered_shares.len() as u64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_shares.len());
    
    let shares = if start < filtered_shares.len() {
        filtered_shares[start..end].to_vec()
    } else {
        vec![]
    };

    // 7. 返回 NFS 共享列表
    Ok(HttpResponse::Ok().json(NfsShareListResponse {
        success: true,
        data: shares,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
