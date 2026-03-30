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

/// Session 服务
pub struct SessionService {
    sessions: Arc<Mutex<SqliteSessionRepository>>,
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
        }
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

    /// 获取会话
    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        let repo = match self.sessions.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("SessionService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };
        repo.get_session(session_id).ok().flatten()
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
}

impl Default for SessionService {
    fn default() -> Self {
        panic!("SessionService requires database connection");
    }
}
