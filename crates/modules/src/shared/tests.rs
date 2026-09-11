use std::error::Error;

use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

const POSTGRES_TAG: &str = "18-alpine";
const POSTGRES_PORT: u16 = 5432;

pub type TestResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

pub struct TestContext<T> {
    context: T,
}

pub struct OnlyPostgres {
    pool: PgPool,
    _container: ContainerAsync<Postgres>,
}
struct OnlyRedis;
struct All;

impl TestContext<OnlyPostgres> {
    pub async fn new() -> TestResult<Self> {
        let container =
            Postgres::default().with_tag(POSTGRES_TAG).start().await?;
        let connection_string = format!(
            "postgres://postgres:postgres@{}:{}/postgres",
            container.get_host().await?,
            container.get_host_port_ipv4(POSTGRES_PORT).await?
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await?;

        sqlx::migrate!("../../migrations").run(&pool).await?;

        Ok(Self {
            context: OnlyPostgres {
                pool,
                _container: container,
            },
        })
    }

    pub fn get_pool(&self) -> PgPool {
        self.context.pool.clone()
    }
}
