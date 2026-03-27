// Phase 108 - 文件更新/重命名 API
// PUT /api/v1/files/{id} — 更新文件（重命名/移动）

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::database::rbac_store::SqliteRbacRepository;
use crate::models::rbac::RbacRepository;
use crate::services::jwt_service::JwtService;

/// 更新文件请求
#[derive(Deserialize)]
pub struct UpdateFileRequest {
    pub name: Option<String>,
    pub path: Option<String>,
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
    pub updated_at: u64,
    pub owner_id: u64,
}

/// 更新文件响应
#[derive(Serialize)]
pub struct UpdateFileResponse {
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

/// 更新文件（Phase 108）
/// - JWT 认证，登录用户可访问
/// - 验证文件 ID 存在性（404）
/// - 验证文件归属权（403），admin 可操作任意文件
/// - 支持重命名和移动
/// - 至少提供一个字段（name 或 path）
pub async fn update_file(
    req: HttpRequest,
    path: web::Path<u64>,
    payload: web::Json<UpdateFileRequest>,
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

    // 3. 验证至少提供一个字段
    if payload.name.is_none() && payload.path.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one field (name or path) must be provided".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 4. 模拟文件数据
    let mut mock_files = vec![
        (1u64, "readme.txt", "/readme.txt", 1024u64, "text/plain", 1u64),
        (2u64, "config.json", "/config.json", 2048u64, "application/json", 1u64),
        (3u64, "photo.jpg", "/Pictures/photo.jpg", 2097152u64, "image/jpeg", 2u64),
        (4u64, "video.mp4", "/Videos/video.mp4", 104857600u64, "video/mp4", 2u64),
    ];

    // 5. 查找文件
    let file_index = mock_files.iter().position(|(fid, _, _, _, _, _)| *fid == file_id);

    match file_index {
        Some(idx) => {
            let (fid, name, path, size, mime, owner_id) = mock_files[idx];

            // 6. 验证文件归属权（admin 可操作任意文件）
            if !is_admin && owner_id != user_id {
                return Ok(HttpResponse::Forbidden().json(ErrorResponse {
                    success: false,
                    error: "You can only modify your own files".to_string(),
                    code: "FORBIDDEN".to_string(),
                }));
            }

            // 7. 更新文件信息
            let mut new_name = name.to_string();
            let mut new_path = path.to_string();

            if let Some(ref new_n) = payload.name {
                // 验证同一目录下不重名
                let dir = new_path.rsplit_once('/').map(|(d, _)| d).unwrap_or("/");
                let name_exists = mock_files.iter().any(|(_, n, p, _, _, oid)| {
                    *oid == owner_id && p.starts_with(dir) && n == new_n
                });
                if name_exists {
                    return Ok(HttpResponse::Conflict().json(ErrorResponse {
                        success: false,
                        error: format!("File '{}' already exists in this directory", new_n),
                        code: "FILE_EXISTS".to_string(),
                    }));
                }
                new_name = new_n.clone();
            }

            if let Some(ref new_p) = payload.path {
                new_path = new_p.clone();
            }

            // 8. 模拟更新时间
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Invalid time"))?
                .as_secs();

            let file_info = FileInfo {
                id: fid,
                name: new_name,
                path: new_path,
                size_bytes: size,
                mime_type: mime.to_string(),
                created_at: 1710489600,
                updated_at: now,
                owner_id,
            };

            Ok(HttpResponse::Ok().json(UpdateFileResponse {
                success: true,
                message: "File updated successfully".to_string(),
                data: file_info,
            }))
        }
        None => {
            // 9. 文件不存在
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("File {} not found", file_id),
                code: "NOT_FOUND".to_string(),
            }))
        }
    }
}
