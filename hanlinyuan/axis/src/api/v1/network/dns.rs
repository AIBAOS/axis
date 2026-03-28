use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsConfig {
    pub primary_dns: String,
    pub secondary_dns: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDnsRequest {
    pub primary_dns: Option<String>,
    pub secondary_dns: Option<String>,
}

pub async fn get_dns(
    State(state): State<AppState>,
) -> Result<Json<DnsConfig>, (StatusCode, String)> {
    read_dns_config().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read DNS: {}", e)))
}

fn read_dns_config() -> Result<DnsConfig, String> {
    let content = fs::read_to_string("/etc/resolv.conf")
        .map_err(|e| format!("Failed to read /etc/resolv.conf: {}", e))?;
    
    let mut primary_dns = "8.8.8.8".to_string();
    let mut secondary_dns = "8.8.4.4".to_string();
    
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("nameserver") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if primary_dns == "8.8.8.8" {
                    primary_dns = parts[1].to_string();
                } else if secondary_dns == "8.8.4.4" {
                    secondary_dns = parts[1].to_string();
                    break;
                }
            }
        }
    }
    
    Ok(DnsConfig {
        primary_dns,
        secondary_dns,
    })
}

pub async fn update_dns(
    State(state): State<AppState>,
    Json(request): Json<UpdateDnsRequest>,
) -> Result<Json<DnsConfig>, (StatusCode, String)> {
    // TODO: 验证并更新 DNS 配置 (如编辑 /etc/resolv.conf)
    Ok(Json(DnsConfig {
        primary_dns: request.primary_dns.unwrap_or_else(|| "8.8.8.8".to_string()),
        secondary_dns: request.secondary_dns.unwrap_or_else(|| "8.8.4.4".to_string()),
    }))
}
