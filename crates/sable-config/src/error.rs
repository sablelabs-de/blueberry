use std::{io, path::PathBuf};

use thiserror::Error;

/// Errors that can occur while loading configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
  /// No `sable.toml` or `sable.<profile>.toml` was found.
  #[error("no config file found for profile `{profile}` (expected `sable.{profile}.toml`)")]
  NotFound { profile: String },

  /// A config file could not be read.
  #[error("failed to read `{path}`: {source}")]
  Read { path: PathBuf, source: io::Error },

  /// A config file is not valid TOML or does not match the expected shape.
  #[error("failed to parse `{path}`: {source}")]
  Parse { path: PathBuf, source: toml::de::Error },
}
