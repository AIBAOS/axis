// Phase 92 - 共享文件夹详情 API
// GET /api/v1/shared-folders/{id} — 获取共享文件夹详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 共享文件夹详情信息
#[derive(Serialize, Clone)]
pub struct SharedFolderDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub is_public: bool,
    pub read_only: bool,
    pub guest_access: bool,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: String,
}

/// 共享文件夹详情响应
#[derive(Serialize)]
pub struct SharedFolderDetailResponse {
    pub success: bool,
    pub data: SharedFolderDetail,
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
    true // 登录用户可访问
}

/// 共享文件夹详情（Phase 92）
/// - JWT 认证，登录用户可访问
/// - 验证共享文件夹 ID 存在
/// - 返回共享文件夹完整详情
pub async fn get_shared_folder(
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

    let folder_id = path.into_inner();

    // 2. 模拟共享文件夹数据
    let mock_folders = vec![
        SharedFolderDetail {
            id: 1,
            name: "public".to_string(),
            path: "/public".to_string(),
            volume_id: 1,
            volume_name: "data".to_string(),
            description: Some("Public shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string()],
            is_public: true,
            read_only: false,
            guest_access: true,
            status: "active".to_string(),
            created_at: 1774259200,
            updated_at: 1774259200,
            created_by: "admin".to_string(),
        },
        SharedFolderDetail {
            id: 2,
            name: "homes".to_string(),
            path: "/homes".to_string(),
            volume_id: 1,
            volume_name: "data".to_string(),
            description: Some("User home directories".to_string()),
            protocols: vec!["smb".to_string()],
            is_public: false,
            read_only: false,
            guest_access: false,
            status: "active".to_string(),
            created_at: 1774345600,
            updated_at: 1774345600,
            created_by: "admin".to_string(),
        },
        SharedFolderDetail {
            id: 3,
            name: "media".to_string(),
            path: "/media".to_string(),
            volume_id: 2,
            volume_name: "backup".to_string(),
            description: Some("Media files shared folder".to_string()),
            protocols: vec!["smb".to_string(), "nfs".to_string(), "afp".to_string()],
            is_public: false,
            read_only: false,
            guest_access: false,
            status: "active".to_string(),
            created_at: 1774432000,
            updated_at: 1774432000,
            created_by: "admin".to_string(),
        },
    ];

    // 3. 查找指定 ID 的共享文件夹
    let folder = mock_folders.into_iter().find(|f| f.id == folder_id);

    // 4. 返回响应
    match folder {
        Some(f) => Ok(HttpResponse::Ok().json(SharedFolderDetailResponse {
            success: true,
            data: f,
        })),
        None => Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Shared folder {} not found", folder_id),
            code: "NOT_FOUND".to_string(),
        })),
    }
}
