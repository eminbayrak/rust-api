use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPool;
use sqlx::Error;

pub async fn establish_connection() -> Result<PgPool, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}