use axum::{Router, routing::get};

pub mod server_version;

pub fn router() -> Router {
  Router::new().route("/server_version", get(server_version::server_version))
}
