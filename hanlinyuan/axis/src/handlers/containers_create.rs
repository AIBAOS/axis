// Phase 169: 容器创建 API
// POST /api/v1/containers — 创建容器

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 创建容器请求
#[derive(Debug, Deserialize)]
pub struct CreateContainerRequest {
    pub name: String,
    pub image: String,
    pub ports: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
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
    pub created_at: String,
}

/// 创建容器响应
#[derive(Serialize)]
pub struct CreateContainerResponse {
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

/// 创建容器（Phase 169）
/// - JWT 认证，admin 角色可访问
/// - 请求体包含：name/image/ports/networks/env
/// - 验证名称格式（400 Bad Request）
/// - 验证镜像格式（400 Bad Request）
/// - 验证名称唯一性（409 Conflict）
/// - 创建成功返回 201 Created + 容器详情
pub async fn create_container(
    req: HttpRequest,
    payload: web::Json<CreateContainerRequest>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
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
            error: "Only admin users can create containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 验证容器名称格式
    if !validate_container_name(&payload.name) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid container name. Must be 1-128 chars".to_string(),
            code: "INVALID_NAME".to_string(),
        }));
    }

    // 5. 验证镜像名称格式
    if !validate_image_name(&payload.image) {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Invalid image name. Must be 1-256 chars".to_string(),
            code: "INVALID_IMAGE".to_string(),
        }));
    }

    // 6. 模拟现有容器数据（用于名称唯一性检查）
    let existing_containers = vec!["nginx-web", "postgres-db", "redis-cache"];

    // 7. 验证名称唯一性
    if existing_containers.iter().any(|n| n == &payload.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Container name '{}' already exists", payload.name),
            code: "NAME_CONFLICT".to_string(),
        }));
    }

    // 8. 模拟创建容器
    let now = chrono::Utc::now().to_rfc3339();
    let new_container = ContainerInfo {
        id: 6, // 模拟自增 ID
        name: payload.name.clone(),
        image: payload.image.clone(),
        status: "created".to_string(),
        ports: payload.ports.clone().unwrap_or_default(),
        networks: payload.networks.clone().unwrap_or_else(|| vec!["bridge".to_string()]),
        created_at: now,
    };

    // 9. 返回创建成功
    Ok(HttpResponse::Created().json(CreateContainerResponse {
        success: true,
        message: "Container created successfully".to_string(),
        data: new_container,
    }))
}
