use serde::Deserialize;

use super::{DatabaseConfig, ServerConfig};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Environment {
    #[default]
    Development,
    Production,
    Staging,
}
