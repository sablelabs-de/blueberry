use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use tokio::task;

use crate::domain::accounts::models::password::Password;

pub const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7wd0Xq7M3hK5gGHg$e4iQkE888c37P/D2Z/jT5WjX+aZ5J5s5s5s5s5s5s5s";

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum HasherError {
    /// The password hash format or parameters are invalid.
    InvalidFormat,
    /// The password does not match the provided hash.
    PasswordMismatch,
    /// An error occurred during internal cryptographic processing or parameter constraints.
    InternalError,
}

fn hash_password_sync(password: Password) -> Result<String, HasherError> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_ref().as_bytes(), &salt)
        .map_err(|_| HasherError::InternalError)?
        .to_string();

    Ok(hash)
}

pub async fn hash_password(password: Password) -> Result<String, HasherError> {
    task::spawn_blocking(move || hash_password_sync(password))
        .await
        .map_err(|_| HasherError::InternalError)?
}

fn verify_password_sync(password: Password, password_hash: String) -> Result<(), HasherError> {
    let parsed_hash = PasswordHash::new(&password_hash).map_err(|_| HasherError::InvalidFormat)?;

    Argon2::default()
        .verify_password(password.as_ref().as_bytes(), &parsed_hash)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => HasherError::PasswordMismatch,
            _ => HasherError::InternalError,
        })?;

    Ok(())
}

pub async fn verify_password(password: Password, password_hash: String) -> Result<(), HasherError> {
    task::spawn_blocking(move || verify_password_sync(password, password_hash))
        .await
        .map_err(|_| HasherError::InternalError)?
}
