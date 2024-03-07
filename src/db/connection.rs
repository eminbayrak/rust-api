// src/db/connection.rs

use dotenv::dotenv;
use std::env;
use tokio_postgres::{NoTls, Error};

pub async fn establish_connection() -> Result<tokio_postgres::Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}
