/// Builds the axum application, mounting our api routes.
pub fn web() -> axum::Router {
  blueberry_api::rest::routes::mount()
}
