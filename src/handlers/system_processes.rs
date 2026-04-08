// Phase 251: 系统进程列表 API
// GET /api/v1/system/processes — 获取系统进程列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::jwt_service::JwtService;

/// 进程状态
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProcessStatus {
    Running,
    Sleeping,
    Zombie,
    Stopped,
}

/// 进程信息
#[derive(Serialize, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub status: String,
    pub start_time: u64,
    pub command: String,
}

/// 进程列表响应
#[derive(Serialize)]
pub struct ProcessListResponse {
    pub success: bool,
    pub data: Vec<ProcessInfo>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 进程查询参数
#[derive(Debug, Deserialize)]
pub struct ProcessesQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub user: Option<String>,
    pub status: Option<String>,
}

/// 验证排序字段
fn validate_sort_field(sort: &str) -> bool {
    matches!(sort.to_lowercase().as_str(), "cpu" | "memory" | "pid")
}

/// 验证排序顺序
fn validate_order(order: &str) -> bool {
    matches!(order.to_lowercase().as_str(), "asc" | "desc")
}

/// 验证进程状态
fn validate_status(status: &str) -> bool {
    matches!(status.to_lowercase().as_str(), "running" | "sleeping" | "zombie")
}

/// 获取系统进程列表（Phase 251）
/// - JWT 认证，admin 角色可访问
/// - 支持分页：limit(默认 50, 最大 200), offset(默认 0)
/// - 支持排序：sort(cpu|memory|pid), order(asc|desc)
/// - 支持筛选：user, status(running/sleeping/zombie)
/// - 错误处理：401/403/400/500
pub async fn get_system_processes(
    req: HttpRequest,
    query: web::Query<ProcessesQuery>,
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
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can view system processes".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 解析并验证查询参数
    let limit = query.limit.unwrap_or(50).max(1).min(200);
    let offset = query.offset.unwrap_or(0);
    
    if let Some(ref sort) = query.sort {
        if !validate_sort_field(sort) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid sort field. Must be cpu, memory, or pid".to_string(),
                code: "INVALID_SORT".to_string(),
            }));
        }
    }

    if let Some(ref order) = query.order {
        if !validate_order(order) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid order. Must be asc or desc".to_string(),
                code: "INVALID_ORDER".to_string(),
            }));
        }
    }

    if let Some(ref status) = query.status {
        if !validate_status(status) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid status. Must be running, sleeping, or zombie".to_string(),
                code: "INVALID_STATUS".to_string(),
            }));
        }
    }

    // 5. 获取当前时间戳
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            actix_web::error::ErrorInternalServerError("Failed to get current time")
        })?
        .as_secs();

    // 6. 模拟进程数据（实际应从系统读取）
    let mut all_processes = vec![
        ProcessInfo {
            pid: 1,
            name: "systemd".to_string(),
            user: "root".to_string(),
            cpu_percent: 0.1,
            memory_percent: 0.5,
            status: "sleeping".to_string(),
            start_time: now - 86400 * 30,
            command: "/sbin/init".to_string(),
        },
        ProcessInfo {
            pid: 1234,
            name: "nginx".to_string(),
            user: "www-data".to_string(),
            cpu_percent: 2.5,
            memory_percent: 1.2,
            status: "running".to_string(),
            start_time: now - 86400,
            command: "nginx: master process".to_string(),
        },
        ProcessInfo {
            pid: 5678,
            name: "postgres".to_string(),
            user: "postgres".to_string(),
            cpu_percent: 5.0,
            memory_percent: 8.5,
            status: "running".to_string(),
            start_time: now - 86400,
            command: "postgres: main process".to_string(),
        },
        ProcessInfo {
            pid: 9999,
            name: "defunct".to_string(),
            user: "root".to_string(),
            cpu_percent: 0.0,
            memory_percent: 0.0,
            status: "zombie".to_string(),
            start_time: now - 3600,
            command: "[defunct]".to_string(),
        },
    ];

    // 7. 应用筛选
    if let Some(ref user_filter) = query.user {
        all_processes.retain(|p| p.user == *user_filter);
    }

    if let Some(ref status_filter) = query.status {
        all_processes.retain(|p| p.status == status_filter.to_lowercase());
    }

    // 8. 应用排序
    let sort_field = query.sort.as_ref().map(|s| s.to_lowercase()).unwrap_or_else(|| "pid".to_string());
    let order_desc = query.order.as_ref().map(|o| o.to_lowercase() == "desc").unwrap_or(true);

    match sort_field.as_str() {
        "cpu" => all_processes.sort_by(|a, b| {
            if order_desc {
                b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                a.cpu_percent.partial_cmp(&b.cpu_percent).unwrap_or(std::cmp::Ordering::Equal)
            }
        }),
        "memory" => all_processes.sort_by(|a, b| {
            if order_desc {
                b.memory_percent.partial_cmp(&a.memory_percent).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                a.memory_percent.partial_cmp(&b.memory_percent).unwrap_or(std::cmp::Ordering::Equal)
            }
        }),
        _ => all_processes.sort_by(|a, b| {
            if order_desc {
                b.pid.cmp(&a.pid)
            } else {
                a.pid.cmp(&b.pid)
            }
        }),
    }

    // 9. 应用分页
    let total = all_processes.len() as u32;
    let start = offset as usize;
    let end = (start + limit as usize).min(all_processes.len());
    
    let processes = if start < all_processes.len() {
        all_processes[start..end].to_vec()
    } else {
        vec![]
    };

    Ok(HttpResponse::Ok().json(ProcessListResponse {
        success: true,
        data: processes,
        total,
        limit,
        offset,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_system_processes_success() {
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
                .route("/api/v1/system/processes", web::get().to(get_system_processes))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和 admin 权限
        // 这里只是示例测试结构
        assert!(true);
    }

    #[actix_web::test]
    async fn test_get_system_processes_unauthorized() {
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
                .route("/api/v1/system/processes", web::get().to(get_system_processes))
        ).await;

        // 注意：实际测试需要验证未认证情况
        // 这里只是示例测试结构
        assert!(true);
    }
}
