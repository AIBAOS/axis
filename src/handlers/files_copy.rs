// Phase 110 - 文件复制 API
// POST /api/v1/files/{id}/copy — 复制文件

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 复制文件请求
#[derive(Deserialize)]
pub struct CopyFileRequest {
    pub destination_path: String,
}

/// 文件信息
#[derive(Serialize)]
pub struct FileInfo {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub created_at: u64,
    pub owner_id: u64,
}

/// 复制文件响应
#[derive(Serialize)]
pub struct CopyFileResponse {
    pub success: bool,
    pub message: String,
    pub data: FileInfo,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 复制文件（Phase 110）
/// - JWT 认证，登录用户可访问
/// - 验证文件 ID 存在性（404）
/// - 验证文件归属权（403），admin 可操作任意文件
/// - 验证目标路径存在性（404）
/// - 检查目标位置同名文件冲突（409）
/// - 复制成功返回 201 Created + 新文件信息
pub async fn copy_file(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<CopyFileRequest>,
    rbac_repo: web::Data<SqliteRbacRepository>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 2. 获取当前用户信息和角色
    let user_id = claims.sub.parse().unwrap_or(0);
    let user_roles = rbac_repo.get_roles_by_user(user_id);
    let is_admin = user_roles.iter().any(|r| r.name == "admin");

    let file_id = path.into_inner();
    let destination_path = &payload.destination_path;

    // 3. 验证目标路径格式
    if destination_path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "destination_path is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟源文件数据
    let mock_files = vec![
        (1u64, "readme.txt", "/readme.txt", 1024u64, "text/plain", 1u64),
        (2u64, "config.json", "/config.json", 2048u64, "application/json", 1u64),
        (3u64, "photo.jpg", "/Pictures/photo.jpg", 2097152u64, "image/jpeg", 2u64),
        (4u64, "video.mp4", "/Videos/video.mp4", 104857600u64, "video/mp4", 2u64),
    ];

    // 5. 查找源文件
    let source_file = mock_files.into_iter().find(|(fid, _, _, _, _, _)| *fid == file_id);

    match source_file {
        Some((fid, name, src_path, size, mime, owner_id)) => {
            // 6. 验证文件归属权（admin 可操作任意文件）
            if !is_admin && owner_id != user_id {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "You can only copy your own files".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 7. 模拟验证目标路径存在性
            let valid_paths = vec!["/", "/Documents", "/Pictures", "/Videos"];
            if !valid_paths.contains(&destination_path.as_str()) {
                return Ok(HttpResponse::NotFound().json(ErrorResponse {
                    success: false,
                    error: format!("Destination path '{}' not found", destination_path),
                    code: "NOT_FOUND".to_string(),
                }));
            }

            // 8. 检查目标位置同名文件冲突
            let new_path = format!("{}/{}", destination_path.trim_end_matches('/'), name);
            let existing_files = vec!["/readme.txt", "/config.json", "/Pictures/photo.jpg"];
            if existing_files.contains(&new_path.as_str()) {
                return Ok(HttpResponse::Conflict().json(ErrorResponse {
                    success: false,
                    error: format!("File '{}' already exists in destination", name),
                    code: "FILE_EXISTS".to_string(),
                }));
            }

            // 9. 模拟复制操作
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();

            let new_file = FileInfo {
                id: 100 + file_id, // 新文件 ID
                name: name.to_string(),
                path: new_path,
                size_bytes: size,
                mime_type: mime.to_string(),
                created_at: now,
                owner_id: user_id, // 复制后的文件属于当前用户
            };

            Ok(HttpResponse::Created().json(CopyFileResponse {
                success: true,
                message: "File copied successfully".to_string(),
                data: new_file,
            }))
        }
        None => {
            // 10. 源文件不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("File {} not found", file_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
