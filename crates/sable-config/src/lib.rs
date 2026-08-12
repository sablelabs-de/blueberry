//! Typed configuration for Sable, loaded from TOML.
//!
//! Config is read from `sable.toml`, and `sable.<profile>.toml
//! Config profiles act as overrides over sable.toml and are deep-merged in that order.
//! The active profile  will come from the `SABLE_PROFILE` environment variable,
//! or the --profile flag. The profile defaults  to `development`. :3
//!
//! ```no_run
//! use sable_config::Config;
//!
//! let config = Config::load().expect("failed to load config");
//! println!("{}", config.server.host);
//! ```

mod error;
mod loader;
pub mod structs;

pub use error::ConfigError;
pub use loader::profile;
pub use structs::{Config, Environment, ServerConfig};
