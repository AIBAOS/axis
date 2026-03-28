use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemResources {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub threads: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
}

pub async fn get_system_resources(
    State(state): State<AppState>,
) -> Result<Json<SystemResources>, (StatusCode, String)> {
    read_system_resources().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read system resources: {}", e)))
}

fn read_system_resources() -> Result<SystemResources, String> {
    Ok(SystemResources {
        cpu: read_cpu_info()?,
        memory: read_memory_info()?,
        disk: read_disk_info()?,
    })
}

fn read_cpu_info() -> Result<CpuInfo, String> {
    // 读取 CPU 型号
    let model = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|content| content.lines()
            .find(|l| l.starts_with("model name"))
            .or_else(|| content.lines().find(|l| l.starts_with("Processor")))
            .map(|l| l.split_once(':').map(|(_, v)| v.trim().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string()))
        )
        .unwrap_or_else(|| "Unknown CPU".to_string());
    
    // 计算核心数（从 /proc/cpuinfo 或 /proc/stat）
    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);
    
    Ok(CpuInfo {
        model,
        cores: cpu_count,
        threads: cpu_count,
    })
}

fn read_memory_info() -> Result<MemoryInfo, String> {
    let content = fs::read_to_string("/proc/meminfo")
        .map_err(|e| format!("Failed to read /proc/meminfo: {}", e))?;
    
    let mut total: u64 = 0;
    let mut available: u64 = 0;
    
    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0) * 1024; // 转换为字节
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1)
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0) * 1024; // 转换为字节
        }
    }
    
    Ok(MemoryInfo {
        total_bytes: total,
        available_bytes: available,
    })
}

fn read_disk_info() -> Result<DiskInfo, String> {
    // 读取根分区的磁盘信息
    let stat = fs::metadata("/").map_err(|e| format!("Failed to read rootfs: {}", e))?;
    let stat = stat.system();
    
    Ok(DiskInfo {
        total_bytes: stat.total_bytes(),
        used_bytes: stat.total_bytes() - stat.free_bytes(),
        available_bytes: stat.free_bytes(),
    })
}
