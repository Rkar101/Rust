use sqlx::{Pool, Postgres};
pub async fn sales_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS sales (
            sale_id SERIAL PRIMARY KEY,
            shop_id INTEGER NOT NULL REFERENCES shops(shop_id),
            user_id INTEGER NOT NULL REFERENCES users(user_id),
            total_amount DECIMAL(12, 2) NOT NULL DEFAULT 0,
            payment_method TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
    "#,
    )
    .execute(&pool)
    .await?;
    Ok(())
}
