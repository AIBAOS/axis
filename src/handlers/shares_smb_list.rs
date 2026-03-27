// Phase 151: SMB 共享列表 API
// GET /api/v1/shares/smb — 获取 SMB 共享列表

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

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

/// 获取 SMB 共享列表（Phase 151）
/// - JWT 认证，仅 admin 角色可访问
/// - 支持分页：page(默认 1), per_page(默认 20, 最大 100)
/// - 支持状态过滤：status(active/inactive)
/// - 返回 SMB 共享列表 + 分页信息
pub async fn list_smb_shares(
    req: HttpRequest,
    query: web::Query<SmbSharesQuery>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let status_filter = query.status.clone();

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

    // 4. 模拟 SMB 共享数据
    let all_shares = vec![
        SmbShareInfo {
            id: 1,
            name: "Public".to_string(),
            path: "/srv/samba/public".to_string(),
            status: "active".to_string(),
            read_only: false,
            guest_access: true,
            enabled: true,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
        SmbShareInfo {
            id: 2,
            name: "Users".to_string(),
            path: "/srv/samba/users".to_string(),
            status: "active".to_string(),
            read_only: false,
            guest_access: false,
            enabled: true,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
        SmbShareInfo {
            id: 3,
            name: "Backup".to_string(),
            path: "/srv/samba/backup".to_string(),
            status: "inactive".to_string(),
            read_only: true,
            guest_access: false,
            enabled: false,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
        SmbShareInfo {
            id: 4,
            name: "Media".to_string(),
            path: "/srv/samba/media".to_string(),
            status: "active".to_string(),
            read_only: true,
            guest_access: true,
            enabled: true,
            created_at: 1711500000,
            updated_at: 1711500000,
        },
    ];

    // 5. 应用状态过滤
    let filtered_shares: Vec<SmbShareInfo> = if let Some(ref status) = status_filter {
        all_shares.into_iter().filter(|s| s.status == *status).collect()
    } else {
        all_shares
    };

    // 6. 应用分页
    let total = filtered_shares.len() as u64;
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;
    
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(filtered_shares.len());
    
    let shares = if start < filtered_shares.len() {
        filtered_shares[start..end].to_vec()
    } else {
        vec![]
    };

    // 7. 返回 SMB 共享列表
    Ok(HttpResponse::Ok().json(SmbShareListResponse {
        success: true,
        data: shares,
        pagination: PaginationInfo {
            page,
            per_page,
            total,
            total_pages,
        },
    }))
}
