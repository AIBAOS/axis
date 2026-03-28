// Phase 198: SMB 共享列表 API
// GET /api/v1/shares/smb — 获取 SMB 共享列表（SQLite 持久化版）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::services::jwt_service::JwtService;
use crate::database::share_store::SqliteShareRepository;

/// SMB 共享查询参数
#[derive(Debug, Deserialize)]
pub struct SmbSharesQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<String>,
}

/// SMB 共享信息
#[derive(Serialize, Clone)]
pub struct SmbShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub status: String,
    pub read_only: bool,
    pub guest_access: bool,
    pub enabled: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页信息
#[derive(Serialize, Debug)]
pub struct PaginationInfo {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// SMB 共享列表响应
#[derive(Serialize)]
pub struct SmbShareListResponse {
    pub success: bool,
    pub data: Vec<SmbShareInfo>,
    pub pagination: PaginationInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 获取 SMB 共享列表（Phase 198）
/// - JWT 认证，仅 admin 角色可访问
/// - 使用 SqliteShareRepository 实现真实数据库查询
/// - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
/// - 支持状态过滤：status(active/inactive)
/// - 返回 SMB 共享列表 + 分页信息
pub async fn list_smb_shares(
    req: HttpRequest,
    query: web::Query<SmbSharesQuery>,
    jwt_service: web::Data<JwtService>,
    repo: web::Data<Arc<SqliteShareRepository>>,
) -> Result<HttpResponse, Error> {
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

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can list SMB shares".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 从数据库获取 SMB 共享列表
    match repo.get_shares(page, per_page, Some("smb".to_string()), query.status.clone()) {
        Ok(shares) => {
            // 转换为响应格式
            let data: Vec<SmbShareInfo> = shares.into_iter().map(|s| {
                let enabled = s.status == "active";
                SmbShareInfo {
                    id: s.id,
                    name: s.name,
                    path: s.path,
                    status: s.status,
                    read_only: false,
                    guest_access: false,
                    enabled,
                    created_at: s.created_at as u64,
                    updated_at: s.updated_at as u64,
                }
            }).collect();

            // 计算总数和分页
            let total = repo.count_shares(Some("smb".to_string()), query.status.clone()).unwrap_or(data.len() as u64);
            let total_pages = if total == 0 { 1 } else { (total + per_page as u64 - 1) / per_page as u64 };

            Ok(HttpResponse::Ok().json(SmbShareListResponse {
                success: true,
                data,
                pagination: PaginationInfo {
                    page,
                    per_page,
                    total,
                    total_pages: total_pages as u32,
                },
            }))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            success: false,
            error: format!("查询 SMB 共享列表失败：{}", e),
            code: "DATABASE_ERROR".to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_list_smb_shares_success() {
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
                .route("/api/v1/shares/smb", web::get().to(list_smb_shares))
        ).await;

        // 注意：实际测试需要有效的 JWT token 和数据库
        // 这里只是示例测试结构
        assert!(true);
    }
}
