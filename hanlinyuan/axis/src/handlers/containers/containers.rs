// 容器列表处理器
// 包含：容器列表接口（GET /api/v1/containers）

use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

/// 容器状态
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContainerStatus {
    Running,
    Stopped,
    Paused,
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerStatus::Running => write!(f, "running"),
            ContainerStatus::Stopped => write!(f, "stopped"),
            ContainerStatus::Paused => write!(f, "paused"),
        }
    }
}

/// 容器网络
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerNetwork {
    pub network_name: String,
    pub ip_address: String,
}

/// 容器端口映射
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerPort {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String,
}

/// 容器标签
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerLabel {
    pub key: String,
    pub value: String,
}

/// 容器信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub state: String,
    pub ports: Vec<ContainerPort>,
    pub networks: Vec<ContainerNetwork>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub labels: Vec<ContainerLabel>,
}

/// 分页参数
#[derive(Debug, Deserialize)]
pub struct ContainerQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub network: Option<String>,
}

impl Default for ContainerQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            status: None,
            network: None,
        }
    }
}

/// 容器列表响应
#[derive(Debug, Serialize)]
pub struct ContainerListResponse {
    pub success: bool,
    pub data: Vec<Container>,
    pub pagination: ContainerPagination,
}

#[derive(Debug, Serialize)]
pub struct ContainerPagination {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

/// 获取容器列表
pub async fn list_containers(
    query: web::Query<ContainerQuery>,
) -> Result<HttpResponse> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100) as u64;
    let status_filter = query.status.as_deref();
    let network_filter = query.network.as_deref();

    // 模拟数据
    let mut all_containers = vec![
        Container {
            id: "abc123def456".to_string(),
            name: "nginx-proxy".to_string(),
            image: "nginx:latest".to_string(),
            status: "running".to_string(),
            state: "running".to_string(),
            ports: vec![
                ContainerPort {
                    host_port: 80,
                    container_port: 80,
                    protocol: "tcp".to_string(),
                },
                ContainerPort {
                    host_port: 443,
                    container_port: 443,
                    protocol: "tcp".to_string(),
                },
            ],
            networks: vec![
                ContainerNetwork {
                    network_name: "bridge".to_string(),
                    ip_address: "172.17.0.2".to_string(),
                },
            ],
            created_at: "2026-03-15T10:00:00Z".to_string(),
            started_at: Some("2026-03-15T10:05:00Z".to_string()),
            labels: vec![
                ContainerLabel {
                    key: "com.example.service".to_string(),
                    value: "web-proxy".to_string(),
                },
            ],
        },
        Container {
            id: "def456ghi789".to_string(),
            name: "mysql-db".to_string(),
            image: "mysql:8.0".to_string(),
            status: "running".to_string(),
            state: "running".to_string(),
            ports: vec![
                ContainerPort {
                    host_port: 3306,
                    container_port: 3306,
                    protocol: "tcp".to_string(),
                },
            ],
            networks: vec![
                ContainerNetwork {
                    network_name: "bridge".to_string(),
                    ip_address: "172.17.0.3".to_string(),
                },
            ],
            created_at: "2026-03-16T14:30:00Z".to_string(),
            started_at: Some("2026-03-16T14:35:00Z".to_string()),
            labels: vec![
                ContainerLabel {
                    key: "com.example.service".to_string(),
                    value: "database".to_string(),
                },
            ],
        },
        Container {
            id: "ghi789jkl012".to_string(),
            name: "redis-cache".to_string(),
            image: "redis:alpine".to_string(),
            status: "running".to_string(),
            state: "paused".to_string(),
            ports: vec![
                ContainerPort {
                    host_port: 6379,
                    container_port: 6379,
                    protocol: "tcp".to_string(),
                },
            ],
            networks: vec![
                ContainerNetwork {
                    network_name: "redis-net".to_string(),
                    ip_address: "172.20.0.2".to_string(),
                },
            ],
            created_at: "2026-03-17T09:15:00Z".to_string(),
            started_at: Some("2026-03-17T09:20:00Z".to_string()),
            labels: vec![
                ContainerLabel {
                    key: "com.example.service".to_string(),
                    value: "cache".to_string(),
                },
            ],
        },
        Container {
            id: "jkl012mno345".to_string(),
            name: "legacy-app".to_string(),
            image: "node:16-alpine".to_string(),
            status: "stopped".to_string(),
            state: "exited".to_string(),
            ports: vec![
                ContainerPort {
                    host_port: 3000,
                    container_port: 3000,
                    protocol: "tcp".to_string(),
                },
            ],
            networks: vec![
                ContainerNetwork {
                    network_name: "bridge".to_string(),
                    ip_address: "172.17.0.4".to_string(),
                },
            ],
            created_at: "2026-03-18T11:00:00Z".to_string(),
            started_at: Some("2026-03-18T11:05:00Z".to_string()),
            labels: vec![
                ContainerLabel {
                    key: "com.example.service".to_string(),
                    value: "legacy-api".to_string(),
                },
            ],
        },
        Container {
            id: "mno345pqr678".to_string(),
            name: "monitoring".to_string(),
            image: "prometheus:latest".to_string(),
            status: "running".to_string(),
            state: "running".to_string(),
            ports: vec![
                ContainerPort {
                    host_port: 9090,
                    container_port: 9090,
                    protocol: "tcp".to_string(),
                },
            ],
            networks: vec![
                ContainerNetwork {
                    network_name: "monitoring-net".to_string(),
                    ip_address: "172.30.0.2".to_string(),
                },
            ],
            created_at: "2026-03-18T15:00:00Z".to_string(),
            started_at: Some("2026-03-18T15:05:00Z".to_string()),
            labels: vec![
                ContainerLabel {
                    key: "com.example.service".to_string(),
                    value: "monitoring".to_string(),
                },
            ],
        },
    ];

    // 状态过滤（支持 all）
    if status_filter != Some("all") && status_filter.is_some() {
        all_containers.retain(|c| c.status == status_filter.unwrap());
    }

    // 网络过滤
    if let Some(net_filter) = network_filter {
        all_containers.retain(|c| {
            c.networks
                .iter()
                .any(|n| n.network_name == net_filter)
        });
    }

    // 按 created_at 倒序排序
    all_containers.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let total = all_containers.len() as u64;
    let start = (page - 1) * per_page;
    let end = start + per_page;

    let paginated_containers: Vec<Container> = all_containers
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| {
            let idx = i as u64;
            if idx >= start && idx < end {
                Some(c)
            } else {
                None
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ContainerListResponse {
        success: true,
        data: paginated_containers,
        pagination: ContainerPagination {
            page,
            per_page,
            total,
            total_pages: (total + per_page - 1) / per_page,
        },
    }))
}
