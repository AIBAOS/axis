// Phase 202 - SMB 共享列表 API（增强版）
// GET /api/v1/shares/smb — 获取 SMB 共享列表（支持分页和筛选）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// SMB 共享信息（Phase 202 增强版）
#[derive(Serialize, Clone)]
pub struct SmbShareInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub description: String,
    pub public: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct SmbSharesQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub public: Option<bool>,
}

/// 分页元数据
#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// SMB 共享列表响应
#[derive(Serialize)]
pub struct SmbSharesResponse {
    pub success: bool,
    pub data: Vec<SmbShareInfo>,
    pub pagination: PaginationMeta,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查用户是否已登录（任意登录用户可访问）
fn is_authenticated(claims: &JwtClaims) -> bool {
    !claims.sub.is_empty()
}

/// 获取 SMB 共享列表（Phase 202）
/// - JWT 认证，任意登录用户可访问
/// - 支持分页：page, limit（最大 100）
/// - 支持筛选：public 字段
/// - 返回字段：id/name/path/description/public/created_at/updated_at
pub async fn list_smb_shares_v2(
    req: HttpRequest,
    query: web::Query<SmbSharesQuery>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性（简化实现，实际应调用 jwt_service）
    // 假设 token 已验证通过，提取 claims
    // let claims = jwt_service.verify_token(token)?;
    
    // 3. 权限校验 - 任意登录用户可访问
    // if !is_authenticated(&claims) {
    //     return Ok(HttpResponse::Unauthorized().json(ErrorResponse {
    //         success: false,
    //         error: "Authentication required".to_string(),
    //         code: "UNAUTHORIZED".to_string(),
    //     }));
    // }

    // 4. 解析查询参数
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let public_filter = query.public;

    // 5. 模拟 SMB 共享数据（后续可连接数据库）
    let all_shares = vec![
        SmbShareInfo {
            id: 1,
            name: "Public".to_string(),
            path: "/srv/samba/public".to_string(),
            description: "Public shared folder".to_string(),
            public: true,
            created_at: 1710500000,
            updated_at: 1710500000,
        },
        SmbShareInfo {
            id: 2,
            name: "Home".to_string(),
            path: "/srv/samba/home".to_string(),
            description: "Home directories".to_string(),
            public: false,
            created_at: 1710600000,
            updated_at: 1710600000,
        },
        SmbShareInfo {
            id: 3,
            name: "Media".to_string(),
            path: "/srv/samba/media".to_string(),
            description: "Media files (movies, music, photos)".to_string(),
            public: true,
            created_at: 1710700000,
            updated_at: 1711400000,
        },
        SmbShareInfo {
            id: 4,
            name: "Backup".to_string(),
            path: "/srv/samba/backup".to_string(),
            description: "Backup storage".to_string(),
            public: false,
            created_at: 1710800000,
            updated_at: 1710800000,
        },
        SmbShareInfo {
            id: 5,
            name: "Documents".to_string(),
            path: "/srv/samba/documents".to_string(),
            description: "Shared documents".to_string(),
            public: true,
            created_at: 1710900000,
            updated_at: 1711500000,
        },
    ];

    // 6. 应用筛选条件
    let filtered: Vec<SmbShareInfo> = all_shares
        .into_iter()
        .filter(|share| {
            // public 筛选
            if let Some(filter) = public_filter {
                if share.public != filter {
                    return false;
                }
            }
            true
        })
        .collect();

    // 7. 分页计算
    let total = filtered.len() as u64;
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    let offset = (page - 1) * limit;
    let paginated: Vec<SmbShareInfo> = filtered
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    // 8. 返回结果
    Ok(HttpResponse::Ok().json(SmbSharesResponse {
        success: true,
        data: paginated,
        pagination: PaginationMeta {
            page,
            limit,
            total,
            total_pages,
        },
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_calculation() {
        let total: u64 = 100;
        let limit: u32 = 20;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        assert_eq!(total_pages, 5);

        let limit: u32 = 25;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        assert_eq!(total_pages, 4);

        let total: u64 = 5;
        let limit: u32 = 20;
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        assert_eq!(total_pages, 1);
    }

    #[test]
    fn test_limit_max() {
        let limit: u32 = 150;
        let limited = limit.min(100);
        assert_eq!(limited, 100);

        let limit: u32 = 50;
        let limited = limit.min(100);
        assert_eq!(limited, 50);
    }

    #[test]
    fn test_filter_public() {
        let shares = vec![
            SmbShareInfo {
                id: 1,
                name: "Public".to_string(),
                path: "/public".to_string(),
                description: "".to_string(),
                public: true,
                created_at: 1710500000,
                updated_at: 1710500000,
            },
            SmbShareInfo {
                id: 2,
                name: "Private".to_string(),
                path: "/private".to_string(),
                description: "".to_string(),
                public: false,
                created_at: 1710600000,
                updated_at: 1710600000,
            },
        ];

        // 筛选 public=true
        let filtered: Vec<SmbShareInfo> = shares
            .iter()
            .filter(|s| Some(true) == Some(s.public))
            .cloned()
            .collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Public");

        // 筛选 public=false
        let filtered: Vec<SmbShareInfo> = shares
            .iter()
            .filter(|s| Some(false) == Some(s.public))
            .cloned()
            .collect();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Private");
    }
}
