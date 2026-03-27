use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub device: String,
    pub model: Option<String>,
    pub capacity: u64,        // in bytes
    pub mount_point: Option<String>,
    pub usage_percent: Option<f32>,
    pub health: Option<String>,
}

pub async fn get_disks(
    State(state): State<AppState>,
) -> Result<Json<Vec<DiskInfo>>, (StatusCode, String)> {
    read_disk_info().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read disks: {}", e)))
}

fn read_disk_info() -> Result<Vec<DiskInfo>, String> {
    let mut disks = Vec::new();
    
    // 使用 lsblk 获取磁盘基本信息
    let lsblk_output = Command::new("lsblk").args(&[
        "-J", "-o", "NAME,MODEL,SIZE,MOUNTPOINT"
    ]).output().ok();
    
    if let Some(output) = lsblk_output {
        let json_str = String::from_utf8_lossy(&output.stdout);
        
        // TODO: 实现完整的 JSON 解析逻辑
        // 简化处理：返回示例数据
        disks = vec![
            DiskInfo {
                device: "sda".to_string(),
                model: Some("ST1000DM003-1SB1".to_string()),
                capacity: 1000204886016,
                mount_point: Some("/".to_string()),
                usage_percent: Some(45.5),
                health: Some("OK".to_string()),
            },
            DiskInfo {
                device: "sdb".to_string(),
                model: Some("WD40EFRX-22FWCT0".to_string()),
                capacity: 4000787030016,
                mount_point: Some("/data".to_string()),
                usage_percent: Some(23.7),
                health: Some("OK".to_string()),
            },
        ];
    }
    
    // 使用 smartctl 获取健康状态
    for disk in &mut disks {
        if disk.device.starts_with("sd") {
            let health = check_smart_health(&disk.device);
            disk.health = Some(health);
        }
    }
    
    Ok(disks)
}

fn check_smart_health(device: &str) -> String {
    let smartctl_output = Command::new("smartctl")
        .args(&["-H", "/dev/".to_string() + device])
        .output();
    
    match smartctl_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("PASSED") || stdout.contains("OK") {
                "OK".to_string()
            } else {
                "WARN".to_string()
            }
        }
        _ => "UNKNOWN".to_string(),
    }
}
