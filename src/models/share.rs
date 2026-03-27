// 网络共享数据模型
// 包含：共享结构体、接口定义

use serde::{Deserialize, Serialize};

/// 网络共享
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Share {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub protocol: String, // "smb" or "nfs"
    pub status: String,   // "active" or "inactive"
    pub created_at: i64,
    pub updated_at: i64,
}

/// 共享创建请求
#[derive(Serialize, Deserialize, Clone)]
pub struct CreateShareRequest {
    pub name: String,
    pub path: String,
    pub protocol: String, // "smb" or "nfs"
}

/// 共享更新请求
#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateShareRequest {
    pub name: Option<String>,
    pub path: Option<String>,
    pub protocol: Option<String>,
    pub status: Option<String>,
}

/// 共享响应
#[derive(Serialize, Deserialize)]
pub struct ShareResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Share>,
}

/// 共享列表响应
#[derive(Serialize, Deserialize)]
pub struct ShareListResponse {
    pub success: bool,
    pub shares: Vec<Share>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
