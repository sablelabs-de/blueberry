use axum::Json;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ServerVersion {
    pub version: &'static str,
}

pub async fn server_version() -> Json<ServerVersion> {
    Json(ServerVersion {
        version: env!("CARGO_PKG_VERSION"),
    })
}
