use actix_web::{web, HttpResponse, Error, Result};
use serde::{Deserialize, Serialize};
use crate::models::session::Session;
use crate::services::session_service::SessionService;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentSessionResponse {
    pub session_id: String,
    pub user_id: u64,
    pub created_at: u64,
    pub ip: String,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSessionsResponse {
    pub sessions: Vec<Session>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSessionResponse {
    pub deleted: bool,
}

// 获取当前会话
pub async fn get_current_session(
    session_service: web::Data<Arc<SessionService>>,
) -> Result<HttpResponse, Error> {
    // 从 Header 中获取 session_id（实际应从 JWT 认证）
    let session = session_service.get_session("demo_session_id").unwrap_or_else(|| {
        Session::new(
            "demo_session_id".to_string(),
            1,
            "127.0.0.1".to_string(),
            "demo-agent".to_string(),
        )
    });
    HttpResponse::Ok().json(CurrentSessionResponse {
        session_id: session.session_id,
        user_id: session.user_id,
        created_at: session.created_at,
        ip: session.ip,
        user_agent: session.user_agent,
    })
}

// 列出会话
pub async fn list_sessions(
    session_service: web::Data<Arc<SessionService>>,
) -> Result<HttpResponse, Error> {
    let sessions = session_service.list_sessions();
    HttpResponse::Ok().json(ListSessionsResponse { sessions })
}

// 删除指定会话
pub async fn delete_session(
    session_service: web::Data<Arc<SessionService>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let session_id = path.into_inner();
    let deleted = session_service.delete_session(&session_id);
    HttpResponse::Ok().json(DeleteSessionResponse { deleted })
}
