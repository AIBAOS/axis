// Phase 146: 容器停止 API
// POST /api/v1/containers/{id}/stop — 停止容器

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 容器信息
#[derive(Serialize, Clone)]
pub struct ContainerInfo {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
    pub networks: Vec<String>,
    pub created_at: u64,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<u64>,
}

/// 停止容器响应
#[derive(Serialize)]
pub struct StopContainerResponse {
    pub success: bool,
    pub message: String,
    pub data: ContainerInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 停止容器（Phase 146）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 验证容器当前状态（已停止则返回 409 Conflict）
/// - 停止成功返回 200 OK + 容器详情
pub async fn stop_container(
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
            error: "Only admin users can stop containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟容器数据
    let mut mock_containers = vec![
        ContainerInfo {
            id: 1,
            name: "nginx-web".to_string(),
            image: "nginx:latest".to_string(),
            status: "running".to_string(),
            ports: vec!["80:80".to_string(), "443:443".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: 1711500000,
            cpu_usage: Some(0.5),
            memory_usage: Some(128 * 1024 * 1024),
        },
        ContainerInfo {
            id: 2,
            name: "postgres-db".to_string(),
            image: "postgres:15".to_string(),
            status: "running".to_string(),
            ports: vec!["5432:5432".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: 1711500000,
            cpu_usage: Some(1.2),
            memory_usage: Some(512 * 1024 * 1024),
        },
        ContainerInfo {
            id: 3,
            name: "redis-cache".to_string(),
            image: "redis:7".to_string(),
            status: "stopped".to_string(),
            ports: vec!["6379:6379".to_string()],
            networks: vec!["bridge".to_string()],
            created_at: 1711500000,
            cpu_usage: None,
            memory_usage: None,
        },
    ];

    // 5. 验证容器 ID 存在性
    let container_index = mock_containers.iter().position(|c| c.id == container_id);
    
    if container_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Container {} not found", container_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let container_index = container_index.unwrap();
    let container = &mock_containers[container_index];

    // 6. 验证容器当前状态（已停止则返回 409 Conflict）
    if container.status == "stopped" {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Container '{}' is already stopped", container.name),
            code: "ALREADY_STOPPED".to_string(),
        }));
    }

    // 7. 模拟停止容器（更新状态）
    mock_containers[container_index].status = "stopped".to_string();
    mock_containers[container_index].cpu_usage = None;
    mock_containers[container_index].memory_usage = None;

    // 8. 返回停止成功 + 容器详情
    Ok(HttpResponse::Ok().json(StopContainerResponse {
        success: true,
        message: "Container stopped successfully".to_string(),
        data: mock_containers[container_index].clone(),
    }))
}
