#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
  pub fn new(password: &str) -> Result<Self, ValidationError> {
    if password.is_empty() || password.chars().all(|c| c.is_whitespace()) {
      return Err(ValidationError::Empty);
    }

    if password.len() < 8 {
      return Err(ValidationError::TooShort);
    }

    if password.len() > 64 {
      return Err(ValidationError::TooLong);
    }

    if !password.chars().any(|c| c.is_uppercase()) {
      return Err(ValidationError::MissingUpper);
    }

    if !password.chars().any(|c| c.is_lowercase()) {
      return Err(ValidationError::MissingLower);
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
      return Err(ValidationError::MissingDigit);
    }

    if !password
      .chars()
      .any(|c| c.is_ascii_graphic() && !c.is_alphanumeric())
    {
      return Err(ValidationError::MissingSpecial);
    }

    Ok(Self(password.to_string()))
  }

  pub fn new_unchecked(password: String) -> Self {
    Self(password)
  }
}

impl AsRef<str> for Password {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl From<Password> for String {
  fn from(addr: Password) -> Self {
    addr.0
  }
}

#[derive(thiserror::Error, strum::Display, Debug, PartialEq)]
pub enum ValidationError {
  Empty,
  TooShort,
  TooLong,
  MissingUpper,
  MissingLower,
  MissingDigit,
  MissingSpecial,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_valid_password() {
    assert!(Password::new("ThisIsSamplePassword!0987654321").is_ok())
  }

  #[test]
  fn rejects_invalid_passwords() {
    let cases = [
      ("", ValidationError::Empty),
      ("short", ValidationError::TooShort),
      (&format!("{}", "long".repeat(30)), ValidationError::TooLong),
      ("nanannananannananana", ValidationError::MissingUpper),
      ("NANANANNAANNANANAA", ValidationError::MissingLower),
      ("nananaNANANANAnanana", ValidationError::MissingDigit),
      ("nanannaNANANANAnananan3", ValidationError::MissingSpecial),
    ];

    for (input, expected) in cases {
      assert_eq!(
        Password::new(input),
        Err(expected),
        "failed for input: {input:?}"
      );
    }
  }
}
