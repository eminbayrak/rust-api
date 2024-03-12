use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Row}; // Import FromRow and Row traits from sqlx

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

// Implement FromRow trait for User struct
impl<'r> FromRow<'r, sqlx::postgres::PgRow> for User {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            username: row.get("username"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    pub data: Vec<User>,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
