use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use sqlx::prelude::Type;

use crate::authn::domain::sessions::models::refresh_token::ParseError;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct RefreshSelector([u8; 16]);

impl RefreshSelector {
    pub fn generate() -> Self {
        let mut bytes = [0; 16];
        rand::fill(&mut bytes);
        Self(bytes)
    }

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let decoded = BASE64_URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|_| ParseError::MalformedSelector)?;
        let bytes: [u8; 16] = decoded
            .try_into()
            .map_err(|_| ParseError::MalformedSelector)?;
        Ok(Self(bytes))
    }
}
