use authn::{
  AccountRepository, LogInCommand, SessionRepository, SignUpCommand, log_in,
  sign_up,
};

use crate::common::{TestContext, TestResult};

mod common;

#[tokio::test]
async fn creates_account() -> TestResult<()> {
  let ctx = TestContext::postgres().await?;

  let account_repository = AccountRepository::new(ctx.pg_pool.clone());

  let session_repository = SessionRepository::new(ctx.pg_pool);

  // let service = AuthNService::new(account_repository, session_repository);

  sign_up(
    &account_repository,
    SignUpCommand {
      username: "sd".to_string(),
      email: "asd@test.pl".to_string(),
      password: "asddfg953k!9Af".to_string(),
    },
  )
  .await?;

  log_in(
    &account_repository,
    &session_repository,
    LogInCommand {
      email: "asd@test.pl".to_string(),
      password: "asddfg953k!9Af".to_string(),
    },
  )
  .await?;

  Ok(())
}
