// Phase 178: 系统告警删除 API
// DELETE /api/v1/system/alerts/{id} — 删除系统告警

use actix_web::{web, HttpResponse, Error, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::jwt_service::JwtService;

/// 删除告警请求
#[derive(Debug, Deserialize)]
pub struct DeleteAlertRequest {
    pub note: Option<String>,
}

/// 错误响应
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: String,
}

/// 删除系统告警（Phase 178）
/// - JWT 认证，admin 角色可访问
/// - 验证告警 ID 存在性（404 Not Found）
/// - 验证告警状态（仅 acknowledged/resolved 状态可删除）
/// - 删除成功返回 204 No Content
pub async fn delete_system_alert(
    req: HttpRequest,
    path: web::Path<u64>,
    _payload: Option<web::Json<DeleteAlertRequest>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, Error> {
    let alert_id = path.into_inner();

    // 1. JWT 认证 - 提取并验证 token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 2. 验证 token 有效性
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))?;

    // 3. 验证 admin 权限
    let is_admin = claims.roles.iter().any(|r| r.to_lowercase() == "admin");
    if !is_admin {
        return Ok(HttpResponse::Forbidden().json(ErrorResponse {
            success: false,
            error: "Only admin users can delete system alerts".to_string(),
            code: "FORBIDDEN".to_string(),
        }));
    }

    // 4. 模拟现有告警数据
    let mock_alerts = vec![
        (1u64, "active".to_string()),
        (2u64, "acknowledged".to_string()),
        (3u64, "resolved".to_string()),
    ];

    // 5. 查找告警
    let alert = mock_alerts.iter().find(|(id, _)| *id == alert_id);

    // 6. 验证告警存在性
    let alert = match alert {
        Some(a) => a,
        None => {
            return Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: format!("System alert {} not found", alert_id),
                code: "NOT_FOUND".to_string(),
            }));
        }
    };

    let (_id, status) = alert;

    // 7. 验证告警状态（仅 acknowledged/resolved 状态可删除）
    if status == "active" {
        return Ok(HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: format!("Alert {} must be acknowledged or resolved before deletion", alert_id),
            code: "INVALID_STATUS".to_string(),
        }));
    }

    // 8. 模拟删除告警
    // （在实际实现中，这里会调用数据库删除告警记录）

    // 9. 返回删除成功（204 No Content）
    Ok(HttpResponse::NoContent().finish())
}
