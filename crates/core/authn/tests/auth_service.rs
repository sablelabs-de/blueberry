use authn::{AccountRepository, AuthNService, SignUpCommand};

use crate::common::{TestContext, TestResult};

mod common;

#[tokio::test]
async fn creates_account() -> TestResult<()> {
    let ctx = TestContext::postgres().await?;

    let repository = AccountRepository::new(ctx.pg_pool);

    let service = AuthNService::new(repository);

    service
        .sign_up(SignUpCommand {
            username: "sd".to_string(),
            email: "asd@test.pl".to_string(),
            password: "asddfg953k!9Af".to_string(),
        })
        .await?;

    service
        .sign_up(SignUpCommand {
            username: "aasd".to_string(),
            email: "asd@tst.pl".to_string(),
            password: "asddfg953k!9Af".to_string(),
        })
        .await?;

    Ok(())
}
