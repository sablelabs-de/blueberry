use std::error::Error;

// Build the rocket application.
pub async fn web() -> Result<rocket::Rocket<rocket::Build>, Box<dyn Error>> {
  Ok(rocket::build())
}
