use sqlx::{Pool, Postgres};


pub async fn user_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error>{
        sqlx::raw_sql(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                user_id SERIAL PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
    "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
                ALTER TABLE users
                ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'employee'
    "#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}