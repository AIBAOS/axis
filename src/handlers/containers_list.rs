// Phase 168: 容器列表 API
// GET /api/v1/containers — 获取容器列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

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

/// 获取容器列表（Phase 168）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
/// - 返回容器列表 + 分页信息
pub async fn list_containers(
    req: HttpRequest,
    query: web::Query<ContainersListQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

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

    // 4. 模拟容器数据
    let all_containers = vec![
        ContainerInfo {
            id: 1,
            name: "nginx-web".to_string(),
            image: "nginx:latest".to_string(),
            status: "running".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 2,
            name: "postgres-db".to_string(),
            image: "postgres:15".to_string(),
            status: "running".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 3,
            name: "redis-cache".to_string(),
            image: "redis:7".to_string(),
            status: "stopped".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 4,
            name: "mongo-db".to_string(),
            image: "mongo:6".to_string(),
            status: "running".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 5,
            name: "node-app".to_string(),
            image: "node:18".to_string(),
            status: "paused".to_string(),
            created_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 5. 应用分页
    let total = all_containers.len() as u64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(all_containers.len());
    
    let containers = if start < all_containers.len() {
        all_containers[start..end].to_vec()
    } else {
        vec![]
    };

    // 6. 返回容器列表
    Ok(HttpResponse::Ok().json(ContainerListResponse {
        success: true,
        data: containers,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
