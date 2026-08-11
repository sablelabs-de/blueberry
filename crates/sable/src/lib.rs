use std::error::Error;

/// Builds the Rocket application, mounting our api routes.
///
/// Returns an error if the application fails to initialize.
pub async fn web() -> Result<rocket::Rocket<rocket::Build>, Box<dyn Error>> {
  Ok(sable_api::routes::mount(rocket::build()))
}
