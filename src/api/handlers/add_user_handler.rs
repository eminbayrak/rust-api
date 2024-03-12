use actix_web::{web, HttpResponse, Responder};
use crate::db::connection;
use crate::api::models::User;
use crate::api::models::UserInfo;
use log::error;
use argon2::{Argon2};

pub async fn add_user(user_info: web::Json<UserInfo>) -> impl Responder {
    // Establish a connection to the database
    let pool = match connection::establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish database connection: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Hash the password
    let argon2 = Argon2::default();
    let salt = [0u8; 32];
    let mut hashed_password = [0u8; 64];
    argon2
        .hash_password_into(user_info.password.as_bytes(), &salt, &mut hashed_password)
        .unwrap();

    // Insert a new user into the database
    let row = match sqlx::query_as::<_, User>("INSERT INTO users (first_name, last_name, username, email, password) VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, first_name, last_name, username, email")
        .bind(&user_info.first_name)
        .bind(&user_info.last_name)
        .bind(&user_info.username)
        .bind(&user_info.email)
        .bind(&hashed_password[..] as &[u8])
        .fetch_one(&pool).await {
            Ok(row) => row,
            Err(e) => {
                error!("Failed to execute SQL query: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

    HttpResponse::Ok().json(row)
}
