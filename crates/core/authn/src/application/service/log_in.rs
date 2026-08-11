use crate::{
    application::crypto::{self, hasher::HasherError},
    domain::accounts::{
        models::{
            email::{self, Email},
            password::{self, Password},
        },
        repository::{AbstractAccountRepository, FindAccountError},
    },
};

pub struct LogInCommand {
    pub email: String,
    pub password: String,
}

#[derive(thiserror::Error, strum::Display, Debug)]
pub enum LogInError {
    Email(#[from] email::ValidationError),
    Password(#[from] password::ValidationError),
    Hasher(#[from] HasherError),
    FindAccount(#[from] FindAccountError),
}

pub async fn log_in(
    account_repository: &impl AbstractAccountRepository,
    cmd: LogInCommand,
) -> Result<(), LogInError> {
    let email = Email::new(&cmd.email)?;
    let password = Password::new(&cmd.password)?;

    let account = account_repository.find_account_by_email(email).await;

    let password_hash = match account {
        Ok(ref account) => &account.password_hash,
        Err(FindAccountError::NotFound) => crypto::hasher::DUMMY_HASH,
        Err(error) => return Err(error.into()),
    }
    .to_owned();

    let verification = crypto::hasher::verify_password(password, password_hash).await;

    let account = account?;
    verification?;

    // todo!

    Ok(())
}
