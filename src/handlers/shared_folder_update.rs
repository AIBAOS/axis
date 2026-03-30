// Phase 93 - 更新共享文件夹 API
// PUT /api/v1/shared-folders/{id} — 更新共享文件夹

use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// 更新共享文件夹请求
#[derive(Deserialize)]
pub struct UpdateSharedFolderRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub protocols: Option<Vec<String>>,
    pub read_only: Option<bool>,
    pub guest_access: Option<bool>,
    pub enabled: Option<bool>,
}

/// 更新共享文件夹响应
#[derive(Serialize)]
pub struct UpdateSharedFolderResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<SharedFolderData>,
}

/// 共享文件夹数据
#[derive(Serialize, Clone)]
pub struct SharedFolderData {
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

/// 更新共享文件夹（Phase 93）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享文件夹 ID 存在
/// - 检查名称唯一性（排除自身）
/// - 验证协议类型有效性
/// - 返回更新后的共享文件夹信息
pub async fn update_shared_folder(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
    req: web::Json<UpdateSharedFolderRequest>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can update shared folders".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let folder_id = path.into_inner();

    // 2. 验证必要参数（至少一个字段）
    if req.name.is_none() && req.description.is_none() && req.protocols.is_none() &&
       req.read_only.is_none() && req.guest_access.is_none() && req.enabled.is_none() {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "At least one field must be provided".to_string(),
            code: "INVALID_PARAMS".to_string(),
        }));
    }

    // 3. 验证协议类型（如果提供）
    if let Some(ref protocols) = req.protocols {
        if protocols.is_empty() {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "At least one protocol must be specified".to_string(),
                code: "INVALID_PROTOCOL".to_string(),
            }));
        }
        for protocol in protocols {
            if !validate_protocol(protocol) {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    success: false,
                    error: format!("Invalid protocol '{}'. Valid protocols: smb, nfs, afp, ftp", protocol),
                    code: "INVALID_PROTOCOL".to_string(),
                }));
            }
        }
    }

    // 4. 模拟共享文件夹数据
    let mut mock_folders = vec![
        SharedFolderData {
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
        SharedFolderData {
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
        SharedFolderData {
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

    // 5. 查找共享文件夹
    let folder_idx = mock_folders.iter().position(|f| f.id == folder_id);

    // 6. 验证共享文件夹存在
    if folder_idx.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Shared folder {} not found", folder_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    // 7. 检查名称唯一性（如果更新名称）
    if let Some(ref new_name) = req.name {
        let name_exists = {
            let temp_clone = mock_folders.clone();
            temp_clone.iter().any(|f| f.id != folder_id && f.name == *new_name)
        };
        if name_exists {
            return Ok(HttpResponse::Conflict().json(ErrorResponse {
                success: false,
                error: format!("Shared folder name '{}' already exists", new_name),
                code: "SHARED_FOLDER_EXISTS".to_string(),
            }));
        }
    }

    // 8. 执行更新
    let folder_idx = .expect("Folder index should exist");
    let folder = &mut mock_folders[folder_idx];

    if let Some(ref new_name) = req.name {
        folder.name = new_name.clone();
    }

    // 8. 更新其他字段
    if let Some(ref new_description) = req.description {
        folder.description = Some(new_description.clone());
    }

    if let Some(ref new_protocols) = req.protocols {
        folder.protocols = new_protocols.clone();
    }

    if let Some(new_read_only) = req.read_only {
        folder.read_only = new_read_only;
    }

    if let Some(new_guest_access) = req.guest_access {
        folder.guest_access = new_guest_access;
    }

    if let Some(new_enabled) = req.enabled {
        folder.status = if new_enabled { "active".to_string() } else { "inactive".to_string() };
    }

    // 9. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    folder.updated_at = now;

    // 10. 返回更新结果
    Ok(HttpResponse::Ok().json(UpdateSharedFolderResponse {
        success: true,
        message: "Shared folder updated successfully".to_string(),
        data: Some(folder.clone()),
    }))
}
