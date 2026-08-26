use axum::Json;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Brand {
    pub name: &'static str,
    pub version: &'static str,
}

pub async fn brand() -> Json<Brand> {
    Json(Brand {
        name: "Blueberry",
        version: env!("CARGO_PKG_VERSION"),
    })
}
