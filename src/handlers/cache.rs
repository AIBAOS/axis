use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_rate: f32,
    pub miss_rate: f32,
    pub total_keys: u64,
    pub memory_usage_bytes: u64,
    pub eviction_count: u64,
}

pub async fn get_cache_stats() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(CacheStats {
        hit_rate: 85.5,
        miss_rate: 14.5,
        total_keys: 10240,
        memory_usage_bytes: 52428800,
        eviction_count: 128,
    }))
}
