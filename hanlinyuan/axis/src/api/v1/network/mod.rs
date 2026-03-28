pub mod interfaces;
pub mod dns;
pub mod proxy;

use axum::Router;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/interfaces", Router::new()
            .route("/", get(interfaces::get_interfaces))
            .route("/:name", get(interfaces::get_interface_by_name).put(interfaces::update_interface))
        )
        .nest("/dns", Router::new()
            .route("/", get(dns::get_dns).put(dns::update_dns))
        )
        .nest("/proxy", Router::new()
            .route("/", get(proxy::get_proxy).put(proxy::update_proxy))
        )
}
