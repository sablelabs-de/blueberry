use std::error::Error;

use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers_modules::{
  postgres::Postgres,
  redis::{REDIS_PORT, Redis},
  testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

pub type TestResult<T> = Result<T, Box<dyn Error + Send + Sync + 'static>>;

// Match these tags to the versions used in production. The module defaults
// are intentionally overridden because they currently point to old images.
const POSTGRES_TAG: &str = "17-alpine";
const REDIS_TAG: &str = "7.4-alpine";
const POSTGRES_PORT: u16 = 5432;

pub struct TestContext {
  pub pg_pool: PgPool,
  pub postgres_url: String,
  pub redis_url: Option<String>,

  _postgres: ContainerAsync<Postgres>,
  _redis: Option<ContainerAsync<Redis>>,
}

impl TestContext {
  pub async fn postgres() -> TestResult<Self> {
    Self::start(false).await
  }

  pub async fn postgres_and_redis() -> TestResult<Self> {
    Self::start(true).await
  }

  async fn start(start_redis: bool) -> TestResult<Self> {
    let postgres = Postgres::default().with_tag(POSTGRES_TAG).start().await?;

    let postgres_host = postgres.get_host().await?;
    let postgres_port = postgres.get_host_port_ipv4(POSTGRES_PORT).await?;
    let postgres_url = format!(
      "postgres://postgres:postgres@{postgres_host}:{postgres_port}/postgres"
    );

    let pg_pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(&postgres_url)
      .await?;

    sqlx::migrate!("../../../migrations").run(&pg_pool).await?;

    let (redis_url, redis) = if start_redis {
      let redis = Redis::default().with_tag(REDIS_TAG).start().await?;
      let redis_host = redis.get_host().await?;
      let redis_port = redis.get_host_port_ipv4(REDIS_PORT).await?;

      (
        Some(format!("redis://{redis_host}:{redis_port}/")),
        Some(redis),
      )
    } else {
      (None, None)
    };

    Ok(Self {
      pg_pool,
      postgres_url,
      redis_url,
      _postgres: postgres,
      _redis: redis,
    })
  }
}
