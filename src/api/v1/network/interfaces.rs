use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub enabled: bool,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
    pub mac_address: String,
    pub speed: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInterfaceRequest {
    pub enabled: Option<bool>,
    pub ip_address: Option<String>,
    pub subnet_mask: Option<String>,
    pub gateway: Option<String>,
}

pub async fn get_interfaces(
    State(state): State<AppState>,
) -> Result<Json<Vec<NetworkInterface>>, (StatusCode, String)> {
    read_interfaces_config().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read interfaces: {}", e)))
}

fn read_interfaces_config() -> Result<Vec<NetworkInterface>, String> {
    let content = fs::read_to_string("/etc/network/interfaces")
        .map_err(|e| format!("Failed to read /etc/network/interfaces: {}", e))?;
    
    let mut interfaces = HashMap::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("iface") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let name = parts[1].to_string();
                let mut iface = interfaces.entry(name).or_insert(NetworkInterface {
                    name: parts[1].to_string(),
                    enabled: true,
                    ip_address: "".to_string(),
                    subnet_mask: "".to_string(),
                    gateway: None,
                    mac_address: "".to_string(),
                    speed: None,
                });
                
                match parts[2] {
                    "inet" => {
                        if parts.len() >= 4 {
                            let addr = parts[3];
                            if addr != "loopback" {
                                iface.ip_address = addr.to_string();
                            }
                        }
                    }
                    "inet6" => {
                        // IPv6 support
                    }
                    _ => {}
                }
            }
        } else if line.starts_with("address") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some(iface) = interfaces.values_mut().next() {
                    iface.ip_address = parts[1].to_string();
                }
            }
        } else if line.starts_with("netmask") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some(iface) = interfaces.values_mut().next() {
                    iface.subnet_mask = parts[1].to_string();
                }
            }
        } else if line.starts_with("gateway") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some(iface) = interfaces.values_mut().next() {
                    iface.gateway = Some(parts[1].to_string());
                }
            }
        } else if line.starts_with("hwaddress") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some(iface) = interfaces.values_mut().next() {
                    iface.mac_address = parts[1].replace("ARPHRD_ETHER:", "");
                }
            }
        }
    }
    
    Ok(interfaces.into_values().collect())
}

pub async fn get_interface_by_name(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<NetworkInterface>, (StatusCode, String)> {
    let mut interfaces = read_interfaces_config().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    interfaces.into_iter()
        .find(|i| i.name == name)
        .ok_or((StatusCode::NOT_FOUND, format!("Interface '{}' not found", name)))
        .map(Json)
}

pub async fn update_interface(
    State(state): State<AppState>,
    axum::extract::Path(name): axum::extract::Path<String>,
    Json(request): Json<UpdateInterfaceRequest>,
) -> Result<Json<NetworkInterface>, (StatusCode, String)> {
    // TODO: 验证并更新网络接口配置
    // 需要调用系统命令 (如 ifconfig, ip route) 或编辑配置文件
    Ok(Json(NetworkInterface {
        name,
        enabled: request.enabled.unwrap_or(true),
        ip_address: request.ip_address.unwrap_or_else(|| "192.168.1.100".to_string()),
        subnet_mask: request.subnet_mask.unwrap_or_else(|| "255.255.255.0".to_string()),
        gateway: request.gateway,
        mac_address: "00:11:22:33:44:55".to_string(),
        speed: Some(1000),
    }))
}
