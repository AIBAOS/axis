use axum::{Routing, Router};

use super:: AppState;

pub mod settings;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/settings", settings::routes())
}
