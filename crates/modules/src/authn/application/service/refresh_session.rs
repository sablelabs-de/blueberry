use chrono::Duration;
use derive_more::Display;

use crate::authn::{
    application::crypto::hasher,
    domain::{
        access_tokens::{
            models::access_token::{AccessToken, AccessTokenRotation},
            repository::{
                AbstractAccessTokenRepository, RotateAccessTokenError,
            },
        },
        sessions::{
            models::refresh_token::{self, RefreshToken, RefreshTokenRotation},
            repository::{AbstractSessionRepository, RotateRefreshTokenError},
        },
    },
};

pub struct RefreshSessionCommand {
    pub refresh_token: String,
}

#[derive(thiserror::Error, Display, Debug)]
pub enum RefreshSessionError {
    RefreshToken(#[from] refresh_token::ParseError),
    RotateRefreshToken(#[from] RotateRefreshTokenError),
    RotateAccessToken(#[from] RotateAccessTokenError),
}

pub async fn refresh_session(
    session_repository: &impl AbstractSessionRepository,
    access_token_repository: &impl AbstractAccessTokenRepository,
    cmd: RefreshSessionCommand,
) -> Result<(), RefreshSessionError> {
    let presented_refresh_token = RefreshToken::parse(&cmd.refresh_token)?;

    let rotated_refresh_token = presented_refresh_token.rotate();
    let rotation = RefreshTokenRotation {
        presented_selector: presented_refresh_token.selector(),
        presented_validator_hash: hasher::refresh_token::hash(
            presented_refresh_token.validator(),
        ),
        new_validator_hash: hasher::refresh_token::hash(
            rotated_refresh_token.validator(),
        ),
        idle_ttl: Duration::days(30), // TODO: should be from config
    };
    let (session_id, user_id) =
        session_repository.rotate_refresh_token(rotation).await?;

    let access_token = AccessToken::generate();
    let access_token_rotation = AccessTokenRotation {
        access_token_hash: hasher::access_token::hash(access_token),
        session_id,
        user_id,
        ttl: Duration::minutes(10), // TODO: should be from config
    };
    access_token_repository
        .rotate(access_token_rotation)
        .await?;

    Ok(())
}
