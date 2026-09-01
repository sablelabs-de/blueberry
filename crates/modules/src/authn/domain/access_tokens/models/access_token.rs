use std::str::FromStr;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::Duration;
use derive_more::{AsRef, Display, From};

use crate::authn::domain::{
    sessions::models::session::SessionId, user_id::UserId,
};

#[derive(Clone, Copy, AsRef)]
#[as_ref([u8])]
pub struct AccessToken([u8; 32]);

impl AccessToken {
    pub fn generate() -> Self {
        let mut bytes = [0; 32];
        rand::fill(&mut bytes);
        Self(bytes)
    }
}

#[derive(From, Display)]
#[display("{}", BASE64_URL_SAFE_NO_PAD.encode(_0))]
pub struct AccessTokenHash([u8; 32]);

impl FromStr for AccessTokenHash {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = BASE64_URL_SAFE_NO_PAD.decode(s)?;

        let bytes: [u8; 32] =
            bytes.try_into().map_err(|_| ParseError::InvalidLength)?;

        Ok(AccessTokenHash(bytes))
    }
}

#[derive(thiserror::Error, Display, Debug, PartialEq)]
pub enum ParseError {
    InvalidBase64(#[from] base64::DecodeError),
    InvalidLength,
}

pub struct AccessTokenCreation {
    pub access_token_hash: AccessTokenHash,
    pub session_id: SessionId,
    pub user_id: UserId,
    pub ttl: Duration,
}

pub struct AccessTokenRotation {
    pub access_token_hash: AccessTokenHash,
    pub session_id: SessionId,
    pub user_id: UserId,
    pub ttl: Duration,
}
