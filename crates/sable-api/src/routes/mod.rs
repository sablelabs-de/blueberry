use rocket::{get, routes, Build, Rocket};

// this is temporary
#[get("/_sable")]
fn sable() -> &'static str {
  "sable"
}

/// Mounts all `sable-api` routes on the given Rocket instance.
pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
  rocket.mount("/", routes![sable])
}
