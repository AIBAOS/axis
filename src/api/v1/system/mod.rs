pub mod info;
pub mod status;
pub mod resources;

use axum::{Router, routing::get};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/info", get(info::get_system_info))
        .route("/status", get(status::get_system_status))
        .route("/resources", get(resources::get_system_resources))
}
