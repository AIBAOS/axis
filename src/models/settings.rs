// System settings and user models
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSettings {
    pub max_file_size_mb: u64,
    pub max_upload_workers: u32,
    pub enable_quota: bool,
    pub default_quota_mb: u64,
    pub storage_path: String,
}

impl Default for SystemSettings {
    fn default() -> Self {
        Self {
            max_file_size_mb: 100,
            max_upload_workers: 4,
            enable_quota: false,
            default_quota_mb: 1024,
            storage_path: "/data/uploads".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub email: String,
    pub quota_mb: u64,
    pub used_mb: u64,
    pub created_at: u64,
}
