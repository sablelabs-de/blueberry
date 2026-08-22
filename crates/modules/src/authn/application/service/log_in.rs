use chrono::Duration;
use derive_more::Display;

use crate::authn::{
  application::crypto::hasher::{self, password::PasswordHasherError},
  domain::{
    access_tokens::{
      models::access_token::{AccessToken, AccessTokenCreation},
      repository::{AbstractAccessTokenRepository, CreateAccessTokenError},
    },
    accounts::{
      models::{
        email::{self, Email},
        password::{self, Password},
      },
      repository::{AbstractAccountRepository, FindAccountError},
    },
    sessions::{
      models::{
        refresh_token::RefreshToken,
        session::{NewSession, SessionId},
      },
      repository::{AbstractSessionRepository, CreateSessionError},
    },
  },
};

pub struct LogInCommand {
  pub email: String,
  pub password: String,
}

#[derive(thiserror::Error, Display, Debug)]
pub enum LogInError {
  Email(#[from] email::ValidationError),
  Password(#[from] password::ValidationError),
  PasswordHasher(#[from] PasswordHasherError),
  FindAccount(#[from] FindAccountError),
  CreateSession(#[from] CreateSessionError),
  CreateAccessToken(#[from] CreateAccessTokenError),
}

pub async fn log_in(
  account_repository: &impl AbstractAccountRepository,
  session_repository: &impl AbstractSessionRepository,
  access_token_repository: &impl AbstractAccessTokenRepository,
  cmd: LogInCommand,
) -> Result<(), LogInError> {
  let email = Email::new(&cmd.email)?;
  let password = Password::new(&cmd.password)?;

  // Do not return an error early, when the account is not found,
  // to prevent revealing whether the account exists.
  let account = account_repository.find_account_by_email(email).await;
  let password_hash = match account {
    Ok(ref account) => &account.password_hash,
    Err(FindAccountError::NotFound) => hasher::password::DUMMY_HASH,
    Err(error) => return Err(error.into()),
  }
  .to_owned();
  let verification = hasher::password::verify(password, password_hash).await;
  let account = account?;
  verification?;

  let session_id = SessionId::new();
  let refresh_token = RefreshToken::generate();
  let new_session = NewSession {
    id: session_id,
    user_id: account.user_id,
    refresh_selector: refresh_token.selector(),
    refresh_validator_hash: hasher::refresh_token::hash(
      refresh_token.validator(),
    ),
    idle_ttl: Duration::days(30), // TODO: should be from config
    absolute_ttl: Duration::days(360),
  };
  session_repository.create(new_session).await?;

  let access_token = AccessToken::generate();
  let access_token_creation = AccessTokenCreation {
    access_token,
    session_id,
    ttl: Duration::minutes(10), // TODO: should be from config
  };
  access_token_repository
    .create(access_token_creation)
    .await?;

  Ok(())
}
