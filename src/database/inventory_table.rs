use sqlx::{Pool, Postgres};

pub async fn inventory_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
            CREATE TABLE IF NOT EXISTS inventory (
                id SERIAL PRIMARY KEY,
                shop_id INTEGER NOT NULL REFERENCES shops(shop_id),
                product_id INTEGER NOT NULL REFERENCES products(product_id),
                price INTEGER NOT NULL,
                quantity INTEGER NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE (shop_id, product_id)
            )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
                ALTER TABLE inventory
                ADD COLUMN IF NOT EXISTS image_path TEXT
    "#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
