use axum::{routing::get, Router};

// this is temporary
async fn sable() -> &'static str {
  "sable"
}

/// Returns a `Router` with all routes mounted.
pub fn mount() -> Router {
  Router::new().route("/_sable", get(sable))
}
