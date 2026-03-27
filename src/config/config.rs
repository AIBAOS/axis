// 配置文件模块 - 单文件实现
// 包含：配置结构体、Toml加载、环境变量覆盖

use serde::{Deserialize, Serialize};
use std::env;

/// JWT 配置
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SettingsJwtConfig {
    pub secret_key: String,
    pub issuer: String,
    pub audience: String,
    pub expiration_minutes: u64,
    pub refresh_enabled: bool,
}

/// 数据库配置
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseConfig {
    pub path: String,
    pub max_connections: u32,
}

/// 应用配置
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppSettings {
    pub jwt: SettingsJwtConfig,
    pub database: DatabaseConfig,
    pub server_host: String,
    pub server_port: u16,
}

impl AppSettings {
    /// 从 config.toml 文件加载配置
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: AppSettings = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(config)
    }

    /// 从环境变量加载配置（覆盖文件中的值）
    pub fn from_env(mut self) -> Self {
        if let Ok(val) = env::var("JWT_SECRET_KEY") {
            self.jwt.secret_key = val;
        }
        if let Ok(val) = env::var("JWT_ISSUER") {
            self.jwt.issuer = val;
        }
        if let Ok(val) = env::var("JWT_AUDIENCE") {
            self.jwt.audience = val;
        }
        if let Ok(val) = env::var("JWT_EXPIRATION_MINUTES") {
            if let Ok(minutes) = val.parse() {
                self.jwt.expiration_minutes = minutes;
            }
        }
        if let Ok(val) = env::var("JWT_REFRESH_ENABLED") {
            if let Ok(enabled) = val.parse() {
                self.jwt.refresh_enabled = enabled;
            }
        }
        if let Ok(val) = env::var("DATABASE_PATH") {
            self.database.path = val;
        }
        if let Ok(val) = env::var("DATABASE_MAX_CONNECTIONS") {
            if let Ok(count) = val.parse() {
                self.database.max_connections = count;
            }
        }
        if let Ok(val) = env::var("SERVER_HOST") {
            self.server_host = val;
        }
        if let Ok(val) = env::var("SERVER_PORT") {
            if let Ok(port) = val.parse() {
                self.server_port = port;
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppSettings {
            jwt: SettingsJwtConfig {
                secret_key: "default_secret".to_string(),
                issuer: "axis-test".to_string(),
                audience: "axis-test-users".to_string(),
                expiration_minutes: 60,
                refresh_enabled: false,
            },
            database: DatabaseConfig {
                path: "NAS.db".to_string(),
                max_connections: 10,
            },
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
        };

        assert_eq!(config.jwt.secret_key, "default_secret");
        assert_eq!(config.database.max_connections, 10);
    }
}
