use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use chrono::Duration;

use crate::authn::domain::sessions::models::session::SessionId;

pub struct AccessToken([u8; 32]);

impl AccessToken {
    pub fn generate() -> Self {
        let mut bytes = [0; 32];
        rand::fill(&mut bytes);
        Self(bytes)
    }

    /// Encode to Base64.
    pub fn encode(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(&self.0)
    }
}

pub struct AccessTokenCreation {
    pub access_token: AccessToken,
    pub session_id: SessionId,
    pub ttl: Duration,
}
