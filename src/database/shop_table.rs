use sqlx::{Pool, Postgres};

pub async fn shop_table_create(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(
        r#"
            CREATE TABLE IF NOT EXISTS shops (
                shop_id SERIAL PRIMARY KEY,
                shop_name TEXT UNIQUE NOT NULL,
                contact TEXT UNIQUE NOT NULL,
                address TEXT NOT NULL,
                contact_link TEXT UNIQUE NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                active BOOL NOT NULL DEFAULT TRUE
            )
    "#,
    )
    .execute(&pool)
    .await?;

    // sqlx::query(
    //     r#"
    //             ALTER TABLE shops
    //             ADD COLUMN IF NOT EXISTS  owner_id INTEGER NOT NULL DEFAULT 1 REFERENCES users(id) 
    
    // "#,
    // )
    // .execute(&pool)
    // .await?;

    Ok(())
}
