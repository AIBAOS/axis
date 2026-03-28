// 配额服务模块
use crate::models::quota::UserQuota;
use crate::database::quota_store::SqliteQuotaRepository;
use std::sync::{Arc, Mutex};

/// 配额服务
pub struct QuotaService {
    repository: Arc<Mutex<SqliteQuotaRepository>>,
}

impl QuotaService {
    pub fn new(db: Arc<Mutex<crate::database::pool::DbConnectionType>>) -> Self {
        let repo = SqliteQuotaRepository::new(db);
        repo.init_table().ok();
        
        Self {
            repository: Arc::new(Mutex::new(repo)),
        }
    }

    /// 获取用户配额
    pub fn get_quota(&self, user_id: u64) -> Option<UserQuota> {
        let repo = self.repository.lock().unwrap();
        repo.get_quota(user_id).ok().flatten()
    }

    /// 设置用户配额
    pub fn set_quota(&self, user_id: u64, quota_bytes: u64) -> Result<(), String> {
        let repo = self.repository.lock().unwrap();
        repo.set_quota(user_id, quota_bytes)
    }

    /// 更新已用空间
    pub fn update_used(&self, user_id: u64, delta: i64) -> Result<UserQuota, String> {
        let repo = self.repository.lock().unwrap();
        repo.update_used(user_id, delta)
    }

    /// 列出所有配额
    pub fn list_quotas(&self, page: u64, page_size: u64) -> Vec<UserQuota> {
        let repo = self.repository.lock().unwrap();
        match repo.list_quotas(page, page_size) {
            Ok(quotas) => quotas,
            Err(_) => Vec::new(),
        }
    }

    /// 获取配额使用情况
    pub fn get_quota_usage(&self, user_id: u64) -> Option<UserQuota> {
        self.get_quota(user_id)
    }
}
