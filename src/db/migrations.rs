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
            password TEXT NULL,
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
        sqlx::query(
            r#"
            DO $$
            DECLARE
                first_names VARCHAR[] := ARRAY['John', 'Emma', 'Michael', 'Sophia', 'William', 'Olivia', 'James', 'Ava', 'Alexander', 'Isabella', 'Daniel', 'Mia', 'Matthew', 'Emily', 'Joseph', 'Charlotte', 'David', 'Abigail', 'Benjamin', 'Amelia'];
                last_names VARCHAR[] := ARRAY['Smith', 'Johnson', 'Williams', 'Jones', 'Brown', 'Davis', 'Miller', 'Wilson', 'Moore', 'Taylor', 'Anderson', 'Thomas', 'Jackson', 'White', 'Harris', 'Martin', 'Thompson', 'Garcia', 'Martinez', 'Robinson'];
            BEGIN
                FOR i IN 1..100 LOOP
                    INSERT INTO users (first_name, last_name, username, email)
                    VALUES (
                        first_names[FLOOR(RANDOM() * array_length(first_names, 1)) + 1],
                        last_names[FLOOR(RANDOM() * array_length(last_names, 1)) + 1],
                        'user' || i,
                        'user' || i || '@example.com'
                    );
                END LOOP;
            END $$;
            "#,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
