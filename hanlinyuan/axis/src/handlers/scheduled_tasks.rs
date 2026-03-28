// 计划任务管理处理器（SQLite 持久化版）
// 包含：列表、详情、创建、更新、删除、启用/禁用、手动执行

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::scheduled_task_store::SqliteScheduledTaskRepository;

#[derive(Debug, Deserialize)]
pub struct ScheduledTaskQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateScheduledTaskRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_task_type")]
    pub task_type: String,
    pub cron_expression: String,
    pub command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateScheduledTaskRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cron_expression: Option<String>,
    pub command: Option<String>,
}

fn default_task_type() -> String { "system".to_string() }

/// GET /api/v1/scheduled-tasks — 计划任务列表
pub async fn list_scheduled_tasks(
    query: web::Query<ScheduledTaskQuery>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

    match repo.get_tasks(query.status.as_deref(), query.enabled, page, per_page) {
        Ok((tasks, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": tasks,
                "pagination": {
                    "page": page,
                    "per_page": per_page,
                    "total": total,
                    "total_pages": total_pages
                }
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询计划任务失败: {}", e)
        }))),
    }
}

/// GET /api/v1/scheduled-tasks/{id} — 计划任务详情
pub async fn get_scheduled_task(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_task_by_id(id) {
        Ok(Some(task)) => Ok(HttpResponse::Ok().json(json!({ "success": true, "data": task }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Scheduled task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询计划任务失败: {}", e)
        }))),
    }
}

/// POST /api/v1/scheduled-tasks — 创建计划任务
pub async fn create_scheduled_task(
    payload: web::Json<CreateScheduledTaskRequest>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    if payload.name.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false, "message": "name 不能为空"
        })));
    }
    if payload.cron_expression.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false, "message": "cron_expression 不能为空"
        })));
    }
    if payload.command.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false, "message": "command 不能为空"
        })));
    }

    match repo.create_task(
        &payload.name, &payload.description, &payload.task_type,
        &payload.cron_expression, &payload.command,
    ) {
        Ok(task) => Ok(HttpResponse::Created().json(json!({ "success": true, "data": task }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false, "message": format!("创建计划任务失败: {}", e)
        }))),
    }
}

/// PUT /api/v1/scheduled-tasks/{id} — 更新计划任务
pub async fn update_scheduled_task(
    path: web::Path<i64>,
    payload: web::Json<UpdateScheduledTaskRequest>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 先检查是否存在
    match repo.get_task_by_id(id) {
        Ok(Some(task)) => {
            if task.status == "running" {
                return Ok(HttpResponse::Conflict().json(json!({
                    "success": false,
                    "message": format!("计划任务 '{}' 正在运行，无法修改", task.name)
                })));
            }

            match repo.update_task(
                id,
                payload.name.as_deref(),
                payload.description.as_deref(),
                payload.cron_expression.as_deref(),
                payload.command.as_deref(),
            ) {
                Ok(true) => {
                    match repo.get_task_by_id(id) {
                        Ok(Some(updated)) => Ok(HttpResponse::Ok().json(json!({ "success": true, "data": updated }))),
                        _ => Ok(HttpResponse::Ok().json(json!({ "success": true, "message": "更新成功" }))),
                    }
                }
                Ok(false) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false, "message": "更新失败：数据库异常"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false, "message": format!("更新计划任务失败: {}", e)
                }))),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false, "message": format!("Scheduled task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false, "message": format!("查询计划任务失败: {}", e)
        }))),
    }
}

/// DELETE /api/v1/scheduled-tasks/{id} — 删除计划任务
pub async fn delete_scheduled_task(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    if let Ok(Some(task)) = repo.get_task_by_id(id) {
        if task.status == "running" {
            return Ok(HttpResponse::Conflict().json(json!({
                "success": false,
                "message": format!("计划任务 '{}' 正在运行，无法删除", task.name)
            })));
        }
    }

    match repo.delete_task(id) {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "success": true, "message": format!("Scheduled task '{}' deleted", id)
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false, "message": format!("Scheduled task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false, "message": format!("删除计划任务失败: {}", e)
        }))),
    }
}

/// POST /api/v1/scheduled-tasks/{id}/toggle — 启用/禁用切换
pub async fn toggle_scheduled_task(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.toggle_task(id) {
        Ok(Some(new_state)) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "enabled": new_state,
            "message": if new_state { "任务已启用" } else { "任务已禁用" }
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false, "message": format!("Scheduled task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false, "message": format!("切换任务状态失败: {}", e)
        }))),
    }
}

/// POST /api/v1/scheduled-tasks/{id}/run — 手动执行
pub async fn run_scheduled_task(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteScheduledTaskRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_task_by_id(id) {
        Ok(Some(task)) => {
            if !task.enabled {
                return Ok(HttpResponse::Conflict().json(json!({
                    "success": false, "message": format!("计划任务 '{}' 已禁用，无法执行", task.name)
                })));
            }
            if task.status == "running" {
                return Ok(HttpResponse::Conflict().json(json!({
                    "success": false, "message": format!("计划任务 '{}' 已在运行中", task.name)
                })));
            }

            match repo.run_task(id) {
                Ok(true) => Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": format!("计划任务 '{}' 已开始执行", task.name),
                    "task_id": id,
                    "status": "running"
                }))),
                Ok(false) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false, "message": "启动任务失败：状态更新异常"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false, "message": format!("执行计划任务失败: {}", e)
                }))),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false, "message": format!("Scheduled task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false, "message": format!("查询计划任务失败: {}", e)
        }))),
    }
}
