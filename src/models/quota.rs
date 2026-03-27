// 用户配额数据模型
// 包含：配额结构体定义

use serde::{Deserialize, Serialize};

/// 用户配额
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserQuota {
    pub user_id: u64,
    pub quota_bytes: u64,
    pub used_bytes: u64,
    pub updated_at: u64,
}

/// 配额响应
#[derive(Serialize, Deserialize)]
pub struct QuotaResponse {
    pub success: bool,
    pub data: Option<UserQuota>,
}

/// 配额列表响应
#[derive(Serialize, Deserialize)]
pub struct QuotaListResponse {
    pub success: bool,
    pub quotas: Vec<UserQuota>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
