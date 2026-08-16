use axum::Router;

pub mod blueberry;
pub mod sable;

/// Returns a `Router` with all routes mounted.
pub fn mount() -> Router {
  Router::new()
    .nest("/_sable", sable::router())
    .nest("/_blueberry", blueberry::router())
}
