use chrono::{DateTime, Utc};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::domain::{
  accounts::models::{email::Email, username::Username},
  user_id::UserId,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct AccountId(Uuid);

impl AccountId {
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}

#[derive(FromRow)]
pub struct Account {
  pub user_id: UserId,
  pub email: String,
  pub email_verified: bool,
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

pub struct NewAccount {
  pub username: Username,
  pub email: Email,
  pub password_hash: String,
}
