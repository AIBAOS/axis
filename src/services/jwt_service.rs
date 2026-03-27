// JWT 处理模块
// 包含：Token 生成、验证、刷新

use crate::models::jwt::{JwtClaims, JwtConfig, JwtResponse, JwtToken};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use ring::rand::SecureRandom;
use ring::pbkdf2;

/// JWT 服务结构
pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// 生成 JWT Token
    pub fn generate_token(&self, user_id: u64, _username: &str, roles: Vec<String>, permissions: Vec<String>) -> Result<JwtResponse, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Invalid time")?
            .as_secs();

        let expires_at = now + self.config.expiration_minutes * 60;

        let claims = JwtClaims {
            sub: user_id.to_string(),
            user_id,
            username: _username.to_string(),
            issuer: self.config.issuer.clone(),
            audience: self.config.audience.clone(),
            exp: expires_at,
            iat: now,
            roles,
            permissions,
        };

        let encoding_key = EncodingKey::from_secret(self.config.secret_key.as_bytes());

        let token = encode(&Header::default(), &claims, &encoding_key)
            .map_err(|e| format!("Failed to encode token: {}", e))?;

        Ok(JwtResponse {
            success: true,
            message: "Token generated successfully".to_string(),
            data: Some(JwtToken {
                access_token: token,
                token_type: "Bearer".to_string(),
                expires_in: self.config.expiration_minutes * 60,
                refresh_token: None, // Phase 2.1: 未实现刷新 Token
            }),
        })
    }

    /// 验证 JWT Token
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims, String> {
        let decoding_key = DecodingKey::from_secret(self.config.secret_key.as_bytes());
        let validation = Validation::default();

        let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
            .map_err(|e| format!("Failed to decode token: {}", e))?;

        Ok(token_data.claims)
    }

    /// 验证 Authorization 头
    pub fn validate_authorization(&self, auth_header: &str) -> Result<JwtClaims, String> {
        let parts: Vec<&str> = auth_header.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return Err("Invalid Authorization header format".to_string());
        }

        self.validate_token(parts[1])
    }
}

/// 生成安全随机盐值
pub fn generate_salt() -> String {
    let mut buffer = [0u8; 16];
    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut buffer).expect("Failed to generate random salt");
    buffer.iter().map(|b| format!("{:02x}", b)).collect()
}

/// 密码哈希（PBKDF2）
pub fn hash_password(password: &str, salt: &str) -> String {
    let mut output = [0u8; 32];
    use std::num::NonZeroU32;
    let iterations = NonZeroU32::new(10000).expect("Invalid iterations");
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt.as_bytes(),
        password.as_bytes(),
        &mut output,
    );
    output.iter().map(|b| format!("{:02x}", b)).collect()
}
