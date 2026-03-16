// User model (shared with settings)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub email: String,
    pub quota_mb: u64,
    pub used_mb: u64,
    pub created_at: u64,
}
