use sqlx::prelude::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, ValidationError> {
        let username = username.trim();

        if username.is_empty() {
            return Err(ValidationError::Empty);
        }

        if username.len() < 2 {
            return Err(ValidationError::TooShort);
        }

        if username.len() > 32 {
            return Err(ValidationError::TooLong);
        }

        let mut last_was_sep = false;

        for (i, c) in username.chars().enumerate() {
            let is_valid = c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '_'
                || c == '.';

            if !is_valid {
                return Err(ValidationError::InvalidCharacters);
            }

            let is_sep = c == '_' || c == '.';

            if i == 0 && is_sep {
                return Err(ValidationError::StartsWithInvalidChar);
            }

            if is_sep && last_was_sep {
                return Err(ValidationError::ConsecutiveSeparators);
            }

            last_was_sep = is_sep;
        }

        if let Some(last) = username.chars().last()
            && (last == '_' || last == '.')
        {
            return Err(ValidationError::EndsWithInvalidChar);
        }

        Ok(Self(username.to_string()))
    }

    pub fn new_unchecked(username: String) -> Self {
        Self(username)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(thiserror::Error, strum::Display, Debug, PartialEq)]
pub enum ValidationError {
    Empty,
    TooShort,
    TooLong,
    InvalidCharacters,
    StartsWithInvalidChar,
    EndsWithInvalidChar,
    ConsecutiveSeparators,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_username() {
        assert!(Username::new("prouser123").is_ok())
    }

    #[test]
    fn rejects_invalid_usernames() {
        let cases = [
            ("", ValidationError::Empty),
            ("s", ValidationError::TooShort),
            (&format!("{}", "long".repeat(10)), ValidationError::TooLong),
            ("ASDs!", ValidationError::InvalidCharacters),
            (".nick", ValidationError::StartsWithInvalidChar),
            ("nick_", ValidationError::EndsWithInvalidChar),
            ("nick..nick", ValidationError::ConsecutiveSeparators),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Username::new(input),
                Err(expected),
                "failed for input: {input:?}"
            );
        }
    }
}
