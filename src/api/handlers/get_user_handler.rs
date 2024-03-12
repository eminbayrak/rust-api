use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use crate::db::connection;
use crate::api::models::User;
use sqlx::Row;
use serde_json::json;

pub async fn users() -> HttpResponse {
    // Establish a connection to the database
    let pool = match connection::establish_connection().await {
        Ok(pool) => pool,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Execute a SQL query to fetch user data
    let rows = match sqlx::query("SELECT id, first_name, last_name, username, email, created_at FROM users LIMIT 100")
        .fetch_all(&pool).await {
        Ok(rows) => rows,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Execute a SQL query to fetch total count of users
    let total_count = match sqlx::query("SELECT count(*) FROM users")
        .fetch_one(&pool).await {
        Ok(row) => row.get::<i64, _>(0),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Process the rows and construct a response
    let mut users = Vec::new();
    for row in rows {
        let id: i32 = row.get(0);
        let first_name: String = row.get(1);
        let last_name: String = row.get(2);
        let username: String = row.get(3);
        let email: String = row.get(4);
        let created_at: NaiveDateTime = row.get(5);
        users.push(User { id, first_name, last_name, username, email, created_at });
    }

    // Return total users count and users
    HttpResponse::Ok().json(json!({ "total_count": total_count, "users": users }))
}
