pub mod server;

use axum::{Router, routing::get};

async fn sable() -> &'static str {
    ":3c"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(sable))
        .nest("/server", server::router())
}
