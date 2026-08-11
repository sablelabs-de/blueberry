use sable::web;

/// Launches the Rocket server, exiting with an error message on failure.
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
