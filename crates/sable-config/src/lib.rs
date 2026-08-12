//! Typed configuration for Sable, loaded from TOML files.
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
