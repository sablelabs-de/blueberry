use derive_more::Display;

use crate::authn::{
  application::crypto::hasher::{self, password::PasswordHasherError},
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

#[derive(thiserror::Error, Display, Debug)]
pub enum SignUpError {
  Username(#[from] username::ValidationError),
  Email(#[from] email::ValidationError),
  Password(#[from] password::ValidationError),
  PasswordHasher(#[from] PasswordHasherError),
  CreateAccount(#[from] CreateAccountError),
}

pub async fn sign_up(
  account_repository: &impl AbstractAccountRepository,
  cmd: SignUpCommand,
) -> Result<(), SignUpError> {
  let username = Username::new(&cmd.username)?;
  let email = Email::new(&cmd.email)?;
  let password = Password::new(&cmd.password)?;

  let password_hash = hasher::password::hash(password).await?;
  let new_user = NewAccount {
    username,
    email,
    password_hash,
  };
  account_repository.create(new_user).await?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::authn::domain::accounts::repository::MockAbstractAccountRepository;

  use super::*;

  // TODO: this test is mostly useless, but its a showcase (template) for future tests
  #[tokio::test]
  async fn sign_up_success() {
    let mut mock_account_repository = MockAbstractAccountRepository::new();
    mock_account_repository
      .expect_create()
      .returning(|_| Ok(()));

    let cmd = SignUpCommand {
      username: "test".to_owned(),
      email: "test@test.test".to_owned(),
      password: "testTEST1!".to_owned(),
    };

    sign_up(&mock_account_repository, cmd).await.unwrap();
  }
}
