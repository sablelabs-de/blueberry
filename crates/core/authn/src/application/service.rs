use crate::{
    application::service::{
        log_in::{LogInCommand, LogInError},
        sign_up::{SignUpCommand, SignUpError},
    },
    domain::accounts::repository::AbstractAccountRepository,
};

pub mod log_in;
pub mod sign_up;

pub struct AuthNService<A: AbstractAccountRepository> {
    account_repository: A,
}

impl<A: AbstractAccountRepository> AuthNService<A> {
    pub fn new(account_repository: A) -> Self {
        Self { account_repository }
    }

    pub async fn sign_up(&self, cmd: SignUpCommand) -> Result<(), SignUpError> {
        sign_up::sign_up(&self.account_repository, cmd).await
    }

    async fn log_in(&self, cmd: LogInCommand) -> Result<(), LogInError> {
        log_in::log_in(&self.account_repository, cmd).await
    }
}
