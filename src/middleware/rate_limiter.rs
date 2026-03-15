use actix_web::{web, HttpResponse, Error};
use actix_web::error::ErrorTooManyRequests;
use std::net::IpAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{warn};

// 滑动窗口限流器：按 IP 限速
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    max_requests_per_second: usize,
}

impl RateLimiter {
    pub fn new(max_requests_per_second: usize) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests_per_second,
        }
    }

    // 检查 IP 是否被允许访问（true=允许，false=被限流）
    pub fn is_allowed(&self, ip: &IpAddr) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.lock().unwrap();

        let entry = requests.entry(*ip).or_insert_with(Vec::new);

        // 清理 1 秒前的旧请求记录
        entry.retain(|&t| now.duration_since(t) <= Duration::from_secs(1));

        if entry.len() < self.max_requests_per_second {
            entry.push(now);
            true
        } else {
            false
        }
    }
}

// 默认配置：每秒 10 个请求
pub fn create_default_rate_limiter() -> RateLimiter {
    RateLimiter::new(10)
}
