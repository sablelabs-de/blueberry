use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::authn::domain::{
  sessions::models::{
    refresh_selector::RefreshSelector, refresh_validator::RefreshValidatorHash,
  },
  user_id::UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, FromRow)]
#[sqlx(transparent)]
pub struct SessionId(Uuid);

impl SessionId {
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

#[derive(FromRow)]
pub struct Session {
  id: SessionId,
  user_id: UserId,

  refresh_selector: RefreshSelector,
  refresh_validator_hash: RefreshValidatorHash,
  refresh_generation: i32,

  ip_address: Option<IpAddr>,
  country: Option<String>,
  region: Option<String>,
  city: Option<String>,

  user_agent: Option<String>,
  operating_system: Option<String>,
  platform: Option<String>,

  idle_expires_at: DateTime<Utc>,
  absolute_expires_at: DateTime<Utc>,

  revoked_at: Option<DateTime<Utc>>,
  revocation_reason: Option<String>,

  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

pub struct NewSession {
  pub id: SessionId,
  pub user_id: UserId,
  pub refresh_selector: RefreshSelector,
  pub refresh_validator_hash: RefreshValidatorHash,
  pub idle_ttl: Duration,
  pub absolute_ttl: Duration,
}
