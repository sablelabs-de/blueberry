use sable_server::web;

/// Launches the server, and will exit with an error on failure.
#[rocket::launch]
async fn rocket() -> _ {
  match web().await {
    Ok(rocket) => rocket,
    Err(e) => {
      eprintln!("Failed to initialize Rocket: {e}");
      std::process::exit(1);
    }
  }
}
