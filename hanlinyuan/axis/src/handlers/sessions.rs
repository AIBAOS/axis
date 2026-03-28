// 会话管理处理器
// 包含：获取当前会话、列出会话、删除会话

use actix_web::{web, HttpResponse, Responder, HttpMessage};
use serde::{Deserialize, Serialize};

use crate::services::session_service::SessionService;

/// 会话响应
#[derive(Serialize, Deserialize)]
pub struct SessionResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<SessionData>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub id: String,
    pub user_id: u64,
    pub username: String,
    pub created_at: u64,
    pub last_activity: u64,
}

impl From<crate::services::session_service::Session> for SessionData {
    fn from(session: crate::services::session_service::Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            username: session.username,
            created_at: session.created_at,
            last_activity: session.last_activity,
        }
    }
}

/// 获取当前会话
pub async fn get_current_session(
    session_service: web::Data<SessionService>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // 从请求扩展中获取 user_id（由 JWT 中间件注入）
    let user_id_opt = req.extensions()
        .get::<crate::models::jwt::JwtClaims>()
        .map(|c| c.sub.parse::<u64>().ok())
        .flatten();

    if let Some(user_id) = user_id_opt {
        let sessions = session_service.get_sessions_by_user(user_id);
        if sessions.is_empty() {
            let response = SessionResponse {
                success: false,
                message: "未找到会话".to_string(),
                data: None,
            };
            return HttpResponse::NotFound().json(response);
        }

        let session = &sessions[0];
        let session_data = SessionData::from(session.clone());
        let response = SessionResponse {
            success: true,
            message: "会话查询成功".to_string(),
            data: Some(session_data),
        };
        HttpResponse::Ok().json(response)
    } else {
        let response = SessionResponse {
            success: false,
            message: "未认证".to_string(),
            data: None,
        };
        HttpResponse::Unauthorized().json(response)
    }
}

/// 列出会话
pub async fn list_sessions(
    session_service: web::Data<SessionService>,
) -> impl Responder {
    let _sessions = session_service.list_sessions();
    
    let response = SessionResponse {
        success: true,
        message: "会话列表查询成功".to_string(),
        data: None,
    };
    HttpResponse::Ok().json(response)
}

/// 删除会话
pub async fn delete_session(
    session_service: web::Data<SessionService>,
    session_id: web::Path<String>,
) -> impl Responder {
    let deleted = session_service.delete_session(&session_id.into_inner());

    if deleted {
        let response = SessionResponse {
            success: true,
            message: "会话已删除".to_string(),
            data: None,
        };
        HttpResponse::Ok().json(response)
    } else {
        let response = SessionResponse {
            success: false,
            message: "会话不存在".to_string(),
            data: None,
        };
        HttpResponse::NotFound().json(response)
    }
}
