use actix_web::HttpResponse;
use crate::db::connection;
use crate::api::models::User;

pub async fn users() -> HttpResponse {
    // Establish a connection to the database
    let client = match connection::establish_connection().await {
        Ok(client) => client,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Execute a SQL query to fetch user data
    let rows = match client.query("SELECT id, first_name, last_name, username, email, created_at FROM users", &[]).await {
        Ok(rows) => rows,
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
        let created_at: String = row.get(5);
        users.push(User { id, first_name, last_name, username, email, created_at });
    }

    HttpResponse::Ok().json(users)
}