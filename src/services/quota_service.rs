/// 配额服务模块
use crate::models::quota::UserQuota;
use crate::database::quota_store::SqliteQuotaRepository;
use std::sync::{Arc, Mutex};

/// QUOTA-1: 配额检查结果
#[derive(Debug)]
pub enum QuotaCheckResult {
    /// 配额充足，剩余空间（字节）
    Available(u64),
    /// 配额不足，需要空间（字节）
    Insufficient(u64),
    /// 未设置配额（无限制）
    NoQuota,
}

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

    /// 安全获取仓库锁，处理 mutex poison
    fn get_repo(&self) -> std::sync::MutexGuard<'_, SqliteQuotaRepository> {
        match self.repository.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("QuotaService mutex poisoned, recovering");
                poisoned.into_inner()
            }
        }
    }

    /// QUOTA-1: 检查用户是否有足够的配额
    /// 返回：Available(剩余空间)、Insufficient(需要空间)、NoQuota(无限制)
    pub fn check_quota(&self, user_id: u64, required_bytes: u64) -> QuotaCheckResult {
        match self.get_quota(user_id) {
            Some(quota) => {
                if quota.quota_bytes == 0 {
                    // 配额为 0 表示无限制
                    QuotaCheckResult::NoQuota
                } else {
                    let remaining = quota.quota_bytes.saturating_sub(quota.used_bytes);
                    if remaining >= required_bytes {
                        QuotaCheckResult::Available(remaining - required_bytes)
                    } else {
                        QuotaCheckResult::Insufficient(required_bytes - remaining)
                    }
                }
            }
            None => {
                // 未设置配额，默认无限制
                QuotaCheckResult::NoQuota
            }
        }
    }

    /// QUOTA-1: 预检查并占用配额
    /// 成功返回 true，失败返回 false
    pub fn reserve_quota(&self, user_id: u64, bytes: u64) -> Result<(), String> {
        match self.check_quota(user_id, bytes) {
            QuotaCheckResult::Available(_) | QuotaCheckResult::NoQuota => {
                // 更新已用空间
                self.update_used(user_id, bytes as i64)?;
                Ok(())
            }
            QuotaCheckResult::Insufficient(needed) => {
                Err(format!("Quota exceeded. Need {} more bytes", needed))
            }
        }
    }

    /// QUOTA-1: 释放配额（用于上传失败回滚）
    pub fn release_quota(&self, user_id: u64, bytes: u64) -> Result<(), String> {
        self.update_used(user_id, -(bytes as i64))
    }

    /// 获取用户配额
    pub fn get_quota(&self, user_id: u64) -> Option<UserQuota> {
        let repo = self.get_repo();
        repo.get_quota(user_id).ok().flatten()
    }

    /// 设置用户配额
    pub fn set_quota(&self, user_id: u64, quota_bytes: u64) -> Result<(), String> {
        let repo = self.get_repo();
        repo.set_quota(user_id, quota_bytes)
    }

    /// 更新已用空间
    pub fn update_used(&self, user_id: u64, delta: i64) -> Result<UserQuota, String> {
        let repo = self.get_repo();
        repo.update_used(user_id, delta)
    }

    /// 列出所有配额
    pub fn list_quotas(&self, page: u64, page_size: u64) -> Vec<UserQuota> {
        let repo = self.get_repo();
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