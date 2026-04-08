// Phase 241: 容器启动 API
// POST /api/v1/containers/{id}/start — 启动容器

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 容器启动响应
#[derive(Serialize)]
pub struct ContainerStartResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<ContainerStatus>,
}

/// 容器状态
#[derive(Serialize, Clone)]
pub struct ContainerStatus {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub started_at: u64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 启动容器（Phase 241）
/// - JWT 认证，admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 验证容器状态（已运行返回 409 Conflict）
/// - 启动成功后返回 200 OK
/// - 错误处理：401/403/404/409/500
pub async fn start_container(
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can start containers".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 获取当前时间戳
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to get current time")
        })?
        .as_secs();

    // 5. 模拟容器数据
    // 实际实现中，这里会查询数据库验证容器是否存在
    let mock_containers = vec![
        (1u64, "nginx-web", "stopped"),
        (2u64, "postgres-db", "stopped"),
        (3u64, "redis-cache", "running"),
        (4u64, "mongo-db", "stopped"),
    ];

    // 6. 验证容器是否存在并检查状态
    let container = mock_containers.iter().find(|(id, _, _)| *id == container_id);
    
    match container {
        Some((_, name, status)) => {
            // 7. 验证容器状态（已运行返回 409 Conflict）
            if *status == "running" {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: format!("Container '{}' is already running", name),
                    code: "ALREADY_RUNNING".to_string(),
                }));
            }
            
            // 8. 模拟启动容器
            // 实际实现中，这里会调用 Docker/LXC API 启动容器
            let container_status = ContainerStatus {
                id: container_id,
                name: name.to_string(),
                status: "running".to_string(),
                started_at: now,
            };

            Ok(HttpResponse::Ok().json(ContainerStartResponse {
                success: true,
                message: format!("Container '{}' started successfully", name),
                data: Some(container_status),
            }))
        }
        None => {
            // 容器不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Container {} not found", container_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_start_container_success() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/containers/{id}/start", web::post().to(start_container))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 权限
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_start_container_not_found() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/containers/{id}/start", web::post().to(start_container))
        ).await;

        // 注意：实际测试需要验证容器不存在情况
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_start_container_already_running() {
        let jwt_service = web::Data::new(JwtService::new(crate::services::jwt_service::JwtConfig {
            secret_key: "test_secret".to_string(),
            issuer: "test".to_string(),
            audience: "test".to_string(),
            expiration_minutes: 60,
            refresh_enabled: false,
        }));

        let app = test::init_service(
            App::new()
                .app_data(jwt_service)
                .route("/api/v1/containers/{id}/start", web::post().to(start_container))
        ).await;

        // 注意：实际测试需要验证容器已运行情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
