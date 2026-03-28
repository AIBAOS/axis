// Phase 195: 备份执行历史 API
// GET /api/v1/backups/{id}/execution-history — 获取备份任务执行历史记录

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::backup_store::SqliteBackupRepository;

/// 查询参数
#[derive(Debug, serde::Deserialize)]
pub struct ExecutionHistoryQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// 执行历史条目响应
#[derive(Serialize)]
pub struct ExecutionHistoryItem {
    pub execution_id: i64,
    pub backup_id: i64,
    pub status: String,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub error_message: Option<String>,
}

/// 执行历史列表响应
#[derive(Serialize)]
pub struct ExecutionHistoryResponse {
    pub success: bool,
    pub data: ExecutionHistoryData,
}

/// 执行历史数据
#[derive(Serialize)]
pub struct ExecutionHistoryData {
    pub backup_id: i64,
    pub executions: Vec<ExecutionHistoryItem>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取备份执行历史（Phase 195）
/// - JWT 认证，登录用户可访问
/// - 验证备份 ID 存在性（404 Not Found）
/// - 支持分页：page(默认 1)/per_page(默认 20)
/// - 按 started_at 降序排列（最新的在前）
pub async fn get_backup_execution_history(
    req: HttpRequest,
    path: web::Path<i64>,
    query: web::Query<ExecutionHistoryQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteBackupRepository>>,
) -> Result<HttpResponse, Error> {
    let backup_id = path.into_inner();
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

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

    // 3. 验证登录用户权限（任意登录用户可访问）
    let _ = claims;

    // 4. 获取执行历史
    match repo.get_execution_history(backup_id, page, per_page) {
        Ok((executions, total)) => {
            // 5. 转换为响应格式
            let items: Vec<ExecutionHistoryItem> = executions
                .into_iter()
                .map(|e| ExecutionHistoryItem {
                    execution_id: e.id,
                    backup_id: e.backup_id,
                    status: e.status,
                    started_at: e.started_at,
                    completed_at: e.completed_at,
                    duration_seconds: e.duration_seconds,
                    error_message: e.error_message,
                })
                .collect();

            Ok(HttpResponse::Ok().json(ExecutionHistoryResponse {
                success: true,
                data: ExecutionHistoryData {
                    backup_id,
                    executions: items,
                    total,
                    page,
                    per_page,
                },
            }))
        }
        Err(e) => {
            // 检查是否是备份不存在
            if e.contains("not found") {
                Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("Backup {} not found", backup_id),
                    code: "NOT_FOUND".to_string(),
                }))
            } else {
                Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    success: false,
                    error: format!("查询执行历史失败：{}", e),
                    code: "DATABASE_ERROR".to_string(),
                }))
            }
        }
    }
}
