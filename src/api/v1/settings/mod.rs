pub mod general;
pub mod security;

use axum::{Router, routing::{get, put}};

pub fn routes() -> Router {
    Router::new()
        .route("/general", get(general::get_general_settings).put(general::update_general_settings))
        .route("/security", get(security::get_security_settings).put(security::update_security_settings))
}
