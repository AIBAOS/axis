use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileOperation {
    CREATE,
    READ,
    UPDATE,
    DELETE,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileAuditLog {
    pub id: u64,
    pub user_id: u64,
    pub operation: FileOperation,
    pub file_path: String,
    pub timestamp: String,
    pub ip_address: String,
    pub details: Option<String>,
}
