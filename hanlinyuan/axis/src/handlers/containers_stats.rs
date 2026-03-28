// Phase 150: 容器统计信息 API
// GET /api/v1/containers/{id}/stats — 获取容器统计信息

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 容器统计信息响应
#[derive(Serialize)]
pub struct ContainerStatsResponse {
    pub success: bool,
    pub data: ContainerStats,
}

/// 容器统计信息（扁平化结构）
#[derive(Serialize)]
pub struct ContainerStats {
    pub container_id: u64,
    pub cpu_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_limit_bytes: u64,
    pub memory_percent: f64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub block_read_bytes: u64,
    pub block_write_bytes: u64,
    pub pids: u32,
    pub status: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取容器统计信息（Phase 150）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 返回容器统计信息（CPU、内存、网络、磁盘等）
pub async fn get_container_stats(
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
            error: "Only admin users can view container stats".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟容器数据
    let mock_containers = vec![
        (1, "nginx-web", "running"),
        (2, "postgres-db", "running"),
        (3, "redis-cache", "stopped"),
    ];

    // 5. 验证容器 ID 存在性
    let container = mock_containers.iter().find(|(id, _, _)| *id == container_id);
    
    if container.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Container {} not found", container_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let container = container.unwrap();

    // 6. 模拟容器统计信息（实际实现中会调用 Docker API 获取统计信息）
    let stats = if container.2 == "running" {
        ContainerStats {
            container_id,
            cpu_percent: 2.5,
            memory_usage_bytes: 134217728,      // 128 MB
            memory_limit_bytes: 536870912,      // 512 MB
            memory_percent: 25.0,
            network_rx_bytes: 1048576,
            network_tx_bytes: 524288,
            block_read_bytes: 2097152,
            block_write_bytes: 1048576,
            pids: 10,
            status: container.2.to_string(),
        }
    } else {
        // 已停止的容器返回零统计
        ContainerStats {
            container_id,
            cpu_percent: 0.0,
            memory_usage_bytes: 0,
            memory_limit_bytes: 0,
            memory_percent: 0.0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            block_read_bytes: 0,
            block_write_bytes: 0,
            pids: 0,
            status: container.2.to_string(),
        }
    };

    // 7. 返回容器统计信息
    Ok(HttpResponse::Ok().json(ContainerStatsResponse {
        success: true,
        data: stats,
    }))
}
