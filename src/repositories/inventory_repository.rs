use std::env;

use sqlx::{PgPool, query};

use crate::models::{InventoryData, InventoryItemView};

pub struct InventoryRepo {
    pool: PgPool,
}

impl InventoryRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_inventory_data(
        &self,
        shop_id: Option<i32>,
    ) -> Result<Vec<InventoryItemView>, sqlx::Error> {
        let items = sqlx::query_as::<_, InventoryItemView>(
            r#"
            SELECT 
                i.id,
                i.shop_id,
                i.product_id,
                p.name AS product_name,
                i.price,
                i.quantity,
                i.image_path,
                i.image_url,
                i.updated_at
            FROM inventory i
            JOIN products p ON i.product_id = p.product_id
            WHERE i.shop_id = $1
            ORDER BY p.name ASC
        "#,
        )
        .bind(shop_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    pub async fn add_inventory_product(&self, inv_pdata: InventoryData) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
        INSERT INTO inventory (
            shop_id,
            product_id,
            price,
            quantity
        )
        VALUES ($1, $2, $3, $4)
        "#,
        )
        .bind(inv_pdata.shop_id)
        .bind(inv_pdata.product_id)
        .bind(inv_pdata.price)
        .bind(inv_pdata.quantity)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn edit_inventory_product(
        &self,
        inv_pdata: InventoryData,
    ) -> Result<(), sqlx::Error> {
        let patch_result = query(
            r#"
                UPDATE inventory
                SET
                    price = COALESCE($2, price),
                    quantity = COALESCE($3, quantity),
                    updated_at = COALESCE($4, updated_at)
                WHERE id = $1
            "#,
        )
        .bind(inv_pdata.id)
        .bind(inv_pdata.price)
        .bind(inv_pdata.quantity)
        .bind(inv_pdata.updated_at)
        .execute(&self.pool)
        .await?;

        if patch_result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn update_image_path(
        &self,
        inventory_id: i32,
        image_path: &str,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE inventory
            SET
                image_path = $2,
                updated_at = NOW()
            WHERE id = $1
        "#,
        )
        .bind(inventory_id)
        .bind(image_path)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}
