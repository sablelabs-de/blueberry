use std::path::PathBuf;

use serde::Deserialize;

use crate::ConfigError;

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub persistent: PersistentDatabaseConfig,
    pub ephemeral: EphemeralDatabaseConfig,
}

impl DatabaseConfig {
    /// Checks that the table for every selected backend is present.
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.persistent.backend()?;
        self.ephemeral.backend()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PersistentDatabaseConfig {
    pub backend: PersistentDatabaseBackend,
    pub sqlite: Option<SqliteConfig>,
    pub postgres: Option<PostgresConfig>,
    pub mysql: Option<MySqlConfig>,
}

impl PersistentDatabaseConfig {
    /// Returns the configuration for the selected backend.
    pub fn backend(&self) -> Result<PersistentBackend<'_>, ConfigError> {
        match self.backend {
            PersistentDatabaseBackend::Sqlite => {
                let sqlite = self.sqlite.as_ref().ok_or_else(|| {
                    missing_table("database.persistent", "sqlite")
                })?;
                Ok(PersistentBackend::Sqlite(sqlite))
            }
            PersistentDatabaseBackend::Postgres => {
                let postgres = self.postgres.as_ref().ok_or_else(|| {
                    missing_table("database.persistent", "postgres")
                })?;
                Ok(PersistentBackend::Postgres(postgres))
            }
            PersistentDatabaseBackend::MySql => {
                let mysql = self.mysql.as_ref().ok_or_else(|| {
                    missing_table("database.persistent", "mysql")
                })?;
                Ok(PersistentBackend::MySql(mysql))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PersistentDatabaseBackend {
    Sqlite,
    Postgres,
    MySql,
}

#[derive(Debug, Clone, Copy)]
pub enum PersistentBackend<'a> {
    Sqlite(&'a SqliteConfig),
    Postgres(&'a PostgresConfig),
    MySql(&'a MySqlConfig),
}

#[derive(Debug, Clone, Deserialize)]
pub struct EphemeralDatabaseConfig {
    pub backend: EphemeralDatabaseBackend,
    /// Only read when the backend is `redis`; `memory` needs nothing.
    pub redis: Option<RedisConfig>,
}

impl EphemeralDatabaseConfig {
    /// Returns the configuration for the selected backend.
    pub fn backend(&self) -> Result<EphemeralBackend<'_>, ConfigError> {
        match self.backend {
            EphemeralDatabaseBackend::Memory => Ok(EphemeralBackend::Memory),
            EphemeralDatabaseBackend::Redis => {
                let redis = self.redis.as_ref().ok_or_else(|| {
                    missing_table("database.ephemeral", "redis")
                })?;
                Ok(EphemeralBackend::Redis(redis))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EphemeralDatabaseBackend {
    Redis,
    Memory,
}

#[derive(Debug, Clone, Copy)]
pub enum EphemeralBackend<'a> {
    Redis(&'a RedisConfig),
    Memory,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SqliteConfig {
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub database: u32,
    #[serde(default)]
    pub tls: bool,
}

fn missing_table(section: &str, backend: &str) -> ConfigError {
    ConfigError::invalid(format!(
        "the backend is `{backend}` but there is no `[{section}.{backend}]` table"
    ))
}
