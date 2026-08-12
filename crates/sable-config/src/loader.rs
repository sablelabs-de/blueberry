use std::{env, fs, io, path::Path};

use serde::Deserialize;
use toml::Value;

use crate::{error::ConfigError, structs::Config};

const PROFILE_ENV: &str = "SABLE_PROFILE";
const DEFAULT_PROFILE: &str = "development";

/// Returns the active profile.
pub fn profile() -> String {
  env::var(PROFILE_ENV).unwrap_or_else(|_| DEFAULT_PROFILE.to_owned())
}

impl Config {
  /// Loads the config for the active profile (see [`profile`]).
  pub fn load() -> Result<Self, ConfigError> {
    Self::load_for(&profile())
  }

  /// Loads the config for `profile` from the current directory.
  pub fn load_for(profile: &str) -> Result<Self, ConfigError> {
    Self::load_from(Path::new("."), profile)
  }

  /// Loads the config from `dir`, merging `sable.toml` (shared base, optional)
  /// with `sable.{profile}.toml` (profile overrides) in that order.
  pub fn load_from(dir: &Path, profile: &str) -> Result<Self, ConfigError> {
    let base_path = dir.join("sable.toml");
    let profile_path = dir.join(format!("sable.{profile}.toml"));

    let base = read_optional(&base_path)?;
    let specific = read_optional(&profile_path)?;

    let value = match (base, specific) {
      (Some(mut base), Some(specific)) => {
        merge(&mut base, specific);
        base
      }
      (Some(base), None) => base,
      (None, Some(specific)) => specific,
      (None, None) => {
        return Err(ConfigError::NotFound {
          profile: profile.to_owned(),
        });
      }
    };

    Config::deserialize(value).map_err(|source| ConfigError::Parse {
      path: profile_path,
      source,
    })
  }
}

fn read_optional(path: &Path) -> Result<Option<Value>, ConfigError> {
  let text = match fs::read_to_string(path) {
    Ok(text) => text,
    Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
    Err(source) => {
      return Err(ConfigError::Read {
        path: path.to_owned(),
        source,
      });
    }
  };

  let value = toml::from_str(&text).map_err(|source| ConfigError::Parse {
    path: path.to_owned(),
    source,
  })?;
  Ok(Some(value))
}

fn merge(base: &mut Value, overlay: Value) {
  match (base, overlay) {
    (Value::Table(base), Value::Table(overlay)) => {
      for (key, value) in overlay {
        if base.get(&key).is_some() {
          merge(base.get_mut(&key).expect("key checked above"), value);
        } else {
          base.insert(key, value);
        }
      }
    }
    (base, overlay) => *base = overlay,
  }
}
