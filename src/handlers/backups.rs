// 备份任务管理处理器（SQLite 持久化版）
// 包含：列表、详情、创建、删除、手动执行

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::database::backup_store::SqliteBackupRepository;

/// 查询参数
#[derive(Debug, Deserialize)]
pub struct BackupQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
    pub backup_type: Option<String>,
}

/// 创建备份请求
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBackupRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_backup_type")]
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
}

fn default_backup_type() -> String { "full".to_string() }

/// GET /api/v1/backups — 备份任务列表（分页 + 筛选）
pub async fn list_backups(
    query: web::Query<BackupQuery>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = std::cmp::min(query.per_page.unwrap_or(20), 100);

    match repo.get_backups(
        query.status.as_deref(),
        query.backup_type.as_deref(),
        page,
        per_page,
    ) {
        Ok((backups, total)) => {
            let total_pages = if total == 0 { 1 } else { (total as u32 + per_page - 1) / per_page };
            Ok(HttpResponse::Ok().json(json!({
                "success": true,
                "data": backups,
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
            "message": format!("查询备份列表失败: {}", e)
        }))),
    }
}

/// GET /api/v1/backups/{id} — 备份任务详情
pub async fn get_backup(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    match repo.get_backup_by_id(id) {
        Ok(Some(backup)) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "data": backup
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Backup task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询备份任务失败: {}", e)
        }))),
    }
}

/// POST /api/v1/backups — 创建备份任务
pub async fn create_backup(
    payload: web::Json<CreateBackupRequest>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse> {
    // 校验源路径非空
    if payload.source_path.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "source_path 不能为空"
        })));
    }
    if payload.destination_path.trim().is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "destination_path 不能为空"
        })));
    }

    match repo.create_backup(
        &payload.name,
        &payload.description,
        &payload.backup_type,
        &payload.source_path,
        &payload.destination_path,
        payload.schedule.as_deref(),
    ) {
        Ok(backup) => Ok(HttpResponse::Created().json(json!({
            "success": true,
            "data": backup
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("创建备份任务失败: {}", e)
        }))),
    }
}

/// DELETE /api/v1/backups/{id} — 删除备份任务
pub async fn delete_backup(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 先检查是否正在运行
    if let Ok(Some(backup)) = repo.get_backup_by_id(id) {
        if backup.status == "running" {
            return Ok(HttpResponse::Conflict().json(json!({
                "success": false,
                "message": format!("备份任务 '{}' 正在运行，无法删除", backup.name)
            })));
        }
    }

    match repo.delete_backup(id) {
        Ok(true) => Ok(HttpResponse::Ok().json(json!({
            "success": true,
            "message": format!("Backup task '{}' deleted", id)
        }))),
        Ok(false) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Backup task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("删除备份任务失败: {}", e)
        }))),
    }
}

/// POST /api/v1/backups/{id}/run — 手动执行备份
pub async fn run_backup(
    path: web::Path<i64>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    // 先检查任务是否存在
    match repo.get_backup_by_id(id) {
        Ok(Some(backup)) => {
            if backup.status == "running" {
                return Ok(HttpResponse::Conflict().json(json!({
                    "success": false,
                    "message": format!("备份任务 '{}' 已在运行中", backup.name)
                })));
            }

            match repo.run_backup(id) {
                Ok(true) => Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": format!("备份任务 '{}' 已开始执行", backup.name),
                    "backup_id": id,
                    "status": "running"
                }))),
                Ok(false) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": "启动备份任务失败：状态更新异常"
                }))),
                Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": format!("执行备份任务失败: {}", e)
                }))),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Backup task '{}' not found", id)
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("查询备份任务失败: {}", e)
        }))),
    }
}
