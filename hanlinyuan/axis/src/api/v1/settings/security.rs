use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub requires_two_factor: bool,
    pub session_timeout: u32,
    pub max_login_attempts: u32,
    pub lockout_duration: u32,
    pub password_minimum_length: u32,
    pub require_special_characters: bool,
}

pub async fn get_security_settings(
    State(state): State<AppState>,
) -> Result<Json<SecuritySettings>, (StatusCode, String)> {
    let config = state.config.read().await;
    let settings = SecuritySettings {
        requires_two_factor: config.security.two_factor.enabled,
        session_timeout: config.security.session.timeout_secs,
        max_login_attempts: config.security.login.max_attempts,
        lockout_duration: config.security.login.lockout_duration_secs,
        password_minimum_length: config.security.password.min_length,
        require_special_characters: config.security.password.require_special_chars,
    };
    Ok(Json(settings))
}

pub async fn update_security_settings(
    State(state): State<AppState>,
    Json(settings): Json<SecuritySettings>,
) -> Result<Json<SecuritySettings>, (StatusCode, String)> {
    let mut config = state.config.write().await;
    config.security.two_factor.enabled = settings.requires_two_factor;
    config.security.session.timeout_secs = settings.session_timeout;
    config.security.login.max_attempts = settings.max_login_attempts;
    config.security.login.lockout_duration_secs = settings.lockout_duration;
    config.security.password.min_length = settings.password_minimum_length;
    config.security.password.require_special_chars = settings.require_special_characters;
    
    config.save().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save config: {}", e))
    })?;
    
    Ok(Json(settings))
}
