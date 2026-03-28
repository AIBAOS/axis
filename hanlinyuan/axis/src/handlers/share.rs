// 共享链接处理器
// 包含：生成共享链接接口

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{Duration, SystemTime};
use std::time::UNIX_EPOCH;

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
pub async fn create_share(
    _req: web::Json<ShareRequest>,
) -> impl Responder {
    // 生成唯一 share_token
    let share_token = Uuid::new_v4().to_string();
    
    let share_req = _req.into_inner();
    let expires_at = if share_req.expires_days > 0 {
        let expiry = SystemTime::now()
            + Duration::from_secs(share_req.expires_days as u64 * 86400);
        Some(
            expiry
                .duration_since(UNIX_EPOCH)
                .unwrap()
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
