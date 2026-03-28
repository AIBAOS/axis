//! SMB 共享列表 Handler - Phase 202
//! 支持分页和筛选，任意登录用户可访问

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::database::share_store::ShareRepository;
use crate::middleware::jwt_auth::JwtClaims;

/// 列表查询参数
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// 页码（从 1 开始）
    pub page: Option<u32>,
    /// 每页数量（默认 20，最大 100）
    pub limit: Option<u32>,
    /// 筛选公共/私有共享
    pub public: Option<bool>,
}

/// SMB 共享信息
#[derive(Debug, Serialize, Clone)]
pub struct ShareInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub public: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 分页信息
#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u32,
    pub total_pages: u32,
}

/// 列表响应
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub success: bool,
    pub message: String,
    pub data: ListData,
}

#[derive(Debug, Serialize)]
pub struct ListData {
    pub items: Vec<ShareInfo>,
    pub pagination: Pagination,
}

/// 错误响应
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<String>,
}

impl ErrorResponse {
    pub fn unauthorized(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("UNAUTHORIZED".to_string()),
        }
    }

    pub fn internal(msg: &str) -> Self {
        ErrorResponse {
            success: false,
            message: msg.to_string(),
            error_code: Some("INTERNAL_ERROR".to_string()),
        }
    }
}

/// SMB 共享列表处理器
/// 
/// GET /api/v1/shares/smb
/// 
/// 认证：JWT Bearer Token（任意登录用户）
/// 查询参数：
/// - page: 页码（可选，默认 1）
/// - limit: 每页数量（可选，默认 20，最大 100）
/// - public: 筛选公共/私有共享（可选）
/// 
/// 响应：
/// - 200 OK: 返回共享列表（含分页信息）
/// - 401 Unauthorized: 未登录
pub async fn list_smb_shares(
    claims: JwtClaims,
    share_repo: web::Data<dyn ShareRepository>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    // 任意登录用户可访问（无需 admin 权限）
    let _user_id = claims.user_id;
    
    // 解析分页参数
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;
    
    // 获取总数
    let total = share_repo.count_smb_shares(query.public.as_ref())
        .await
        .map_err(|e| {
            log::error!("查询共享总数失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    // 计算总页数
    let total_pages = if total == 0 { 1 } else { (total + limit - 1) / limit };
    
    // 查询共享列表
    let shares = share_repo.list_smb_shares(limit, offset, query.public.as_ref())
        .await
        .map_err(|e| {
            log::error!("查询共享列表失败：{}", e);
            actix_web::error::ErrorInternalServerError("数据库查询失败")
        })?;
    
    // 转换为响应格式
    let items: Vec<ShareInfo> = shares
        .into_iter()
        .map(|s| ShareInfo {
            id: s.id,
            name: s.name,
            path: s.path,
            description: s.description,
            public: s.public,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    
    Ok(HttpResponse::Ok().json(ListResponse {
        success: true,
        message: "获取共享列表成功".to_string(),
        data: ListData {
            items,
            pagination: Pagination {
                page,
                limit,
                total,
                total_pages,
            },
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pagination_calculation() {
        // 总数 100，每页 20 → 5 页
        let total = 100;
        let limit = 20;
        let total_pages = if total == 0 { 1 } else { (total + limit - 1) / limit };
        assert_eq!(total_pages, 5);
        
        // 总数 101，每页 20 → 6 页
        let total = 101;
        let total_pages = if total == 0 { 1 } else { (total + limit - 1) / limit };
        assert_eq!(total_pages, 6);
        
        // 总数 0 → 1 页
        let total = 0;
        let total_pages = if total == 0 { 1 } else { (total + limit - 1) / limit };
        assert_eq!(total_pages, 1);
    }
    
    #[test]
    fn test_query_params() {
        // 默认值
        let query = ListQuery {
            page: None,
            limit: None,
            public: None,
        };
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(20).min(100);
        assert_eq!(page, 1);
        assert_eq!(limit, 20);
        
        // 自定义值
        let query = ListQuery {
            page: Some(5),
            limit: Some(50),
            public: Some(true),
        };
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(20).min(100);
        assert_eq!(page, 5);
        assert_eq!(limit, 50);
        
        // 超过最大值
        let query = ListQuery {
            page: Some(1),
            limit: Some(200),
            public: None,
        };
        let limit = query.limit.unwrap_or(20).min(100);
        assert_eq!(limit, 100);
    }
    
    #[test]
    fn test_error_response() {
        let err = ErrorResponse::unauthorized("未授权访问");
        assert!(!err.success);
        assert_eq!(err.message, "未授权访问");
        assert_eq!(err.error_code, Some("UNAUTHORIZED".to_string()));
        
        let err = ErrorResponse::internal("服务器错误");
        assert_eq!(err.error_code, Some("INTERNAL_ERROR".to_string()));
    }
}
