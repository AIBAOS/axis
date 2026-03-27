pub mod log;

use axum::{Router, routing::{get, post}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(log::get_audit_logs).post(log::create_audit_log))
        .route("/:id", get(log::get_audit_log_by_id))
}
