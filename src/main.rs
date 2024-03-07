mod api;
mod config;
mod db;
mod utils;
use actix_web::{App, HttpServer};
use crate::api::routes::configure_routes;
use crate::db::migrations::run_migrations;
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Run migrations before starting the HTTP server
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url_clone = database_url.clone();
    let pool = PgPool::connect(&database_url_clone).await.unwrap();

    run_migrations(&pool).await.unwrap_or_else(|err| {
        panic!("Failed to run migrations: {}", err);
    });

    // Start the HTTP server
    HttpServer::new(|| {
        App::new().configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
