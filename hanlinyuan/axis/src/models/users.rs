// Phase 34 用户模型
// 包含：用户列表响应结构

use serde::{Deserialize, Serialize};

/// 用户信息（用于列表展示）
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserListItem {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub created_at: u64,
}

/// 用户列表响应
#[derive(Serialize, Deserialize)]
pub struct UserListResponse {
    pub success: bool,
    pub data: Vec<UserListItem>,
    pub pagination: UserPagination,
}

/// 分页信息
#[derive(Serialize, Deserialize)]
pub struct UserPagination {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}
