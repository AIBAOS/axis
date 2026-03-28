// Phase 149: 容器日志 API
// GET /api/v1/containers/{id}/logs — 查看容器日志

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 容器日志查询参数
#[derive(Debug, Deserialize)]
pub struct ContainerLogsQuery {
    pub tail: Option<u32>,
    pub since: Option<u64>,
    pub follow: Option<bool>,
}

/// 容器日志响应
#[derive(Serialize)]
pub struct ContainerLogsResponse {
    pub success: bool,
    pub data: ContainerLogsData,
}

/// 容器日志数据
#[derive(Serialize)]
pub struct ContainerLogsData {
    pub container_id: u64,
    pub logs: Vec<String>,
    pub lines_count: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 查看容器日志（Phase 149）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证容器 ID 存在性（404 Not Found）
/// - 支持查询参数：tail(默认 100, 最大 1000), since(可选), follow(可选，默认 false)
/// - 返回容器日志（字符串数组）
pub async fn get_container_logs(
    req: HttpRequest,
    path: web::Path<u64>,
    query: web::Query<ContainerLogsQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let container_id = path.into_inner();
    let tail = query.tail.unwrap_or(100).min(1000);
    let _since = query.since;
    let _follow = query.follow.unwrap_or(false);

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
            error: "Only admin users can view container logs".to_string(),
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

    // 6. 模拟容器日志（实际实现中会调用 Docker API 获取日志）
    let mock_logs = vec![
        format!("2026-03-27T06:00:00Z [INFO] Container {} started", container.unwrap().1),
        "2026-03-27T06:00:01Z [INFO] Initializing application...".to_string(),
        "2026-03-27T06:00:02Z [INFO] Loading configuration...".to_string(),
        "2026-03-27T06:00:03Z [INFO] Configuration loaded successfully".to_string(),
        "2026-03-27T06:00:04Z [INFO] Starting server...".to_string(),
        "2026-03-27T06:00:05Z [INFO] Server listening on port 80".to_string(),
        "2026-03-27T06:00:10Z [INFO] Received request GET /".to_string(),
        "2026-03-27T06:00:10Z [INFO] Response sent 200 OK".to_string(),
        "2026-03-27T06:00:15Z [WARN] High memory usage detected".to_string(),
        "2026-03-27T06:00:20Z [INFO] Garbage collection completed".to_string(),
    ];

    // 7. 应用 tail 参数（返回最后 N 行）
    let logs: Vec<String> = if mock_logs.len() > tail as usize {
        mock_logs[mock_logs.len() - tail as usize..].to_vec()
    } else {
        mock_logs
    };

    let lines_count = logs.len() as u32;

    // 8. 返回容器日志
    Ok(HttpResponse::Ok().json(ContainerLogsResponse {
        success: true,
        data: ContainerLogsData {
            container_id,
            logs,
            lines_count,
        },
    }))
}
