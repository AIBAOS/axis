// Phase 58 - 删除打印机 API
// DELETE /api/v1/printers/{id} — 删除打印机

use actix_web::{web, HttpResponse, Error};
use serde::Serialize;

use crate::models::jwt::JwtClaims;

/// 删除打印机响应
#[derive(Serialize)]
pub struct DeletePrinterResponse {
    pub success: bool,
    pub message: String,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
}

/// 检查当前用户是否为管理员
fn is_admin(claims: &JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 删除打印机（Phase 58）
/// - JWT 认证，仅 admin 角色可访问
/// - 验证打印机 ID 存在
/// - 删除成功返回 204 No Content
pub async fn delete_printer(
    jwt_claims: web::Data<JwtClaims>,
    path: web::Path<u64>,
) -> Result<HttpResponse, Error> {
    // 1. JWT 认证：检查当前用户是否为 admin
    if !is_admin(jwt_claims.get_ref()) {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            message: "Only admin users can delete printers".to_string(),
        }));
    }

    let printer_id = path.into_inner();

    // 2. 模拟数据库查询（验证打印机是否存在）
    // 实际实现应连接数据库查询
    let mock_printers = vec![1, 2, 3, 4, 5];

    if !mock_printers.contains(&printer_id) {
        return Ok(HttpResponse::NotFound().json(ErrorResponse {
            success: false,
            message: format!("Printer {} not found", printer_id),
        }));
    }

    // 3. 模拟删除操作
    // 实际实现应调用数据库 delete 方法

    log::info!("Printer {} deleted by admin", printer_id);

    // 4. 返回 204 No Content
    Ok(HttpResponse::NoContent().finish())
}
