use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::domain::{
    accounts::{
        models::{
            account::{Account, AccountId, NewAccount},
            email::Email,
            username::Username,
        },
        repository::{AbstractAccountRepository, CreateAccountError, FindAccountError},
    },
    shared::errors::UnexpectedError,
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
    async fn create(&self, new_account: NewAccount) -> Result<(), CreateAccountError> {
        let id = AccountId::new();

        let mut tx = self.pool.begin().await.map_err(UnexpectedError::new)?;

        insert_account_tx(&mut tx, id, new_account.email, new_account.password_hash).await?;

        insert_user_tx(&mut tx, id, new_account.username).await?;

        tx.commit().await.map_err(UnexpectedError::new)?;

        Ok(())
    }

    async fn find_account_by_email(&self, email: Email) -> Result<Account, FindAccountError> {
        let account = sqlx::query_as!(
            Account,
            r#"
                SELECT
                    id AS "id: AccountId",
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
    id: AccountId,
    email: Email,
    password_hash: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO "accounts"
            (id, email, password_hash)
            VALUES
            ($1, $2, $3)
        "#,
        id as AccountId,
        email as Email,
        password_hash
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_user_tx(
    tx: &mut Transaction<'_, Postgres>,
    id: AccountId,
    username: Username,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
            INSERT INTO "users"
            (id, username)
            VALUES
            ($1, $2)
        "#,
        id as AccountId,
        username as Username,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
