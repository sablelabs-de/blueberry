pub mod password {
  use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
  };
  use tokio::task;

  use crate::authn::domain::accounts::models::password::Password;

  pub const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7wd0Xq7M3hK5gGHg$e4iQkE888c37P/D2Z/jT5WjX+aZ5J5s5s5s5s5s5s5s";

  #[derive(thiserror::Error, strum::Display, Debug)]
  pub enum PasswordHasherError {
    /// The password hash format or parameters are invalid.
    InvalidFormat,
    /// The password does not match the provided hash.
    PasswordMismatch,
    /// An error occurred during internal cryptographic processing or parameter constraints.
    InternalError,
  }

  fn hash_sync(password: Password) -> Result<String, PasswordHasherError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
      .hash_password(password.as_ref().as_bytes(), &salt)
      .map_err(|_| PasswordHasherError::InternalError)?
      .to_string();

    Ok(hash)
  }

  pub async fn hash(password: Password) -> Result<String, PasswordHasherError> {
    task::spawn_blocking(move || hash_sync(password))
      .await
      .map_err(|_| PasswordHasherError::InternalError)?
  }

  fn verify_sync(
    password: Password,
    password_hash: String,
  ) -> Result<(), PasswordHasherError> {
    let parsed_hash = PasswordHash::new(&password_hash)
      .map_err(|_| PasswordHasherError::InvalidFormat)?;

    Argon2::default()
      .verify_password(password.as_ref().as_bytes(), &parsed_hash)
      .map_err(|e| match e {
        argon2::password_hash::Error::Password => {
          PasswordHasherError::PasswordMismatch
        }
        _ => PasswordHasherError::InternalError,
      })?;

    Ok(())
  }

  pub async fn verify(
    password: Password,
    password_hash: String,
  ) -> Result<(), PasswordHasherError> {
    task::spawn_blocking(move || verify_sync(password, password_hash))
      .await
      .map_err(|_| PasswordHasherError::InternalError)?
  }
}

pub mod refresh_token {
  use sha2::{Digest, Sha256};

  use crate::authn::domain::sessions::models::refresh_validator::{
    RefreshValidator, RefreshValidatorHash,
  };

  pub fn hash(validator: &RefreshValidator) -> RefreshValidatorHash {
    let hash_array: [u8; 32] = Sha256::digest(validator).into();
    hash_array.into()
  }
}
