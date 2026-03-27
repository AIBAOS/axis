use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaMetadata {
    pub resolution: String,
    pub duration: u64,      // seconds
    pub bitrate: u32,       // kbps
    pub codec: String,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaFile {
    pub id: u64,
    pub file_path: String,
    pub file_name: String,
    pub file_type: String,
    pub size: u64,
    pub metadata: MediaMetadata,
    pub created_at: String,
}
