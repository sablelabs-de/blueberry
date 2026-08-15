use serde::Deserialize;

use super::ServerConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub environment: Environment,
  pub server: ServerConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Environment {
  #[default]
  Development,
  Production,
  Staging,
}
