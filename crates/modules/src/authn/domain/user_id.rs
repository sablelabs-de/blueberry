// TODO: this should be in another file (module)

use derive_more::Display;
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, Hash, Type)]
#[sqlx(transparent)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}
