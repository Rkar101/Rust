use sqlx::{Pool, Postgres};

pub async fn product_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            product_id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            category_id INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(&pool)
    .await?;
    Ok(())
}
