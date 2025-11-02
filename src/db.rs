use anyhow::Result;
use sqlx::{migrate, MySqlPool};
use std::env::var;

pub async fn pool() -> Result<MySqlPool> {
    let database_url = var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await?;

    migrate!("src/migrations").run(&pool).await?;

    Ok(pool)
}
