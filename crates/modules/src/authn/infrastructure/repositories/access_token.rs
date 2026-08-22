use async_trait::async_trait;
use redis::{AsyncTypedCommands, aio::ConnectionManager};

use crate::{
  authn::domain::{
    access_tokens::{
      models::access_token::AccessTokenCreation,
      repository::{AbstractAccessTokenRepository, CreateAccessTokenError},
    },
    sessions::models::session::SessionId,
    user_id::UserId,
  },
  shared::errors::UnexpectedError,
};

struct AccessTokenRepository {
  conn: ConnectionManager,
}

impl AccessTokenRepository {
  fn token_key(token: &str) -> String {
    format!("auth:{token}")
  }

  fn user_tokens_key(user_id: UserId) -> String {
    format!("user:{user_id}:tokens")
  }

  fn session_tokens_key(session_id: SessionId) -> String {
    format!("session:{session_id}:tokens")
  }
}

#[async_trait]
impl AbstractAccessTokenRepository for AccessTokenRepository {
  async fn create(
    &self,
    access_token_creation: AccessTokenCreation,
  ) -> Result<(), CreateAccessTokenError> {
    let mut conn = self.conn.clone();

    let auth_token = access_token_creation.access_token.encode();
    let key = Self::token_key(&auth_token);
    let member = access_token_creation.session_id.to_string();

    conn.sadd(key, member).await.map_err(UnexpectedError::new)?;

    todo!();

    Ok(())
  }
}
