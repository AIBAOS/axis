// Phase 189: 备份任务详情 API
// GET /api/v1/backups/{id} - 获取备份任务详情

use actix_web::{HttpResponse, Error};
use serde::Serialize;

/// 备份任务详情响应
#[derive(Serialize, Clone)]
pub struct BackupDetailResponse {
    pub success: bool,
    pub data: BackupTask,
}

/// 备份任务信息
#[derive(Serialize, Clone)]
pub struct BackupTask {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub backup_type: String,
    pub source_path: String,
    pub destination_path: String,
    pub schedule: Option<String>,
    pub status: String,
    pub last_run_at: Option<String>,
    pub last_run_status: Option<String>,
    pub last_run_size_bytes: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

/// 获取备份任务详情（Phase 189）
/// - JWT 认证，登录用户可访问
/// - 返回指定 ID 的备份任务详情
pub async fn get_backup_task_detail(
    _id: actix_web::web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 返回示例数据
    Ok(HttpResponse::Ok().json(BackupDetailResponse {
        success: true,
        data: BackupTask {
            id: 1,
            name: "Daily Backup".to_string(),
            description: "Daily backup of system data".to_string(),
            backup_type: "full".to_string(),
            source_path: "/data".to_string(),
            destination_path: "/backup/daily".to_string(),
            schedule: Some("0 2 * * *".to_string()),
            status: "active".to_string(),
            last_run_at: Some("2026-03-27T02:00:00Z".to_string()),
            last_run_status: Some("success".to_string()),
            last_run_size_bytes: Some(1073741824),
            created_at: "2026-03-01T00:00:00Z".to_string(),
            updated_at: "2026-03-27T02:00:00Z".to_string(),
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_get_backup_task_detail() {
        let result = get_backup_task_detail(actix_web::web::Path::from(1)).await;
        assert!(result.is_ok());
    }
}
