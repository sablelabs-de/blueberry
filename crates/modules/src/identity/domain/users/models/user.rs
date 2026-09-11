use chrono::{DateTime, Utc};
use derive_more::Display;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

use crate::identity::domain::users::models::username::Username;

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(FromRow)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
