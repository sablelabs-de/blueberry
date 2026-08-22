use crate::authn::{
  application::service::{
    log_in::{LogInCommand, LogInError},
    refresh_session::{RefreshSessionCommand, RefreshSessionError},
    sign_up::{SignUpCommand, SignUpError},
  },
  domain::{
    access_tokens::repository::AbstractAccessTokenRepository,
    accounts::repository::AbstractAccountRepository,
    sessions::repository::AbstractSessionRepository,
  },
};

pub mod log_in;
pub mod refresh_session;
pub mod sign_up;

pub struct AuthNService<
  A: AbstractAccountRepository,
  B: AbstractSessionRepository,
  C: AbstractAccessTokenRepository,
> {
  account_repository: A,
  session_repository: B,
  access_token_repository: C,
}

impl<
  A: AbstractAccountRepository,
  B: AbstractSessionRepository,
  C: AbstractAccessTokenRepository,
> AuthNService<A, B, C>
{
  pub fn new(
    account_repository: A,
    session_repository: B,
    access_token_repository: C,
  ) -> Self {
    Self {
      account_repository,
      session_repository,
      access_token_repository,
    }
  }

  pub async fn sign_up(&self, cmd: SignUpCommand) -> Result<(), SignUpError> {
    sign_up::sign_up(&self.account_repository, cmd).await
  }

  async fn log_in(&self, cmd: LogInCommand) -> Result<(), LogInError> {
    log_in::log_in(
      &self.account_repository,
      &self.session_repository,
      &self.access_token_repository,
      cmd,
    )
    .await
  }

  async fn refresh_session(
    &self,
    cmd: RefreshSessionCommand,
  ) -> Result<(), RefreshSessionError> {
    refresh_session::refresh_session(&self.session_repository, cmd).await
  }
}
