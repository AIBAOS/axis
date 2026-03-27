pub mod disks;

use axum::{Router, routing::get};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/disks", get(disks::get_disks))
}
