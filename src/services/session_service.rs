use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::models::session::Session;

pub struct SessionService {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionService {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_session(
        &self,
        session_id: String,
        user_id: u64,
        ip: String,
        user_agent: String,
    ) -> Session {
        let session = Session::new(session_id, user_id, ip, user_agent);
        let mut store = self.sessions.lock().unwrap();
        store.insert(session.session_id.clone(), session.clone());
        session
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        let store = self.sessions.lock().unwrap();
        store.get(session_id).cloned()
    }

    pub fn list_sessions(&self) -> Vec<Session> {
        let store = self.sessions.lock().unwrap();
        store.values().cloned().collect()
    }

    pub fn delete_session(&self, session_id: &str) -> bool {
        let mut store = self.sessions.lock().unwrap();
        store.remove(session_id).is_some()
    }
}
