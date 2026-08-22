use async_trait::async_trait;
use derive_more::Display;

use crate::{
  authn::domain::sessions::models::{
    refresh_token::RefreshTokenRotation,
    session::{NewSession, SessionId},
  },
  shared::errors::UnexpectedError,
};

#[derive(thiserror::Error, Display, Debug)]
pub enum CreateSessionError {
  Unexpected(#[from] UnexpectedError),
}

#[derive(thiserror::Error, Display, Debug)]
pub enum RotateRefreshTokenError {
  InvalidToken,
  ReuseDetectedAndRevoked,
  Unexpected(#[from] UnexpectedError),
}

#[async_trait]
pub trait AbstractSessionRepository {
  async fn create(
    &self,
    new_session: NewSession,
  ) -> Result<(), CreateSessionError>;

  async fn rotate_refresh_token(
    &self,
    rotation: RefreshTokenRotation,
  ) -> Result<SessionId, RotateRefreshTokenError>;
}
