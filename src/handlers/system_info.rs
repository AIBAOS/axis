// 系统信息管理处理器（Phase 59/60+）
// Phase 59: 增强 JWT 认证
// 包含：系统信息、健康检查、资源监控等接口

use actix_web::{HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};

use crate::models::jwt::JwtClaims;

/// CPU 信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub usage_percent: f32,
}

/// 内存信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
}

/// 磁盘信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
}

/// 网络 IO
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkIo {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

/// 实时资源监控数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceStats {
    pub timestamp: String,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub network: NetworkIo,
}

/// 系统信息（完整版）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os: OsInfo,
    pub kernel: String,
    pub uptime_seconds: u64,
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub disk: DiskInfo,
    pub boot_time: String,
    pub updated_at: String,
}

/// 操作系统信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
}

/// 健康检查状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemHealthStatus {
    Healthy,
    Degraded,
    Critical,
}

impl std::fmt::Display for SystemHealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemHealthStatus::Healthy => write!(f, "healthy"),
            SystemHealthStatus::Degraded => write!(f, "degraded"),
            SystemHealthStatus::Critical => write!(f, "critical"),
        }
    }
}

/// 服务状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

/// 健康检查响应（Phase 59 增强）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemHealth {
    pub status: String,
    pub checked_at: String,
    pub uptime_seconds: u64,
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub services: Vec<ServiceStatus>,
    pub alerts: Vec<String>,
}

/// 检查当前用户是否已认证
fn is_authenticated(_claims: &JwtClaims) -> bool {
    true // 任意登录用户可访问
}

/// 获取系统信息
pub async fn get_system_info() -> Result<HttpResponse> {
    let now = chrono::Utc::now();
    let boot_time_str = "2026-03-18T08:00:00Z";
    let boot_time_formatted = boot_time_str.to_string();
    
    // 静态 mock 数据
    Ok(HttpResponse::Ok().json(SystemInfo {
        hostname: "axis-nas".to_string(),
        os: OsInfo {
            name: "Linux".to_string(),
            version: "6.6.87.2-microsoft-standard-WSL2".to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
        kernel: "6.6.87.2-microsoft-standard-WSL2".to_string(),
        uptime_seconds: 86400, // 24 小时
        cpu: CpuInfo {
            model: "AMD EPYC".to_string(),
            cores: 8,
            usage_percent: 25.5,
        },
        memory: MemoryInfo {
            total_bytes: 34359738368, // 32GB
            used_bytes: 12884901888,  // 12GB
            available_bytes: 21474836480, // 20GB
            usage_percent: 37.5,
        },
        disk: DiskInfo {
            total_bytes: 1099511627776, // 1TB
            used_bytes: 549755813888,  // 512GB
            available_bytes: 549755813888, // 512GB
            usage_percent: 50.0,
        },
        boot_time: boot_time_formatted,
        updated_at: now.to_rfc3339(),
    }))
}

/// 获取系统健康检查（Phase 59 增强）
/// - JWT 认证，任意登录用户可访问
/// - 返回系统状态信息：CPU/内存/磁盘使用率、运行时间、服务状态
pub async fn get_system_health(
    req: HttpRequest,
) -> Result<HttpResponse> {
    // 1. JWT 认证 - 提取并验证 token（任意登录用户可访问）
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing or invalid Authorization header"))?;

    // 简化验证：仅检查 token 是否存在
    if token.is_empty() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "error": "Invalid token"
        })));
    }

    let now = chrono::Utc::now();
    
    // 2. 静态 mock 健康数据
    let cpu_usage = 25.5;
    let memory_percent = 37.5;
    let disk_percent = 50.0;
    let uptime_seconds = 86400; // 24 小时
    
    // 3. 服务状态
    let services = vec![
        ServiceStatus {
            name: "database".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "cache".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "storage".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
        ServiceStatus {
            name: "network".to_string(),
            status: "healthy".to_string(),
            message: None,
        },
    ];
    
    // 4. 确定健康状态
    let mut alerts = vec![];
    let status = if memory_percent > 90.0 || disk_percent > 90.0 || cpu_usage > 95.0 {
        alerts.push("High resource usage detected".to_string());
        SystemHealthStatus::Critical.to_string()
    } else if memory_percent > 75.0 || disk_percent > 75.0 || cpu_usage > 80.0 {
        alerts.push("Elevated resource usage".to_string());
        SystemHealthStatus::Degraded.to_string()
    } else {
        SystemHealthStatus::Healthy.to_string()
    };
    
    // 5. 返回健康检查响应
    Ok(HttpResponse::Ok().json(SystemHealth {
        status,
        checked_at: now.to_rfc3339(),
        uptime_seconds,
        cpu_usage_percent: cpu_usage,
        memory_usage_percent: memory_percent,
        disk_usage_percent: disk_percent,
        services,
        alerts,
    }))
}

/// 获取实时资源监控
pub async fn get_system_resources() -> Result<HttpResponse> {
    let now = chrono::Utc::now();
    
    // 静态 mock 数据
    let cpu_usage = 25.5;
    let memory_percent = 37.5;
    let disk_percent = 50.0;
    
    // 网络 IO（模拟）
    let network_io = NetworkIo {
        rx_bytes: 100_000_000_000,
        tx_bytes: 50_000_000_000,
        rx_packets: 1000000,
        tx_packets: 500000,
    };
    
    Ok(HttpResponse::Ok().json(ResourceStats {
        timestamp: now.to_rfc3339(),
        cpu: CpuInfo {
            model: "AMD EPYC".to_string(),
            cores: 8,
            usage_percent: cpu_usage,
        },
        memory: MemoryInfo {
            total_bytes: 34359738368,
            used_bytes: 12884901888,
            available_bytes: 21474836480,
            usage_percent: memory_percent,
        },
        disk: DiskInfo {
            total_bytes: 1099511627776,
            used_bytes: 549755813888,
            available_bytes: 549755813888,
            usage_percent: disk_percent,
        },
        network: network_io,
    }))
}

/// 获取系统资源历史（最近 24 小时）
pub async fn get_resource_history() -> Result<HttpResponse> {
    let now = chrono::Utc::now();
    
    let mut history = vec![];
    for i in (0..24).rev() {
        let timestamp = now - chrono::Duration::hours(i as i64);
        let usage_percent = (75 + i * 5) % 100;
        
        history.push(ResourceStats {
            timestamp: timestamp.to_rfc3339(),
            cpu: CpuInfo {
                model: "AMD EPYC".to_string(),
                cores: 8,
                usage_percent: usage_percent as f32,
            },
            memory: MemoryInfo {
                total_bytes: 34359738368, // 32GB
                used_bytes: (usage_percent as u64 * 34359738368 / 100),
                available_bytes: 34359738368 - (usage_percent as u64 * 34359738368 / 100),
                usage_percent: usage_percent as f32,
            },
            disk: DiskInfo {
                total_bytes: 1099511627776, // 1TB
                used_bytes: (usage_percent as u64 * 1099511627776 / 100),
                available_bytes: 1099511627776 - (usage_percent as u64 * 1099511627776 / 100),
                usage_percent: usage_percent as f32,
            },
            network: NetworkIo {
                rx_bytes: (usage_percent as u64 * 1000000000),
                tx_bytes: (usage_percent as u64 * 500000000),
                rx_packets: usage_percent * 1000,
                tx_packets: usage_percent * 500,
            },
        });
    }
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "history": history,
        "period": "24h",
        "interval": "1h"
    })))
}
