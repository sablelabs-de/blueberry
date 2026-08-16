pub mod brand;

pub use brand::Brand;

use axum::{Router, routing::get};

pub fn router() -> Router {
  Router::new().route("/brand", get(brand::brand))
}
