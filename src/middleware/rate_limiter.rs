use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform, Service}, Error};
use futures_util::future::{ok, Ready};
use std::future::Future;
use std::net::IpAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::debug;

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
        let mut requests = match self.requests.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                debug!("RateLimiter mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };

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

    // 清理过期条目（默认 60 秒内未访问即删除）
    pub fn cleanup_old_entries(&self, max_age_secs: u64) {
        let now = Instant::now();
        let mut requests = match self.requests.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                debug!("RateLimiter cleanup mutex poisoned, recovering");
                poisoned.into_inner()
            }
        };

        let old_keys: Vec<IpAddr> = requests
            .iter()
            .filter_map(|(ip, timestamps)| {
                if timestamps.is_empty() {
                    Some(*ip)
                } else {
                    let latest = timestamps.last().unwrap();
                    if now.duration_since(*latest).as_secs() > max_age_secs {
                        Some(*ip)
                    } else {
                        None
                    }
                }
            })
            .collect();

        for ip in &old_keys {
            requests.remove(ip);
        }

        debug!("RateLimiter cleaned {} stale entries", old_keys.len());
    }
}

// 默认配置：每秒 10 个请求
pub fn create_default_rate_limiter() -> RateLimiter {
    RateLimiter::new(10)
}

// 简单限流中间件（禁用以避免类型问题）
// 在 Phase 2.1 中启用
pub struct RateLimitMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RateLimitMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // 简单起见，暂不启用限流
        let _ = self;
        ok(RateLimitMiddleware { service })
    }
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
