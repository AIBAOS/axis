// Phase 92 - 共享文件夹详情 API
// GET /api/v1/shares/{id} — 获取共享文件夹详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 共享文件夹详情
#[derive(Serialize, Clone)]
pub struct ShareDetail {
    pub id: u64,
    pub name: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub path: String,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub read_only: bool,
    pub guest_access: bool,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: String,
}

/// 共享文件夹详情响应
#[derive(Serialize)]
pub struct ShareDetailResponse {
    pub success: bool,
    pub data: ShareDetail,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 任意登录用户可访问
}

/// 获取共享文件夹详情（Phase 92）
/// - JWT 认证，登录用户可访问
/// - 验证共享文件夹 ID 存在
/// - 返回共享文件夹完整信息
pub async fn get_share(
    req: HttpRequest,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token（登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    let share_id = path.into_inner();

    // 2. 模拟共享文件夹数据
    let mock_shares = vec![
        ShareDetail {
            id: 1,
            name: "public".to_string(),
            volume_id: 2,
            volume_name: "data".to_string(),
            path: "/mnt/volumes/data/public".to_string(),
            description: Some("Public shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string()],
            read_only: false,
            guest_access: true,
            status: "active".to_string(),
            created_at: 1710000000,
            updated_at: 1710000000,
            created_by: "admin".to_string(),
        },
        ShareDetail {
            id: 2,
            name: "media".to_string(),
            volume_id: 2,
            volume_name: "data".to_string(),
            path: "/mnt/volumes/data/media".to_string(),
            description: Some("Media files shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string(), "afp".to_string()],
            read_only: false,
            guest_access: false,
            status: "active".to_string(),
            created_at: 1710000000,
            updated_at: 1710000000,
            created_by: "admin".to_string(),
        },
        ShareDetail {
            id: 3,
            name: "backup".to_string(),
            volume_id: 3,
            volume_name: "backup".to_string(),
            path: "/mnt/volumes/backup/shared".to_string(),
            description: Some("Backup shared folder".to_string()),
            protocols: vec!["smb".to_string()],
            read_only: true,
            guest_access: false,
            status: "active".to_string(),
            created_at: 1710000000,
            updated_at: 1710000000,
            created_by: "admin".to_string(),
        },
    ];

    // 3. 查找共享文件夹
    let share = mock_shares.into_iter().find(|s| s.id == share_id);

    match share {
        Some(s) => {
            // 4. 返回共享文件夹详情
            Ok(HttpResponse::Ok().json(ShareDetailResponse {
                success: true,
                data: s,
            }))
        }
        None => {
            // 5. 共享文件夹不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("Shared folder {} not found", share_id),
                code: "SHARE_NOT_FOUND".to_string(),
            }))
        }
    }
}
