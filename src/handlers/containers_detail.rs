// Phase 167: 容器详情 API
// GET /api/v1/containers/{id} — 获取容器详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 容器详情信息
#[derive(Serialize, Clone)]
pub struct ContainerDetail {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
    pub networks: Vec<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<u64>,
}

/// 容器详情响应
#[derive(Serialize)]
pub struct ContainerDetailResponse {
    pub success: bool,
    pub data: ContainerDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取容器详情（Phase 167）
/// - JWT 认证，admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 返回容器详情
pub async fn get_container_detail(
    req: HttpRequest,
    path: web::Path<u64>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let container_id = path.into_inner();

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
            error: "Only admin users can view container details".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟容器数据
    let mock_containers = vec![
        ContainerDetail {
            id: 1,
            name: "nginx-web".to_string(),
            image: "nginx:latest".to_string(),
            status: "running".to_string(),
            ports: vec!["80:80".to_string(), "443:443".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: "2026-03-27T06:00:00Z".to_string(),
            started_at: Some("2026-03-27T06:00:05Z".to_string()),
            cpu_usage: Some(2.5),
            memory_usage: Some(134217728),
        },
        ContainerDetail {
            id: 2,
            name: "postgres-db".to_string(),
            image: "postgres:15".to_string(),
            status: "running".to_string(),
            ports: vec!["5432:5432".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: "2026-03-27T06:00:00Z".to_string(),
            started_at: Some("2026-03-27T06:00:10Z".to_string()),
            cpu_usage: Some(5.0),
            memory_usage: Some(536870912),
        },
        ContainerDetail {
            id: 3,
            name: "redis-cache".to_string(),
            image: "redis:7".to_string(),
            status: "stopped".to_string(),
            ports: vec!["6379:6379".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: "2026-03-27T06:00:00Z".to_string(),
            started_at: None,
            cpu_usage: None,
            memory_usage: None,
        },
    ];

    // 5. 查找容器
    let container = mock_containers.into_iter().find(|c| c.id == container_id);

    // 6. 验证容器存在性
    match container {
        Some(container) => {
            // 7. 返回容器详情
            Ok(HttpResponse::Ok().json(ContainerDetailResponse {
                success: true,
                data: container,
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Container {} not found", container_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
