use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime_seconds: u64,
}

pub async fn get_system_info(
    State(state): State<AppState>,
) -> Result<Json<SystemInfo>, (StatusCode, String)> {
    read_system_info().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read system info: {}", e)))
}

fn read_system_info() -> Result<SystemInfo, String> {
    // 获取主机名
    let hostname = Command::new("hostname").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    // 获取 OS 信息
    let os = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| content.lines()
            .find(|l| l.starts_with("PRETTY_NAME="))
            .map(|l| l.trim_start_matches("PRETTY_NAME=\"").trim_end_matches('"').to_string())
        )
        .unwrap_or_else(|| "Unknown OS".to_string());
    
    // 获取内核版本
    let kernel = Command::new("uname").args(&["-r"]).output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    // 获取运行时间（秒）
    let uptime_seconds = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|content| content.split_whitespace().next())
        .and_then(|s| s.split('.').next())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    
    Ok(SystemInfo {
        hostname,
        os,
        kernel,
        uptime_seconds,
    })
}
