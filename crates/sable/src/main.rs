use sable::web;

/// Runs the axum server, exiting with an error message on failure.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
  axum::serve(listener, web()).await?;
  Ok(())
}
