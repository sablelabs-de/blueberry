use async_trait::async_trait;
use redis::{AsyncTypedCommands, aio::ConnectionManager};

use crate::{
    authn::domain::{
        access_tokens::{
            models::access_token::{
                AccessTokenCreation, AccessTokenHash, AccessTokenRotation,
            },
            repository::{
                AbstractAccessTokenRepository, CreateAccessTokenError,
                RotateAccessTokenError,
            },
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
    fn token_key(token: &AccessTokenHash) -> String {
        format!("auth:{token}")
    }

    fn user_tokens_key(user_id: &UserId) -> String {
        format!("user:{user_id}:tokens")
    }

    fn session_token_key(session_id: &SessionId) -> String {
        format!("session:{session_id}:token")
    }
}

#[async_trait]
impl AbstractAccessTokenRepository for AccessTokenRepository {
    async fn create(
        &self,
        access_token_creation: AccessTokenCreation,
    ) -> Result<(), CreateAccessTokenError> {
        let mut conn = self.conn.clone();

        let seconds = access_token_creation.ttl.as_seconds_f64() as u64;

        // access_token_hash -> (user_id, session_id) stored as "<user_id>.<session_id>"
        let key = Self::token_key(&access_token_creation.access_token_hash);
        let value = format!(
            "{}.{}",
            access_token_creation.user_id, access_token_creation.session_id
        );
        conn.set_ex(key, value, seconds)
            .await
            .map_err(UnexpectedError::new)?;

        // session_id -> access_token_hash
        let session_key =
            Self::session_token_key(&access_token_creation.session_id);
        let session_value = access_token_creation.access_token_hash.to_string();
        conn.set_ex(session_key, session_value, seconds)
            .await
            .map_err(UnexpectedError::new)?;

        Ok(())
    }

    async fn rotate(
        &self,
        access_token_rotation: AccessTokenRotation,
    ) -> Result<(), RotateAccessTokenError> {
        let mut conn = self.conn.clone();

        let seconds = access_token_rotation.ttl.as_seconds_f64() as u64;

        let session_key =
            Self::session_token_key(&access_token_rotation.session_id);
        let session_value = access_token_rotation.access_token_hash.to_string();
        let old_access_token_hash: Option<String> = redis::cmd("SET")
            .arg(&session_key)
            .arg(&session_value)
            .arg("GET")
            .arg("EX")
            .arg(seconds)
            .arg("XX")
            .query_async(&mut conn)
            .await
            .map_err(UnexpectedError::new)?;
        let old_access_token_hash: AccessTokenHash = old_access_token_hash
            .ok_or(RotateAccessTokenError::NotFound)?
            .parse()
            .map_err(UnexpectedError::new)?;

        let old_token_key = Self::token_key(&old_access_token_hash);
        conn.del(old_token_key)
            .await
            .map_err(UnexpectedError::new)?;

        // access_token_hash -> (user_id, session_id) stored as "<user_id>.<session_id>"
        let key = Self::token_key(&access_token_rotation.access_token_hash);
        let value = format!(
            "{}.{}",
            access_token_rotation.user_id, access_token_rotation.session_id
        );
        conn.set_ex(key, value, seconds)
            .await
            .map_err(UnexpectedError::new)?;

        Ok(())
    }
}
