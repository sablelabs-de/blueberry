use std::fmt::Debug;

use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use derive_more::{AsRef, From};
use sqlx::prelude::Type;
use subtle::{Choice, ConstantTimeEq};

use crate::authn::domain::sessions::models::refresh_token::ParseError;

#[derive(AsRef)]
#[as_ref([u8])]
pub struct RefreshValidator([u8; 32]);

impl RefreshValidator {
    pub fn generate() -> Self {
        let mut bytes = [0; 32];
        rand::fill(&mut bytes);
        Self(bytes)
    }

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let decoded = BASE64_URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|_| ParseError::MalformedValidator)?;
        let bytes: [u8; 32] = decoded
            .try_into()
            .map_err(|_| ParseError::MalformedValidator)?;
        Ok(Self(bytes))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Type, From)]
#[sqlx(transparent)]
pub struct RefreshValidatorHash([u8; 32]);

impl RefreshValidatorHash {
    pub fn matches(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl ConstantTimeEq for RefreshValidatorHash {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl Debug for RefreshValidatorHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}
