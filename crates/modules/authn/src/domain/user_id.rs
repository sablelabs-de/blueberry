// todo! this should be in another file

use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
}
