// 会话管理处理器
// 包含：获取当前会话、列出会话、删除会话

use actix_web::{web, HttpResponse, Responder, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::services::session_service::SessionService;
use crate::services::jwt_service::JwtService;

/// 会话响应
#[derive(Serialize, Deserialize)]
pub struct SessionResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<SessionData>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionListResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<SessionData>>,
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

/// 检查是否为管理员
fn is_admin(claims: &crate::models::jwt::JwtClaims) -> bool {
    claims.roles.iter().any(|r| r.to_lowercase() == "admin")
}

/// 获取当前会话
pub async fn get_current_session(
    session_service: web::Data<SessionService>,
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // Bug #73 修复：添加 JWT 认证
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        let response = SessionResponse {
            success: false,
            message: "未认证".to_string(),
            data: None,
        };
        return HttpResponse::Unauthorized().json(response);
    }

    let claims = match jwt_service.validate_token(token.unwrap()) {
        Ok(c) => c,
        Err(_) => {
            let response = SessionResponse {
                success: false,
                message: "无效令牌".to_string(),
                data: None,
            };
            return HttpResponse::Unauthorized().json(response);
        }
    };

    let user_id = claims.user_id;
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
}

/// 列出会话
pub async fn list_sessions(
    session_service: web::Data<SessionService>,
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // Bug #73 修复：添加管理员权限检查
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        let response = SessionListResponse {
            success: false,
            message: "未认证".to_string(),
            data: None,
        };
        return HttpResponse::Unauthorized().json(response);
    }

    let claims = match jwt_service.validate_token(token.unwrap()) {
        Ok(c) => c,
        Err(_) => {
            let response = SessionListResponse {
                success: false,
                message: "无效令牌".to_string(),
                data: None,
            };
            return HttpResponse::Unauthorized().json(response);
        }
    };

    // 仅管理员可列出所有会话
    if !is_admin(&claims) {
        let response = SessionListResponse {
            success: false,
            message: "权限不足".to_string(),
            data: None,
        };
        return HttpResponse::Forbidden().json(response);
    }

    let sessions = session_service.list_sessions();
    let session_data: Vec<SessionData> = sessions.into_iter().map(SessionData::from).collect();
    
    let response = SessionListResponse {
        success: true,
        message: "会话列表查询成功".to_string(),
        data: Some(session_data),
    };
    HttpResponse::Ok().json(response)
}

/// 删除会话
pub async fn delete_session(
    session_service: web::Data<SessionService>,
    session_id: web::Path<String>,
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> impl Responder {
    // Bug #73 修复：添加权限检查
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "));

    if token.is_none() {
        let response = SessionResponse {
            success: false,
            message: "未认证".to_string(),
            data: None,
        };
        return HttpResponse::Unauthorized().json(response);
    }

    let claims = match jwt_service.validate_token(token.unwrap()) {
        Ok(c) => c,
        Err(_) => {
            let response = SessionResponse {
                success: false,
                message: "无效令牌".to_string(),
                data: None,
            };
            return HttpResponse::Unauthorized().json(response);
        }
    };

    // 权限检查：管理员可删除任何会话，普通用户只能删除自己的会话
    let target_session_id = session_id.into_inner();
    
    // 获取目标会话信息
    let target_session = session_service.get_session(&target_session_id);
    
    if let Some(session) = target_session {
        let is_own_session = session.user_id == claims.user_id;
        let is_admin_user = is_admin(&claims);
        
        if !is_own_session && !is_admin_user {
            let response = SessionResponse {
                success: false,
                message: "无权删除此会话".to_string(),
                data: None,
            };
            return HttpResponse::Forbidden().json(response);
        }
        
        let deleted = session_service.delete_session(&target_session_id);

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
                message: "会话删除失败".to_string(),
                data: None,
            };
            HttpResponse::InternalServerError().json(response)
        }
    } else {
        let response = SessionResponse {
            success: false,
            message: "会话不存在".to_string(),
            data: None,
        };
        HttpResponse::NotFound().json(response)
    }
}