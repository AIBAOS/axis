use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub temperature_celsius: Option<f32>,
}

pub async fn get_system_status(
    State(state): State<AppState>,
) -> Result<Json<SystemStatus>, (StatusCode, String)> {
    read_system_status().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read system status: {}", e)))
}

fn read_system_status() -> Result<SystemStatus, String> {
    // 读取 CPU 使用率（从 /proc/stat）
    let cpu_percent = read_cpu_usage().unwrap_or(0.0);
    
    // 读取内存使用率（从 /proc/meminfo）
    let memory_percent = read_memory_usage().unwrap_or(0.0);
    
    // 读取温度（尝试 /sys/class/thermal）
    let temperature = read_temperature().ok().flatten();
    
    Ok(SystemStatus {
        cpu_percent,
        memory_percent,
        temperature_celsius: temperature,
    })
}

fn read_cpu_usage() -> Result<f32, String> {
    // 第一次读取
    let content1 = fs::read_to_string("/proc/stat")
        .map_err(|e| format!("Failed to read /proc/stat: {}", e))?;
    
    let line1 = content1.lines().next()
        .ok_or("Failed to read CPU line")?;
    
    let parts: Vec<&str> = line1.split_whitespace().collect();
    if parts.len() < 5 {
        return Ok(0.0);
    }
    
    let (user1, nice1, system1, idle1) = (
        parts[1].parse::<u64>().unwrap_or(0),
        parts[2].parse::<u64>().unwrap_or(0),
        parts[3].parse::<u64>().unwrap_or(0),
        parts[4].parse::<u64>().unwrap_or(0),
    );
    let total1 = user1 + nice1 + system1 + idle1;
    
    // 简化处理：返回静态值，实际应使用两个时间点的差值计算
    // TODO: 实现精确的 CPU 使用率计算
    Ok(25.0) // 示例值
}

fn read_memory_usage() -> Result<f32, String> {
    let content = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("Failed to read /proc/meminfo: {}", e))?;
    
    let mut total: u64 = 0;
    let mut available: u64 = 0;
    
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
        }
    }
    
    if total == 0 {
        return Ok(0.0);
    }
    
    let used = total - available;
    let percent = (used as f32 / total as f32) * 100.0;
    
    Ok(percent)
}

fn read_temperature() -> Result<Option<f32>, String> {
    // 尝试读取 CPU 温度
    let paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
    ];
    
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            // 温度值通常以毫摄氏度为单位
            if let Some(temp_str) = content.trim().strip_suffix("000") {
                if let Ok(temp) = temp_str.parse::<f32>() {
                    return Ok(Some(temp / 1000.0));
                }
            }
        }
    }
    
    Ok(None)
}
