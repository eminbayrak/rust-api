use sqlx::{Error, PgPool, Row};

pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    // Create users table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            first_name VARCHAR(50) NOT NULL,
            last_name VARCHAR(50) NOT NULL,
            username VARCHAR(50) NOT NULL,
            email VARCHAR(100) NOT NULL,
            password BYTEA NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Check if the users table is empty
    let count = sqlx::query("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?
        .get::<i64, _>(0);

    // If the table is empty, seed dummy data
    if count == 0 {
        for i in 1..=100 {
            sqlx::query(
                r#"
                INSERT INTO users (first_name, last_name, username, email)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(format!("John{}", i))
            .bind(format!("Doe{}", i))
            .bind(format!("user{}", i))
            .bind(format!("user{}@example.com", i))
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
