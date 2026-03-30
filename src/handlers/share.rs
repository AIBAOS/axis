// 共享链接处理器
// 包含：生成共享链接接口

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;

use crate::services::jwt_service::JwtService;

/// 共享链接请求
#[derive(Deserialize)]
pub struct ShareRequest {
    pub file_id: String,
    pub expires_days: u32,  // 1/7/30/0(永久)
}

/// 共享链接响应
#[derive(Serialize)]
pub struct ShareResponse {
    pub share_token: String,
    pub expires_at: Option<String>,
}

/// 生成共享链接
/// 需要登录用户访问（共享文件需要认证）
pub async fn create_share(
    http_req: HttpRequest,
    _req: web::Json<ShareRequest>,
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // JWT 认证
    let token = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Authentication required"
        }));
    }

    let _claims = match jwt_service.validate_token(&token.expect("Token should exist")) {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "success": false,
                "message": "Invalid token"
            }));
        }
    };

    // 生成唯一 share_token
    let share_token = Uuid::new_v4().to_string();
    
    let share_req = _req.into_inner();
    let expires_at = if share_req.expires_days > 0 {
        let expiry = SystemTime::now()
            + Duration::from_secs(share_req.expires_days as u64 * 86400);
        Some(
            expiry
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string()
        )
    } else {
        None
    };

    let response = ShareResponse {
        share_token,
        expires_at,
    };

    HttpResponse::Ok().json(response)
}
