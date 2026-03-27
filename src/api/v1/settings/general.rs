use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub site_name: String,
    pub timezone: String,
    pub language: String,
    pub theme: String,
}

pub async fn get_general_settings(
    State(state): State<AppState>,
) -> Result<Json<GeneralSettings>, (StatusCode, String)> {
    let config = state.config.read().await;
    let settings = GeneralSettings {
        site_name: config.site.name.clone(),
        timezone: config.site.timezone.clone(),
        language: config.site.language.clone(),
        theme: config.site.theme.clone(),
    };
    Ok(Json(settings))
}

pub async fn update_general_settings(
    State(state): State<AppState>,
    Json(settings): Json<GeneralSettings>,
) -> Result<Json<GeneralSettings>, (StatusCode, String)> {
    let mut config = state.config.write().await;
    config.site.name = settings.site_name;
    config.site.timezone = settings.timezone;
    config.site.language = settings.language;
    config.site.theme = settings.theme;
    
    config.save().map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save config: {}", e))
    })?;
    
    Ok(Json(settings))
}
