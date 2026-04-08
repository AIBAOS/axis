// Phase 94 - 删除共享文件夹 API
// DELETE /api/v1/shared-folders/{id} — 删除共享文件夹

use actix_web::{web, HttpResponse, Error};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 删除共享文件夹响应
#[derive(Serialize)]
pub struct DeleteSharedFolderResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<DeletedFolderData>,
}

/// 已删除的共享文件夹数据
#[derive(Serialize)]
pub struct DeletedFolderData {
    pub id: u64,
    pub name: String,
    pub deleted_at: u64,
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
    claims.is_admin()
}

/// 删除共享文件夹（Phase 94）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证共享文件夹 ID 存在
/// - 返回删除结果
pub async fn delete_shared_folder(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete shared folders".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    let folder_id = path.into_inner();

    // 2. 模拟共享文件夹数据
    let mock_folders = vec![
        (1, "public".to_string()),
        (2, "homes".to_string()),
        (3, "media".to_string()),
    ];

    let folder = mock_folders.iter().find(|(id, _)| *id == folder_id);

    // 3. 验证共享文件夹存在
    if folder.is_none() {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            error: format!("Shared folder {} not found", folder_id),
            code: "NOT_FOUND".to_string(),
        }));
    }

    let (_, folder_name) = .expect("Folder should exist");

    // 4. 获取当前时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| Error::from(actix_web::error::ErrorInternalServerError("Invalid time")))?
        .as_secs();

    // 5. 执行删除（模拟）
    // 实际应调用数据库删除操作

    // 6. 返回删除结果
    Ok(HttpResponse::Ok().json(DeleteSharedFolderResponse {
        success: true,
        message: "Shared folder deleted successfully".to_string(),
        data: Some(DeletedFolderData {
            id: folder_id,
            name: folder_name.clone(),
            deleted_at: now,
        }),
    }))
}
