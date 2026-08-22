use chrono::Duration;
use derive_more::Display;

use crate::authn::domain::sessions::models::{
  refresh_selector::RefreshSelector,
  refresh_validator::{RefreshValidator, RefreshValidatorHash},
};

pub struct RefreshToken {
  selector: RefreshSelector,
  validator: RefreshValidator,
}

impl RefreshToken {
  pub fn generate() -> Self {
    Self {
      selector: RefreshSelector::generate(),
      validator: RefreshValidator::generate(),
    }
  }

  pub fn parse(token: &str) -> Result<Self, ParseError> {
    let (selector, validator) =
      token.split_once('.').ok_or(ParseError::InvalidStructure)?;

    Ok(Self {
      selector: RefreshSelector::parse(selector)?,
      validator: RefreshValidator::parse(validator)?,
    })
  }

  pub fn rotate(&self) -> Self {
    Self {
      selector: self.selector,
      validator: RefreshValidator::generate(),
    }
  }

  pub fn selector(&self) -> RefreshSelector {
    self.selector
  }

  pub fn validator(&self) -> &RefreshValidator {
    &self.validator
  }
}

pub struct RefreshTokenRotation {
  pub presented_selector: RefreshSelector,
  pub presented_validator_hash: RefreshValidatorHash,

  pub new_validator_hash: RefreshValidatorHash,

  pub idle_ttl: Duration,
}

#[derive(thiserror::Error, Display, Debug, PartialEq)]
pub enum ParseError {
  InvalidStructure,
  MalformedSelector,
  MalformedValidator,
}
