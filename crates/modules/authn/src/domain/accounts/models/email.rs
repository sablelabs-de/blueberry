use sqlx::prelude::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct Email(String);

impl Email {
  /// Creates a new `EmailAddress` while normalizing the email's structure.
  pub fn new(email: &str) -> Result<Self, ValidationError> {
    let email = email.to_lowercase();

    if email.is_empty() || email.chars().all(|c| c.is_whitespace()) {
      return Err(ValidationError::Empty);
    }

    // This check is simplified wrt the full RFC
    let (local, domain) =
      email.split_once('@').ok_or(ValidationError::MissingAt)?;

    if domain.contains('@') {
      return Err(ValidationError::InvalidStructure);
    }

    if local.len() > 64 || domain.len() > 255 {
      return Err(ValidationError::TooLong);
    }

    if local.is_empty() || domain.is_empty() {
      return Err(ValidationError::InvalidStructure);
    }

    if !domain.contains('.') {
      return Err(ValidationError::InvalidDomain);
    }

    if domain.starts_with('.') || domain.ends_with('.') {
      return Err(ValidationError::InvalidDomain);
    }

    if domain.contains("..") {
      return Err(ValidationError::InvalidDomain);
    }

    Ok(Self(email))
  }

  /// Creates a new `EmailAddress` without checking the email's structure.
  ///
  /// Use this when the email is already known to be valid, for example when
  /// reading from the database.
  pub fn new_unchecked(email: String) -> Self {
    Self(email)
  }
}

impl AsRef<str> for Email {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl From<Email> for String {
  fn from(addr: Email) -> Self {
    addr.0
  }
}

#[derive(thiserror::Error, strum::Display, Debug, PartialEq)]
pub enum ValidationError {
  Empty,
  TooLong,
  MissingAt,
  InvalidStructure,
  InvalidDomain,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_valid_email() {
    assert!(Email::new("test@test.com").is_ok())
  }

  #[test]
  fn rejects_invalid_emails() {
    let cases = [
      ("", ValidationError::Empty),
      (
        &format!("{}@{}.com", "local".repeat(100), "domain".repeat(100)),
        ValidationError::TooLong,
      ),
      ("mac.com", ValidationError::MissingAt),
      ("john@other@example.com", ValidationError::InvalidStructure),
      ("alice@..com", ValidationError::InvalidDomain),
    ];

    for (input, expected) in cases {
      assert_eq!(
        Email::new(input),
        Err(expected),
        "failed for input: {input:?}"
      );
    }
  }
}
