use chrono::Duration;

use crate::authn::{
    application::crypto::hasher,
    domain::sessions::{
        models::refresh_token::{self, RefreshToken, RefreshTokenRotation},
        repository::{AbstractSessionRepository, RotateRefreshTokenError},
    },
};

pub struct RefreshSessionCommand {
    pub refresh_token: String,
}

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum RefreshSessionError {
    RefreshToken(#[from] refresh_token::ParseError),
    RotateRefreshToken(#[from] RotateRefreshTokenError),
}

pub async fn refresh_session(
    session_repository: &impl AbstractSessionRepository,
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
    let session_id = session_repository.rotate_refresh_token(rotation).await?;

    Ok(())
}
