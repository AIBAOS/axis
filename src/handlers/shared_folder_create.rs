// Phase 89 - 创建共享文件夹 API
// POST /api/v1/shared-folders — 创建共享文件夹

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 创建共享文件夹请求
#[derive(Deserialize)]
pub struct CreateSharedFolderRequest {
    pub name: String,
    pub path: String,
    pub volume_id: u64,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub is_public: Option<bool>,
}

/// 创建共享文件夹响应
#[derive(Serialize)]
pub struct CreateSharedFolderResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<SharedFolderData>,
}

/// 共享文件夹数据
#[derive(Serialize)]
pub struct SharedFolderData {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub volume_id: u64,
    pub volume_name: String,
    pub description: Option<String>,
    pub protocols: Vec<String>,
    pub is_public: bool,
    pub status: String,
    pub created_at: u64,
    pub created_by: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r == "admin")
}

/// 验证协议类型
fn validate_protocol(protocol: &str) -> bool {
    let valid_protocols = ["smb", "nfs", "afp", "ftp"];
    valid_protocols.contains(&protocol.to_lowercase().as_str())
}

/// 创建共享文件夹（Phase 89）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证存储卷 ID 存在
/// - 检查共享文件夹名称唯一性
/// - 验证协议类型有效性
/// - 返回创建的共享文件夹信息
pub async fn create_shared_folder(
    jwt_claims: web::Data<JwtClaims>,
    req: web::Json<CreateSharedFolderRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can create shared folders".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 2. 验证必要参数
    if req.name.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "name is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if req.path.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "path is required".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    if req.protocols.is_empty() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one protocol must be specified".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 验证协议类型
    for protocol in &req.protocols {
        if !validate_protocol(protocol) {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: format!("Invalid protocol '{}'. Valid protocols: smb, nfs, afp, ftp", protocol),
                code: "INVALID_PROTOCOL".to_string(),
            }));
        }
    }

    // 4. 模拟存储卷数据（验证存在性）
    let mock_volumes = vec![
        (1, "root".to_string()),
        (2, "data".to_string()),
        (3, "backup".to_string()),
    ];

    let volume = mock_volumes.iter().find(|(id, _)| *id == req.volume_id);

    // 5. 验证存储卷存在
    if volume.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Storage volume {} not found", req.volume_id),
            code: "VOLUME_NOT_FOUND".to_string(),
        }));
    }

    let (_, volume_name) = .expect("Volume should exist");

    // 6. 检查共享文件夹名称唯一性（模拟）
    let existing_names = vec!["public".to_string(), "homes".to_string(), "media".to_string()];
    if existing_names.contains(&req.name) {
        return Ok(HttpResponse::Conflict().json(ErrorResponse {
            success: false,
            error: format!("Shared folder name '{}' already exists", req.name),
            code: "SHARED_FOLDER_EXISTS".to_string(),
        }));
    }

    // 7. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 8. 创建共享文件夹（模拟）
    let new_shared_folder = SharedFolderData {
        id: 100 + req.volume_id,
        name: req.name.clone(),
        path: req.path.clone(),
        volume_id: req.volume_id,
        volume_name: volume_name.clone(),
        description: req.description.clone(),
        protocols: req.protocols.clone(),
        is_public: req.is_public.unwrap_or(false),
        status: "active".to_string(),
        created_at: now,
        created_by: "admin".to_string(),
    };

    // 9. 返回创建结果
    Ok(HttpResponse::Created().json(CreateSharedFolderResponse {
        success: true,
        message: "Shared folder created successfully".to_string(),
        data: Some(new_shared_folder),
    }))
}
