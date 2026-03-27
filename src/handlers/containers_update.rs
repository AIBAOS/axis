// Phase 170: 容器更新 API
// PUT /api/v1/containers/{id} — 更新容器配置

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 更新容器请求
#[derive(Debug, Deserialize)]
pub struct UpdateContainerRequest {
    pub name: Option<String>,
    pub image: Option<String>,
    pub ports: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub cpu_limit: Option<f64>,
    pub memory_limit: Option<u64>,
}

/// 容器信息
#[derive(Serialize, Clone)]
pub struct ContainerInfo {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
    pub networks: Vec<String>,
    pub env: Vec<String>,
    pub cpu_limit: Option<f64>,
    pub memory_limit: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}

/// 更新容器响应
#[derive(Serialize)]
pub struct UpdateContainerResponse {
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

/// 验证容器名称格式
fn validate_container_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 128
}

/// 验证镜像名称格式
fn validate_image_name(image: &str) -> bool {
    !image.is_empty() && image.len() <= 256
}

/// 更新容器配置（Phase 170）
/// - JWT 认证，admin 角色可访问
/// - 请求体包含：name/image/ports/networks/env/cpu_limit/memory_limit（可选，部分更新）
/// - 验证容器 ID 存在性（404 Not Found）
/// - 验证名称格式（400 Bad Request）
/// - 验证镜像格式（400 Bad Request）
/// - 验证名称唯一性（409 Conflict，排除自身）
/// - 更新成功返回 200 OK + 容器详情
pub async fn update_container(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateContainerRequest>,
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
            error: "Only admin users can update containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证容器名称格式（如果提供）
    if let Some(ref name) = payload.name {
        if !validate_container_name(name) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid container name. Must be 1-128 chars".to_string(),
                code: "INVALID_NAME".to_string(),
            }));
        }
    }

    // 5. 验证镜像名称格式（如果提供）
    if let Some(ref image) = payload.image {
        if !validate_image_name(image) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid image name. Must be 1-256 chars".to_string(),
                code: "INVALID_IMAGE".to_string(),
            }));
        }
    }

    // 6. 模拟现有容器数据
    let mut mock_containers = vec![
        ContainerInfo {
            id: 1,
            name: "nginx-web".to_string(),
            image: "nginx:latest".to_string(),
            status: "running".to_string(),
            ports: vec!["80:80".to_string(), "443:443".to_string()],
            networks: vec!["bridge".to_string()],
            env: vec![],
            cpu_limit: None,
            memory_limit: None,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 2,
            name: "postgres-db".to_string(),
            image: "postgres:15".to_string(),
            status: "running".to_string(),
            ports: vec!["5432:5432".to_string()],
            networks: vec!["bridge".to_string()],
            env: vec!["POSTGRES_PASSWORD=secret".to_string()],
            cpu_limit: Some(2.0),
            memory_limit: Some(536870912),
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
        ContainerInfo {
            id: 3,
            name: "redis-cache".to_string(),
            image: "redis:7".to_string(),
            status: "stopped".to_string(),
            ports: vec!["6379:6379".to_string()],
            networks: vec!["bridge".to_string()],
            env: vec![],
            cpu_limit: None,
            memory_limit: None,
            created_at: "2026-03-27T06:00:00Z".to_string(),
            updated_at: "2026-03-27T06:00:00Z".to_string(),
        },
    ];

    // 7. 查找容器
    let container_index = mock_containers.iter().position(|c| c.id == container_id);

    // 8. 验证容器存在性
    if container_index.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Container {} not found", container_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let container_index = container_index.unwrap();

    // 9. 验证名称唯一性（排除自身）
    if let Some(ref new_name) = payload.name {
        let name_exists = mock_containers.iter().any(|c| c.id != container_id && c.name == *new_name);
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("Container name '{}' already exists", new_name),
                code: "NAME_CONFLICT".to_string(),
            }));
        }
    }

    // 10. 部分更新容器配置
    let container = &mut mock_containers[container_index];
    
    if let Some(new_name) = &payload.name {
        container.name = new_name.clone();
    }
    if let Some(new_image) = &payload.image {
        container.image = new_image.clone();
    }
    if let Some(new_ports) = &payload.ports {
        container.ports = new_ports.clone();
    }
    if let Some(new_networks) = &payload.networks {
        container.networks = new_networks.clone();
    }
    if let Some(new_env) = &payload.env {
        container.env = new_env.clone();
    }
    if let Some(new_cpu_limit) = payload.cpu_limit {
        container.cpu_limit = Some(new_cpu_limit);
    }
    if let Some(new_memory_limit) = payload.memory_limit {
        container.memory_limit = Some(new_memory_limit);
    }

    // 11. 更新时间戳
    let now = chrono::Utc::now().to_rfc3339();
    container.updated_at = now.clone();

    // 12. 返回更新成功
    Ok(HttpResponse::Ok().json(UpdateContainerResponse {
        success: true,
        message: "Container updated successfully".to_string(),
        data: container.clone(),
    }))
}
