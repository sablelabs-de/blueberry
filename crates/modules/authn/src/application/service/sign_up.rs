use crate::{
  application::crypto::{self, hasher::HasherError},
  domain::accounts::{
    models::{
      account::NewAccount,
      email::{self, Email},
      password::{self, Password},
      username::{self, Username},
    },
    repository::{AbstractAccountRepository, CreateAccountError},
  },
};

pub struct SignUpCommand {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum SignUpError {
  Username(#[from] username::ValidationError),
  Email(#[from] email::ValidationError),
  Password(#[from] password::ValidationError),
  Hasher(#[from] HasherError),
  CreateAccount(#[from] CreateAccountError),
}

pub async fn sign_up(
  account_repository: &impl AbstractAccountRepository,
  cmd: SignUpCommand,
) -> Result<(), SignUpError> {
  let username = Username::new(&cmd.username)?;
  let email = Email::new(&cmd.email)?;
  let password = Password::new(&cmd.password)?;

  let password_hash = crypto::hasher::hash_password(password).await?;

  let new_user = NewAccount {
    username,
    email,
    password_hash,
  };

  account_repository.create(new_user).await?;

  Ok(())
}
