// Phase 106 - 文件删除 API
// DELETE /api/v1/files/{id} — 删除文件

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::Serialize;

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 删除文件响应
#[derive(Serialize)]
pub struct FileDeleteResponse {
    pub success: bool,
    pub message: String,
    pub deleted_file: DeletedFileInfo,
}

/// 已删除文件信息
#[derive(Serialize)]
pub struct DeletedFileInfo {
    pub file_id: u64,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除文件（Phase 106）
/// - JWT 认证，登录用户可访问
/// - 验证文件 ID 存在性（404）
/// - 验证文件归属权（403），admin 可删除任意文件
/// - 删除成功返回 200 OK
pub async fn delete_file(
    req: HttpRequest,
    path: web::Path<u64>,
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

    // 3. 模拟文件数据
    let mock_files = vec![
        (1u64, "readme.txt", "/readme.txt", 1024u64, 1u64), // file_id, name, path, size, owner_id
        (2u64, "config.json", "/config.json", 2048u64, 1u64),
        (3u64, "photo.jpg", "/Pictures/photo.jpg", 2097152u64, 2u64),
        (4u64, "video.mp4", "/Videos/video.mp4", 104857600u64, 2u64),
    ];

    // 4. 查找文件
    let file = mock_files.into_iter().find(|(fid, _, _, _, _)| *fid == file_id);

    match file {
        Some((fid, name, path, size, owner_id)) => {
            // 5. 验证文件归属权（admin 可删除任意文件）
            if !is_admin && owner_id != user_id {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "You can only delete your own files".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 6. 模拟删除操作
            let deleted_file = DeletedFileInfo {
                file_id: fid,
                name: name.to_string(),
                path: path.to_string(),
                size_bytes: size,
            };

            Ok(HttpResponse::Ok().json(FileDeleteResponse {
                success: true,
                message: "File deleted successfully".to_string(),
                deleted_file,
            }))
        }
        None => {
            // 7. 文件不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("File {} not found", file_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
