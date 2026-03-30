// Phase 227: 容器列表 API (数据库增强版)
// GET /api/v1/containers — 获取容器列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::container_store::SqliteContainerRepository;

/// 容器列表查询参数
#[derive(Debug, Deserialize)]
pub struct ContainersListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 容器信息
#[derive(Serialize, Clone)]
pub struct ContainerInfo {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub status: String,
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

/// 容器列表响应
#[derive(Serialize)]
pub struct ContainerListResponse {
    pub success: bool,
    pub data: Vec<ContainerInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取容器列表（Phase 227 - 数据库增强版）
/// - JWT 认证，admin 角色可访问
/// - 使用 SqliteContainerRepository 实现真实数据库查询
/// - 支持分页：page(默认 1)/per_page(默认 20, 最大 100)
/// - 支持筛选：status(running/stopped/paused)
/// - 返回容器列表 + 分页信息
pub async fn list_containers(
    req: HttpRequest,
    query: web::Query<ContainersListQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteContainerRepository>>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).max(1).min(100);

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
            error: "Only admin users can list containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库获取容器列表
    match repo.get_containers(None, page, per_page) {
        Ok((containers, total)) => {
            // 5. 转换为响应格式
            let data: Vec<ContainerInfo> = containers.into_iter().map(|c| ContainerInfo {
                id: c.id as u64,
                name: c.name,
                image: c.image,
                status: c.status,
                created_at: c.created_at.to_string(),
            }).collect();

            // 6. 计算分页信息
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            Ok(HttpResponse::Ok().json(ContainerListResponse {
                success: true,
                data,
                pagination: PaginationInfo {
                    page,
                    per_page,
                    total,
                    total_pages: total_pages as u32,
                },
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: format!("查询容器列表失败：{}", e),
                code: "DATABASE_ERROR".to_string(),
            }))
        }
    }
}
