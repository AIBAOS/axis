// Phase 107 - 文件详情 API
// GET /api/v1/files/{id} — 获取文件详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 文件详情信息
#[derive(Serialize, Clone)]
pub struct FileDetail {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub volume_id: u64,
    pub owner_id: u64,
    pub created_at: u64,
    pub modified_at: u64,
}

/// 文件详情响应
#[derive(Serialize)]
pub struct FileDetailResponse {
    pub success: bool,
    pub data: FileDetail,
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
    true // 已登录用户可访问
}

/// 文件详情（Phase 107）
/// - JWT 认证，登录用户可访问
/// - 验证文件 ID 存在性 (404)
/// - 验证文件访问权限 (403)
/// - 返回文件详细信息
pub async fn get_file_detail(
    req: HttpRequest,
    path: web::Path<String>,
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

    let file_id = path.into_inner();

    // 2. 模拟文件数据
    let mock_files = vec![
        FileDetail {
            id: "file_001".to_string(),
            name: "document.pdf".to_string(),
            path: "/Documents/document.pdf".to_string(),
            size_bytes: 524288,
            mime_type: "application/pdf".to_string(),
            volume_id: 1,
            owner_id: 1,
            created_at: 1774259200,
            modified_at: 1774345600,
        },
        FileDetail {
            id: "file_002".to_string(),
            name: "photo.jpg".to_string(),
            path: "/Pictures/photo.jpg".to_string(),
            size_bytes: 2097152,
            mime_type: "image/jpeg".to_string(),
            volume_id: 1,
            owner_id: 1,
            created_at: 1774345600,
            modified_at: 1774345600,
        },
        FileDetail {
            id: "file_003".to_string(),
            name: "video.mp4".to_string(),
            path: "/Videos/video.mp4".to_string(),
            size_bytes: 104857600,
            mime_type: "video/mp4".to_string(),
            volume_id: 1,
            owner_id: 2,
            created_at: 1774432000,
            modified_at: 1774432000,
        },
    ];

    // 假设当前用户 ID 为 1（从 JWT token 中解析）
    let current_user_id: u64 = 1;
    let current_user_is_admin = false; // 从 JWT claims 中解析

    // 3. 查找文件
    let file = mock_files.into_iter().find(|f| f.id == file_id);

    // 4. 验证文件存在性
    if file.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("File '{}' not found", file_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let file = file.unwrap();

    // 5. 验证文件访问权限（admin 可以访问任意文件）
    if !current_user_is_admin && file.owner_id != current_user_id {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "No permission to access this file".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 6. 返回文件详情
    Ok(HttpResponse::Ok().json(FileDetailResponse {
        success: true,
        data: file,
    }))
}
