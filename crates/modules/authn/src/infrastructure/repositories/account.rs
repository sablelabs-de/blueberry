use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{
  accounts::{
    models::{
      account::{Account, NewAccount},
      email::Email,
      username::Username,
    },
    repository::{
      AbstractAccountRepository, CreateAccountError, FindAccountError,
    },
  },
  shared::errors::UnexpectedError,
  user_id::UserId,
};

impl From<sqlx::Error> for CreateAccountError {
  fn from(error: sqlx::Error) -> Self {
    match &error {
      sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
        match db_err.constraint() {
          Some("accounts_email_key") => Self::EmailAlreadyExists,
          Some("users_username_key") => Self::UsernameAlreadyExists,
          _ => UnexpectedError::new(error).into(),
        }
      }
      _ => UnexpectedError::new(error).into(),
    }
  }
}

pub struct AccountRepository {
  pool: PgPool,
}

impl AccountRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl AbstractAccountRepository for AccountRepository {
  async fn create(
    &self,
    new_account: NewAccount,
  ) -> Result<(), CreateAccountError> {
    let id = UserId::new();

    let mut tx = self.pool.begin().await.map_err(UnexpectedError::new)?;

    insert_user_tx(&mut tx, id, new_account.username).await?;

    insert_account_tx(
      &mut tx,
      id,
      new_account.email,
      new_account.password_hash,
    )
    .await?;

    tx.commit().await.map_err(UnexpectedError::new)?;

    Ok(())
  }

  async fn find_account_by_email(
    &self,
    email: Email,
  ) -> Result<Account, FindAccountError> {
    let account = sqlx::query_as!(
      Account,
      r#"
                SELECT
                    user_id AS "user_id: UserId",
                    email AS "email: Email",
                    email_verified,
                    password_hash,
                    created_at,
                    updated_at
                FROM "accounts"
                WHERE email = $1
                LIMIT 1
            "#,
      email as Email
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(UnexpectedError::new)?;

    account.ok_or(FindAccountError::NotFound)
  }
}

async fn insert_account_tx(
  tx: &mut Transaction<'_, Postgres>,
  user_id: UserId,
  email: Email,
  password_hash: String,
) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
            INSERT INTO "accounts"
            (user_id, email, password_hash)
            VALUES
            ($1, $2, $3)
        "#,
    user_id as UserId,
    email as Email,
    password_hash
  )
  .execute(&mut **tx)
  .await?;

  Ok(())
}

async fn insert_user_tx(
  tx: &mut Transaction<'_, Postgres>,
  user_id: UserId,
  username: Username,
) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
            INSERT INTO "users"
            (id, username)
            VALUES
            ($1, $2)
        "#,
    user_id as UserId,
    username as Username,
  )
  .execute(&mut **tx)
  .await?;

  Ok(())
}
