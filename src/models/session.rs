use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub session_id: String,
    pub user_id: u64,
    pub created_at: u64,
    pub ip: String,
    pub user_agent: String,
}

impl Session {
    pub fn new(session_id: String, user_id: u64, ip: String, user_agent: String) -> Self {
        Self {
            session_id,
            user_id,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ip,
            user_agent,
        }
    }
}
