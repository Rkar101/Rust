use sqlx::{Pool, Postgres};

pub async fn emp_shop_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS employee_shops (
        employee_id INTEGER NOT NULL REFERENCES users(user_id),
        shop_id INTEGER NOT NULL REFERENCES shops(shop_id),
        PRIMARY KEY (employee_id, shop_id));
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(())

}
