use std::{io, path::PathBuf};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    /// No `blueberry.toml` or `blueberry.<profile>.toml` was found.
    #[error(
        "no config file found for profile `{profile}` (expected `blueberry.{profile}.toml`)"
    )]
    NotFound { profile: String },

    /// A config file could not be read.
    #[error("failed to read `{path}`: {source}")]
    Read { path: PathBuf, source: io::Error },

    /// A config file is not valid TOML or does not match the expected shape.
    #[error("failed to parse `{path}`: {source}")]
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },

    /// The config parsed, but the values it holds are not a usable combination.
    #[error("invalid config: {message}")]
    Invalid { message: String },
}

impl ConfigError {
    pub(crate) fn invalid(message: impl Into<String>) -> Self {
        Self::Invalid {
            message: message.into(),
        }
    }
}
