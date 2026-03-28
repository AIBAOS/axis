use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::env;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub enabled: bool,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProxyRequest {
    pub enabled: Option<bool>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

pub async fn get_proxy(
    State(state): State<AppState>,
) -> Result<Json<ProxyConfig>, (StatusCode, String)> {
    let http_proxy = env::var("HTTP_PROXY").ok();
    let https_proxy = env::var("HTTPS_PROXY").ok();
    let no_proxy = env::var("NO_PROXY").ok();
    
    Ok(Json(ProxyConfig {
        enabled: http_proxy.is_some() || https_proxy.is_some(),
        http_proxy,
        https_proxy,
        no_proxy,
    }))
}

pub async fn update_proxy(
    State(state): State<AppState>,
    Json(request): Json<UpdateProxyRequest>,
) -> Result<Json<ProxyConfig>, (StatusCode, String)> {
    // TODO: 验证并更新代理配置
    Ok(Json(ProxyConfig {
        enabled: request.enabled.unwrap_or(false),
        http_proxy: request.http_proxy,
        https_proxy: request.https_proxy,
        no_proxy: request.no_proxy,
    }))
}
