use sqlx::{PgPool, Postgres, query, query_as, query_scalar};

use crate::{
    auth::extractor::AuthRole,
    errors::shop_error::ShopDataError,
    models::{Product, ResponseShopData, ShopData},
};
pub struct ShopRepository {
    pool: PgPool,
}

impl ShopRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_shop_count(&self) -> Result<i64, sqlx::Error> {
        let shop_count = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM shops
        "#,
        )
        .fetch_one(&self.pool) // You likely need to pass your database pool here!
        .await?;

        Ok(shop_count)
    }

    pub async fn create_shop(&self, shop_data: ShopData) -> Result<(), ShopDataError> {
        query(
            r#"
            INSERT INTO shops (shop_name, contact, address, contact_link)
            Values ($1,$2,$3,$4)  
        "#,
        )
        .bind(shop_data.shop_name)
        .bind(shop_data.contact)
        .bind(shop_data.address)
        .bind(shop_data.contact_link)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_shop_by_owner(
        &self,
        shop_data: ShopData,
        owner_id: i32,
    ) -> Result<(), ShopDataError> {
        let mut tx: sqlx::Transaction<'_, Postgres> = self.pool.begin().await?;

        let shop_id: i32 = query_scalar(
            r#"
        INSERT INTO shops (shop_name, contact, address, contact_link)
        VALUES ($1, $2, $3, $4)
        RETURNING shop_id
        "#,
        )
        .bind(shop_data.shop_name)
        .bind(shop_data.contact)
        .bind(shop_data.address)
        .bind(shop_data.contact_link)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            r#"
        INSERT INTO employee_shops (employee_id, shop_id)
        VALUES ($1, $2)
        "#,
        )
        .bind(owner_id)
        .bind(shop_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn edit_shop_by_owner(
        &self,
        shop_data: ResponseShopData,
    ) -> Result<(), ShopDataError> {
        query(
            r#"

             UPDATE shops
                SET
                    shop_name = COALESCE($2, shop_name),
                    contact = COALESCE($3, contact),
                    address = COALESCE($4, address),
                    contact_link = COALESCE($5, contact_link)
                WHERE shop_id = $1
        "#,
        )
        .bind(shop_data.shop_id)
        .bind(shop_data.shop_name)
        .bind(shop_data.contact)
        .bind(shop_data.address)
        .bind(shop_data.contact_link)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_shop_byowner(
        &self,
        emp_id: i32,
        emp_role: AuthRole,
    ) -> Result<Vec<ResponseShopData>, ShopDataError> {
        match emp_role {
            AuthRole::Admin => {
                let shop_list = query_as::<_, ResponseShopData>(
                    r#"
                    SELECT * FROM shops
                "#,
                )
                .bind(emp_id)
                .fetch_all(&self.pool)
                .await?;

                println!("Result: {:?}", &shop_list);

                Ok(shop_list)
            }
            AuthRole::Owner => {
                let shop_id: i32 = query_scalar(
                    r#"
                            SELECT shop_id FROM employee_shops WHERE employee_id = $1
                        "#,
                )
                .bind(emp_id)
                .fetch_one(&self.pool)
                .await?;
                let shop_list = query_as::<_, ResponseShopData>(
                    r#"
                    SELECT * FROM shops WHERE shop_id = $1
                "#,
                )
                .bind(shop_id)
                .fetch_all(&self.pool)
                .await?;

                println!("Result: {:?}", &shop_list);

                Ok(shop_list)
            }
            AuthRole::Employee => {
                let shop_list = query_as::<_, ResponseShopData>(
                    r#"
                    SELECT * FROM shops
                "#,
                )
                .bind(emp_id)
                .fetch_all(&self.pool)
                .await?;

                println!("Result: {:?}", &shop_list);

                Ok(shop_list)
            },
        }
    }

    pub async fn get_shop_products(&self, shop_id: i32) -> Result<Vec<Product>, ShopDataError> {
        let product_list = query_as::<_, Product>(
            r#"
                    SELECT * FROM inventory WHERE shop_id = $1
                "#,
        )
        .bind(shop_id)
        .fetch_all(&self.pool)
        .await?;

        println!("Result: {:?}", &product_list);

        Ok(product_list)
    }
}
