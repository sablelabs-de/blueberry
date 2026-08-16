use axum::{
  Router,
  extract::Request,
  middleware::{Next, from_fn},
  response::{IntoResponse, Redirect, Response},
};

pub mod blueberry;
pub mod sable;

/// Returns a `Router` with all routes mounted.
pub fn mount() -> Router {
  Router::new()
    .nest("/_sable", sable::router())
    .nest("/_blueberry", blueberry::router())
    .layer(from_fn(normalize_slash))
}

/// Redirects trailing-slash paths to their non-trailing-slash equiv.
/// Preserves the query string, ex: `/path/?a=1` -> `/path?a=1`.
async fn normalize_slash(req: Request, next: Next) -> Response {
  let uri = req.uri();
  let Some(path) = uri.path().strip_suffix('/') else {
    return next.run(req).await;
  };
  if path.is_empty() {
    return next.run(req).await;
  }
  let mut new_uri = path.to_owned();
  if let Some(q) = uri.query() {
    new_uri.push('?');
    new_uri.push_str(q);
  }
  Redirect::permanent(&new_uri).into_response()
}
