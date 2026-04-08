// Session 服务模块
// 包含：会话管理服务

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::database::session_store::SqliteSessionRepository;
use crate::database::pool::DbConnectionType;

/// 会话数据结构
#[derive(Clone, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: u64,
    pub username: String,
    pub created_at: u64,
    pub last_activity: u64,
}

/// SESS-1: 会话超时配置（默认 30 分钟）
const DEFAULT_SESSION_TIMEOUT_SECS: u64 = 30 * 60;

/// Session 服务
pub struct SessionService {
    sessions: Arc<Mutex<SqliteSessionRepository>>,
    session_timeout_secs: u64,
}

impl SessionService {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        let session_repo = SqliteSessionRepository::new(db);
        // 初始化会话表
        if let Err(e) = session_repo.init_table() {
            eprintln!("Failed to init session table: {}", e);
        }
        
        Self {
            sessions: Arc::new(Mutex::new(session_repo)),
            session_timeout_secs: DEFAULT_SESSION_TIMEOUT_SECS,
        }
    }

    /// 设置会话超时时间（秒）
    pub fn set_session_timeout(&mut self, timeout_secs: u64) {
        self.session_timeout_secs = timeout_secs;
    }

    /// 创建新会话
    pub fn create_session(&self, user_id: u64, username: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        let session = Session {
            id: id.clone(),
            user_id,
            username: username.to_string(),
            created_at: now,
            last_activity: now,
        };

        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        if let Err(e) = repo.create_session(&session) {
            eprintln!("Failed to create session: {}", e);
        }
        
        id
    }

    /// 获取会话（SESS-1: 检查超时）
    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        
        match repo.get_session(session_id) {
            Ok(Some(session)) => {
                // SESS-1: 检查会话是否超时
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |d| d.as_secs());
                
                if now - session.last_activity > self.session_timeout_secs {
                    // 会话已超时，删除并返回 None
                    eprintln!("Session {} expired (inactive for {}s)", session_id, now - session.last_activity);
                    let _ = repo.delete_session(session_id);
                    None
                } else {
                    Some(session)
                }
            }
            Ok(None) => None,
            Err(e) => {
                eprintln!("Failed to get session: {}", e);
                None
            }
        }
    }

    /// 更新最后活动时间
    pub fn update_activity(&self, session_id: &str) -> bool {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        repo.update_activity(session_id).unwrap_or(false)
    }

    /// 删除会话
    pub fn delete_session(&self, session_id: &str) -> bool {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        repo.delete_session(session_id).unwrap_or(false)
    }

    /// 列出会话
    pub fn list_sessions(&self) -> Vec<Session> {
        let _repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        // TODO: 从数据库查询所有会话
        Vec::new()
    }

    /// 根据用户ID获取会话
    pub fn get_sessions_by_user(&self, user_id: u64) -> Vec<Session> {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        repo.get_sessions_by_user(user_id).unwrap_or_default()
    }

    /// SESS-1: 清理所有过期会话
    pub fn cleanup_expired_sessions(&self) -> usize {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        
        // 获取所有会话
        match repo.get_all_sessions() {
            Ok(sessions) => {
                let expired: Vec<String> = sessions
                    .iter()
                    .filter(|s| now - s.last_activity > self.session_timeout_secs)
                    .map(|s| s.id.clone())
                    .collect();
                
                let count = expired.len();
                for session_id in &expired {
                    let _ = repo.delete_session(session_id);
                }
                
                if count > 0 {
                    eprintln!("Cleaned up {} expired sessions", count);
                }
                
                count
            }
            Err(_) => 0
        }
    }
}

impl Default for SessionService {
    fn default() -> Self {
        panic!("SessionService requires database connection");
    }
}
