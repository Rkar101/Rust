use sqlx::{Pool, Postgres};
pub async fn sale_item_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS sale_items (
            id SERIAL PRIMARY KEY,
            sale_id INTEGER NOT NULL REFERENCES sales(sale_id),
            product_id INTEGER NOT NULL REFERENCES products(product_id),
            quantity INTEGER NOT NULL,
            unit_price DECIMAL NOT NULL,
            subtotal DECIMAL NOT NULL
        )
    "#,
    )
    .execute(&pool)
    .await?;


    sqlx::query(
        r#"
                ALTER TABLE sale_items
                ADD COLUMN IF NOT EXISTS inventory_id INTEGER NOT NULL REFERENCES inventory(id)
    "#,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
