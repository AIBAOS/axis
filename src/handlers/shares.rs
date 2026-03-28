// 网络共享管理 API 处理器
// 包含：共享列表、详情、创建、更新、删除接口

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::database::share_store::SqliteShareRepository;
use crate::models::share::{Share, CreateShareRequest, UpdateShareRequest};

/// 分页查询参数
#[derive(Deserialize)]
pub struct PaginatedQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub protocol: Option<String>,
    pub status: Option<String>,
}

/// 共享列表响应
#[derive(Serialize)]
pub struct ShareListResponse {
    pub success: bool,
    pub shares: Vec<Share>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

/// 共享详情响应
#[derive(Serialize)]
pub struct ShareDetailResponse {
    pub success: bool,
    pub data: Option<Share>,
}

/// 共享操作响应
#[derive(Serialize)]
pub struct ShareOperationResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Share>,
}

/// 获取共享列表
pub async fn list_shares(
    share_repo: web::Data<SqliteShareRepository>,
    query: web::Query<PaginatedQuery>,
) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10).min(100); // 上限 100
    let protocol = query.protocol.clone();
    let status = query.status.clone();

    match share_repo.get_shares(page, per_page, protocol, status) {
        Ok(shares) => {
            let total = shares.len() as u64; // 简化版，实际应查询总数
            let shares: Vec<crate::models::share::Share> = shares.into_iter().map(|s| {
                crate::models::share::Share {
                    id: s.id,
                    name: s.name,
                    path: s.path,
                    protocol: s.protocol,
                    status: s.status,
                    description: s.description,
                    created_at: s.created_at,
                    updated_at: s.updated_at,
                }
            }).collect();
            let response = ShareListResponse {
                success: true,
                shares,
                total,
                page: page as u64,
                page_size: per_page as u64,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to list shares: {}", e);
            HttpResponse::InternalServerError().json(ShareListResponse {
                success: false,
                shares: Vec::new(),
                total: 0,
                page: 1,
                page_size: per_page as u64,
            })
        }
    }
}

/// 获取共享详情
pub async fn get_share(
    share_repo: web::Data<SqliteShareRepository>,
    path: web::Path<u64>,
) -> impl Responder {
    let share_id = path.into_inner();

    match share_repo.get_share_by_id(share_id) {
        Ok(Some(share)) => {
            let share = crate::models::share::Share {
                id: share.id,
                name: share.name,
                path: share.path,
                protocol: share.protocol,
                status: share.status,
                description: share.description,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };
            let response = ShareDetailResponse {
                success: true,
                data: Some(share),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            HttpResponse::NotFound().json(ShareDetailResponse {
                success: false,
                data: None,
            })
        }
        Err(e) => {
            log::error!("Failed to get share {}: {}", share_id, e);
            HttpResponse::InternalServerError().json(ShareDetailResponse {
                success: false,
                data: None,
            })
        }
    }
}

/// 创建共享
pub async fn create_share(
    share_repo: web::Data<SqliteShareRepository>,
    req: web::Json<CreateShareRequest>,
) -> impl Responder {
    let create_req = req.into_inner();
    let path = create_req.path.clone();

    // 检查路径存在性（简化版，实际可异步检查）
    if !std::path::Path::new(&path).exists() {
        log::warn!("Path {} does not exist, creating anyway", path);
    }

    match share_repo.create_share(&create_req.name, &path, &create_req.protocol, create_req.description.as_deref(), None, None, false, false) {
        Ok(share) => {
            let share = crate::models::share::Share {
                id: share.id,
                name: share.name,
                path: share.path,
                protocol: share.protocol,
                status: share.status,
                description: share.description,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };
            let response = ShareOperationResponse {
                success: true,
                message: "Share created successfully".to_string(),
                data: Some(share),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Failed to create share: {}", e);
            HttpResponse::InternalServerError().json(ShareOperationResponse {
                success: false,
                message: format!("Failed to create share: {}", e),
                data: None,
            })
        }
    }
}

/// 更新共享
pub async fn update_share(
    share_repo: web::Data<SqliteShareRepository>,
    path: web::Path<u64>,
    req: web::Json<UpdateShareRequest>,
) -> impl Responder {
    let share_id = path.into_inner();
    let update_req = req.into_inner();

    match share_repo.update_share(
        share_id,
        update_req.name,
        update_req.path,
        update_req.protocol,
        update_req.status,
    ) {
        Ok(share) => {
            let share = crate::models::share::Share {
                id: share.id,
                name: share.name,
                path: share.path,
                protocol: share.protocol,
                status: share.status,
                description: share.description,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };
            let response = ShareOperationResponse {
                success: true,
                message: "Share updated successfully".to_string(),
                data: Some(share),
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            if e.starts_with("Share") {
                HttpResponse::NotFound().json(ShareOperationResponse {
                    success: false,
                    message: e,
                    data: None,
                })
            } else {
                log::error!("Failed to update share {}: {}", share_id, e);
                HttpResponse::InternalServerError().json(ShareOperationResponse {
                    success: false,
                    message: format!("Failed to update share: {}", e),
                    data: None,
                })
            }
        }
    }
}

/// 删除共享
pub async fn delete_share(
    share_repo: web::Data<SqliteShareRepository>,
    path: web::Path<u64>,
) -> impl Responder {
    let share_id = path.into_inner();

    match share_repo.delete_share(share_id) {
        Ok(_) => {
            HttpResponse::Ok().json(ShareOperationResponse {
                success: true,
                message: "Share deleted successfully".to_string(),
                data: None,
            })
        }
        Err(e) => {
            if e == "Share is active, cannot delete" {
                HttpResponse::Conflict().json(ShareOperationResponse {
                    success: false,
                    message: e,
                    data: None,
                })
            } else if e.starts_with("Share") {
                HttpResponse::NotFound().json(ShareOperationResponse {
                    success: false,
                    message: e,
                    data: None,
                })
            } else {
                log::error!("Failed to delete share {}: {}", share_id, e);
                HttpResponse::InternalServerError().json(ShareOperationResponse {
                    success: false,
                    message: format!("Failed to delete share: {}", e),
                    data: None,
                })
            }
        }
    }
}

/// 切换共享启用/禁用状态
pub async fn toggle_share(
    share_repo: web::Data<SqliteShareRepository>,
    path: web::Path<u64>,
) -> impl Responder {
    let share_id = path.into_inner();

    match share_repo.toggle_share(share_id) {
        Ok(share) => {
            let share = crate::models::share::Share {
                id: share.id,
                name: share.name,
                path: share.path,
                protocol: share.protocol,
                status: share.status,
                description: share.description,
                created_at: share.created_at,
                updated_at: share.updated_at,
            };
            HttpResponse::Ok().json(ShareOperationResponse {
                success: true,
                message: format!("Share {}d", if share.status == "active" { "enabled" } else { "disabled" }),
                data: Some(share),
            })
        }
        Err(e) => {
            if e.starts_with("Share") {
                HttpResponse::NotFound().json(ShareOperationResponse {
                    success: false,
                    message: e,
                    data: None,
                })
            } else {
                log::error!("Failed to toggle share {}: {}", share_id, e);
                HttpResponse::InternalServerError().json(ShareOperationResponse {
                    success: false,
                    message: format!("Failed to toggle share: {}", e),
                    data: None,
                })
            }
        }
    }
}
