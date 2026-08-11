/// Builds the axum application, mounting our api routes.
pub fn web() -> axum::Router {
  sable_api::routes::mount()
}
