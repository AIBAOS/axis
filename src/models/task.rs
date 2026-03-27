// 任务模型
use serde::{Deserialize, Serialize};

/// 任务状态枚举
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// 任务结构体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub params: Option<serde_json::Value>,
    pub scheduled_at: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub user_id: u64,
}

/// 任务列表查询参数
#[derive(Deserialize)]
pub struct TaskListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub status: Option<String>,
}

/// 任务列表响应
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskListResponse {
    pub success: bool,
    pub tasks: Vec<Task>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

/// 创建任务请求
#[derive(Deserialize)]
pub struct TaskCreateRequest {
    pub task_type: String,
    pub name: String,
    pub metadata: Option<serde_json::Value>,
}

/// 创建任务响应
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskCreateResponse {
    pub success: bool,
    pub task: Task,
}