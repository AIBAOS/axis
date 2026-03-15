// 数据库配置
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub r#type: String,  // "sqlite" or "postgres"
    pub path: Option<String>,
    pub postgres_url: Option<String>,
}

impl DatabaseConfig {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }
}
