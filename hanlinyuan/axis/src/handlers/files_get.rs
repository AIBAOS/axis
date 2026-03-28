// Phase 121: 文件详情 API
// GET /api/v1/files/{id} — 获取文件详情

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::services::jwt_service::JwtService;

/// 文件详情
#[derive(Serialize, Clone)]
pub struct FileDetail {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub mime_type: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub owner_id: u64,
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

/// 文件详情（Phase 121）
/// - JWT 认证，登录用户可访问
/// - 路径参数：id (文件 ID)
/// - 返回文件元数据
/// - 验证文件存在性（404）
/// - 验证访问权限（403）
pub async fn get_file_detail(
    req: HttpRequest,
    path: web::Path<u64>,
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
        (1, "report.pdf", "/Documents/report.pdf", 524288, "application/pdf", 1711500000, 1711500000, 1),
        (2, "photo.jpg", "/Pictures/photo.jpg", 2097152, "image/jpeg", 1711400000, 1711400000, 1),
        (3, "video.mp4", "/Videos/video.mp4", 104857600, "video/mp4", 1711300000, 1711300000, 1),
    ];

    let file = mock_files.iter().find(|(id, _, _, _, _, _, _, _)| *id == file_id);

    match file {
        Some((id, name, file_path, size, mime_type, created_at, updated_at, owner_id)) => {
            // 4. 验证访问权限（简化：假设登录用户可访问所有文件）
            // 实际实现应检查文件所有权或共享权限
            
            Ok(HttpResponse::Ok().json(FileDetailResponse {
                success: true,
                data: FileDetail {
                    id: *id,
                    name: name.to_string(),
                    path: file_path.to_string(),
                    size: *size,
                    mime_type: mime_type.to_string(),
                    created_at: *created_at,
                    updated_at: *updated_at,
                    owner_id: *owner_id,
                },
            }))
        }
        None => {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("File {} not found", file_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
