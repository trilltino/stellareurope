use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub type DbPool = PgPool;

pub async fn create_pool() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    Ok(pool)
}