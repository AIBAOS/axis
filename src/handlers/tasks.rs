// 任务处理器（Phase 59+）
// 包含：任务列表/创建/更新/删除/状态管理

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

/// 任务状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Running => write!(f, "running"),
            TaskStatus::Completed => write!(f, "completed"),
            TaskStatus::Failed => write!(f, "failed"),
        }
    }
}

/// 任务信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub progress: u8,
    pub created_at: String,
    pub updated_at: String,
}

/// 任务列表分页参数
#[derive(Debug, Deserialize)]
pub struct TaskListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
}

impl Default for TaskListQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
            status: None,
        }
    }
}

/// 任务列表响应
#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub success: bool,
    pub data: Vec<Task>,
    pub pagination: TaskPagination,
}

#[derive(Debug, Serialize)]
pub struct TaskPagination {
    pub page: u64,
    pub limit: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 获取任务列表（分页 + 状态过滤）
pub async fn get_tasks(query: web::Query<TaskListQuery>) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20) as u64;
    let status_filter = query.status.as_deref();

    let mut mock_tasks = vec![
        Task {
            id: 1,
            name: "文件上传任务".to_string(),
            status: "completed".to_string(),
            progress: 100,
            created_at: "2026-03-18T20:00:00Z".to_string(),
            updated_at: "2026-03-18T20:10:00Z".to_string(),
        },
        Task {
            id: 2,
            name: "媒体转码任务".to_string(),
            status: "running".to_string(),
            progress: 65,
            created_at: "2026-03-18T21:00:00Z".to_string(),
            updated_at: "2026-03-19T03:00:00Z".to_string(),
        },
        Task {
            id: 3,
            name: "备份任务".to_string(),
            status: "pending".to_string(),
            progress: 0,
            created_at: "2026-03-19T02:00:00Z".to_string(),
            updated_at: "2026-03-19T02:00:00Z".to_string(),
        },
        Task {
            id: 4,
            name: "系统更新任务".to_string(),
            status: "failed".to_string(),
            progress: 80,
            created_at: "2026-03-19T01:00:00Z".to_string(),
            updated_at: "2026-03-19T01:30:00Z".to_string(),
        },
        Task {
            id: 5,
            name: "文件扫描任务".to_string(),
            status: "running".to_string(),
            progress: 45,
            created_at: "2026-03-19T02:30:00Z".to_string(),
            updated_at: "2026-03-19T03:10:00Z".to_string(),
        },
    ];

    // 状态过滤
    if let Some(status) = status_filter {
        mock_tasks.retain(|task| task.status == status);
    }

    let total = mock_tasks.len() as u64;
    let start = (page - 1) * limit;
    let end = start + limit;

    let paginated_tasks: Vec<Task> = mock_tasks
        .into_iter()
        .enumerate()
        .filter_map(|(i, task)| {
            let idx = i as u64;
            if idx >= start && idx < end {
                Some(task)
            } else {
                None
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(TaskListResponse {
        success: true,
        data: paginated_tasks,
        pagination: TaskPagination {
            page,
            limit,
            total,
            total_pages: (total + limit - 1) / limit,
        },
    }))
}

/// 获取单个任务
pub async fn get_task(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    let mock_tasks = vec![
        Task {
            id: 1,
            name: "文件上传任务".to_string(),
            status: "completed".to_string(),
            progress: 100,
            created_at: "2026-03-18T20:00:00Z".to_string(),
            updated_at: "2026-03-18T20:10:00Z".to_string(),
        },
        Task {
            id: 2,
            name: "媒体转码任务".to_string(),
            status: "running".to_string(),
            progress: 65,
            created_at: "2026-03-18T21:00:00Z".to_string(),
            updated_at: "2026-03-19T03:00:00Z".to_string(),
        },
    ];

    match mock_tasks.iter().find(|t| t.id == id) {
        Some(task) => Ok(HttpResponse::Ok().json(task.clone())),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Task {} not found", id)
        }))),
    }
}

/// 创建后台任务
pub async fn create_task(
    payload: web::Json<CreateTaskRequest>,
) -> Result<HttpResponse> {
    let name = &payload.name;

    Ok(HttpResponse::Created().json(Task {
        id: 100,
        name: name.clone(),
        status: "pending".to_string(),
        progress: 0,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// 更新任务
pub async fn update_task(
    path: web::Path<u64>,
    payload: web::Json<UpdateTaskRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证任务存在
    let mock_tasks = vec![1, 2, 3, 4, 5, 100];
    if !mock_tasks.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Task {} not found", id)
        })));
    }

    Ok(HttpResponse::Ok().json(Task {
        id,
        name: payload.name.clone().unwrap_or_else(|| "unknown".to_string()),
        status: payload.status.as_deref().unwrap_or("pending").to_string(),
        progress: payload.progress.unwrap_or(0),
        created_at: "2026-03-18T20:00:00Z".to_string(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// 删除任务
pub async fn delete_task(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证任务存在
    let mock_tasks = vec![1, 2, 3, 4, 5, 100];
    if !mock_tasks.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Task {} not found", id)
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Task {} deleted", id)
    })))
}

/// 取消任务
pub async fn cancel_task(
    path: web::Path<u64>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 简化模拟：验证任务存在且可取消
    let cancelable_tasks = vec![1, 2, 3, 4, 5];
    if !cancelable_tasks.contains(&id) {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Task {} not found or cannot be cancelled", id)
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Task {} cancelled", id)
    })))
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    pub params: Option<serde_json::Value>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub progress: Option<u8>,
    pub error_message: Option<String>,
}
