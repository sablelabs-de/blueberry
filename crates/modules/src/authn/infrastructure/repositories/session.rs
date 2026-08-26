use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    authn::domain::{
        sessions::{
            models::{
                refresh_selector::RefreshSelector,
                refresh_token::RefreshTokenRotation,
                refresh_validator::RefreshValidatorHash,
                session::{NewSession, SessionId},
            },
            repository::{
                AbstractSessionRepository, CreateSessionError,
                RotateRefreshTokenError,
            },
        },
        user_id::UserId,
    },
    shared::errors::UnexpectedError,
};

pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AbstractSessionRepository for SessionRepository {
    async fn create(
        &self,
        new_session: NewSession,
    ) -> Result<(), CreateSessionError> {
        sqlx::query!(
      r#"
              INSERT INTO "sessions"
              (id, user_id, refresh_selector, refresh_validator_hash, idle_expires_at, absolute_expires_at)
              VALUES
              ($1, $2, $3, $4, NOW() + $5::interval, NOW() + $6::interval)
          "#,
      new_session.id as SessionId,
      new_session.user_id as UserId,
      new_session.refresh_selector as RefreshSelector,
      new_session.refresh_validator_hash as RefreshValidatorHash,
      new_session.idle_ttl as _,
      new_session.absolute_ttl as _
    )
    .execute(&self.pool)
    .await
    .map_err(UnexpectedError::new)?;

        Ok(())
    }

    async fn rotate_refresh_token(
        &self,
        rotation: RefreshTokenRotation,
    ) -> Result<SessionId, RotateRefreshTokenError> {
        let mut tx = self.pool.begin().await.map_err(UnexpectedError::new)?;

        let row = sqlx::query!(
      r#"
            SELECT
                id AS "id: SessionId",
                refresh_validator_hash AS "refresh_validator_hash: RefreshValidatorHash"
            FROM "sessions"
            WHERE
                refresh_selector = $1
                AND revoked_at IS NULL
                AND idle_expires_at > NOW()
                AND absolute_expires_at > NOW()
            FOR UPDATE
        "#,
      rotation.presented_selector as RefreshSelector
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(UnexpectedError::new)?;

        let Some(row) = row else {
            tx.rollback().await.map_err(UnexpectedError::new)?;
            return Err(RotateRefreshTokenError::InvalidToken);
        };

        let (session_id, stored_validator_hash) =
            (row.id, row.refresh_validator_hash);

        if !rotation
            .presented_validator_hash
            .matches(&stored_validator_hash)
        {
            sqlx::query!(
                r#"
            UPDATE sessions
            SET
                revoked_at = NOW(),
                revocation_reason = 'refresh_token_reuse',
                updated_at = NOW()
            WHERE id = $1
        "#,
                session_id as SessionId,
            )
            .execute(&mut *tx)
            .await
            .map_err(UnexpectedError::new)?;

            tx.commit().await.map_err(UnexpectedError::new)?;

            return Err(RotateRefreshTokenError::ReuseDetectedAndRevoked);
        }

        sqlx::query!(
            r#"
        UPDATE sessions
        SET
            refresh_validator_hash = $2,
            refresh_generation = refresh_generation + 1,
            updated_at = NOW(),
            idle_expires_at = LEAST(
                NOW() + $3::interval,
                absolute_expires_at
            )
        WHERE id = $1
      "#,
            session_id as SessionId,
            rotation.new_validator_hash as RefreshValidatorHash,
            rotation.idle_ttl as _
        )
        .execute(&mut *tx)
        .await
        .map_err(UnexpectedError::new)?;

        tx.commit().await.map_err(UnexpectedError::new)?;

        Ok(session_id)
    }
}
