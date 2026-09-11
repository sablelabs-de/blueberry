use crate::{
    authn::{
        application::service::sign_up::{SignUpCommand, sign_up},
        domain::accounts::models::{account::Account, email::Email},
        infrastructure::repositories::account::AccountRepository,
    },
    identity::domain::users::models::{user::User, username::Username},
    shared::tests::{OnlyPostgres, TestContext, TestResult},
};

#[tokio::test]
async fn sign_up_success() -> TestResult<()> {
    let ctx = TestContext::<OnlyPostgres>::new().await?;

    let account_repository = AccountRepository::new(ctx.get_pool());

    let username = "test".to_owned();
    let email = "test_email@test.com".to_owned();
    let password = "password!AB1".to_owned();

    sign_up(
        &account_repository,
        SignUpCommand {
            username: username.clone(),
            email: email.clone(),
            password: password.clone(),
        },
    )
    .await?;

    let test_pool = ctx.get_pool();
    let account: Account = sqlx::query_as(
        "
        SELECT * FROM accounts WHERE email = $1
        ",
    )
    .bind(&email)
    .fetch_one(&test_pool)
    .await?;

    assert_eq!(account.email, Email::new(&email)?);
    assert_eq!(account.email_verified, false);

    let user: User = sqlx::query_as(
        "
        SELECT * FROM users WHERE username = $1
        ",
    )
    .bind(&username)
    .fetch_one(&test_pool)
    .await?;

    assert_eq!(user.username, Username::new(&username)?);

    Ok(())
}
