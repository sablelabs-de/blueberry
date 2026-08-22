use async_trait::async_trait;
use derive_more::Display;

use crate::{
  authn::domain::access_tokens::models::access_token::AccessTokenCreation,
  shared::errors::UnexpectedError,
};

#[derive(thiserror::Error, Display, Debug)]
pub enum CreateAccessTokenError {
  Unexpected(#[from] UnexpectedError),
}

#[async_trait]
pub trait AbstractAccessTokenRepository {
  async fn create(
    &self,
    new_access_token: AccessTokenCreation,
  ) -> Result<(), CreateAccessTokenError>;
}
