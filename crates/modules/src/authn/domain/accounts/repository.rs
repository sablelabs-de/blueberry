use async_trait::async_trait;

use crate::{
    authn::domain::accounts::models::{
        account::{Account, NewAccount},
        email::Email,
    },
    shared::errors::UnexpectedError,
};

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum CreateAccountError {
    EmailAlreadyExists,
    UsernameAlreadyExists,
    Unexpected(#[from] UnexpectedError),
}

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum FindAccountError {
    NotFound,
    Unexpected(#[from] UnexpectedError),
}

#[async_trait]
pub trait AbstractAccountRepository {
    async fn create(
        &self,
        new_user: NewAccount,
    ) -> Result<(), CreateAccountError>;

    async fn find_account_by_email(
        &self,
        email: Email,
    ) -> Result<Account, FindAccountError>;
}
