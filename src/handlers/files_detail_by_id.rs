// Phase 107: 文件详情 API
// GET /api/v1/files/{id} — 获取文件详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 文件详情
#[derive(Serialize, Clone)]
pub struct FileDetail {
    pub file_id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub created_at: u64,
    pub modified_at: u64,
    pub owner_id: u64,
    pub is_shared: bool,
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

/// 文件详情（Phase 107）
/// - JWT 认证，登录用户可访问
/// - 路径参数：id (文件 ID)
/// - 返回文件详细信息
/// - 文件不存在返回 404
pub async fn get_file_detail(
    req: HttpRequest,
    path: web::Path<String>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let file_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 模拟文件数据
    let mock_files = vec![
        (
            "file_1",
            "document.pdf",
            "/Documents/document.pdf",
            524288,
            "application/pdf",
            1,
            "System Volume",
            1711500000,
            1711500000,
            1,
            false,
        ),
        (
            "file_2",
            "photo.jpg",
            "/Pictures/photo.jpg",
            2097152,
            "image/jpeg",
            1,
            "System Volume",
            1711400000,
            1711400000,
            1,
            true,
        ),
        (
            "file_3",
            "video.mp4",
            "/Videos/video.mp4",
            104857600,
            "video/mp4",
            1,
            "System Volume",
            1711300000,
            1711300000,
            1,
            false,
        ),
    ];

    // 4. 查找文件
    let file = mock_files.iter().find(|(id, _, _, _, _, _, _, _, _, _, _)| *id == file_id);

    match file {
        Some((
            id,
            name,
            path,
            size,
            mime_type,
            volume_id,
            volume_name,
            created_at,
            modified_at,
            owner_id,
            is_shared,
        )) => {
            Ok(HttpResponse::Ok().json(FileDetailResponse {
                success: true,
                data: FileDetail {
                    file_id: id.to_string(),
                    name: name.to_string(),
                    path: path.to_string(),
                    size_bytes: *size,
                    mime_type: mime_type.to_string(),
                    volume_id: *volume_id,
                    volume_name: volume_name.to_string(),
                    created_at: *created_at,
                    modified_at: *modified_at,
                    owner_id: *owner_id,
                    is_shared: *is_shared,
                },
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("File '{}' not found", file_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
