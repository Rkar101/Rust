use sqlx::{PgPool, Postgres, QueryBuilder, Transaction, query, query_as};

use crate::models::{CreateProduct, PatchProduct, Product};

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_products(
        &self,
        page: i64,
        limit: i64,
        name: Option<String>,
        sort: Option<String>,
        order: Option<String>,
        min_price: Option<i64>,
        max_price: Option<i64>,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let offset = (page - 1) * limit;

        let mut builder = QueryBuilder::<Postgres>::new("SELECT * FROM products");
        let mut has_where = false;
        if let Some(name) = name {
            let search = format!("%{}%", name);
            builder.push(" WHERE name ILIKE ");
            builder.push_bind(search);
            has_where = true;
        }

        // if let Some(min_price) = min_price {
        //     if has_where {
        //         builder.push(" AND price >= ");
        //         builder.push_bind(min_price);
        //     } else {
        //         builder.push(" WHERE price >= ");
        //         builder.push_bind(min_price);
        //         has_where = true;
        //     }
        // }

        // if let Some(max_price) = max_price {
        //     if has_where {
        //         builder.push(" AND price <= ");
        //         builder.push_bind(max_price);
        //     } else {
        //         builder.push(" WHERE price <= ");
        //         builder.push_bind(max_price);
        //         //    has_where = true;
        //     }
        // }

        // builder.push(" ORDER BY ");

        // match sort.as_deref() {
        //     Some("price") => builder.push("price"),
        //     Some("stock") => builder.push("stock"),
        //     Some("name") => builder.push("name"),
        //     _ => builder.push("id"),
        // };

        // match order.as_deref() {
        //     Some("desc") => builder.push(" DESC"),
        //     _ => builder.push(" ASC"),
        // };

        // builder.push(" LIMIT ");
        // builder.push_bind(limit);

        // builder.push(" OFFSET ");
        // builder.push_bind(offset);

        let products = builder
            .build_query_as::<Product>()
            .fetch_all(&mut **tx)
            .await?;

        Ok(products)
    }

    pub async fn get_product_by_id(&self, id: i32) -> Result<Option<Product>, sqlx::Error> {
        let product = query_as::<_, Product>(
            r#"
                    SELECT * FROM products WHERE product_id = $1
                "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        println!("Result: {:?}", &product);

        Ok(product)
    }

    pub async fn patch_product_by_id(
        &self,
        id: i32,
        product: &PatchProduct,
    ) -> Result<(), sqlx::Error> {
        let patch_result = query(
            r#"
                UPDATE products
                SET
                    name = COALESCE($1, name),
                    price = COALESCE($2, price),
                    stock = COALESCE($3, stock)
                WHERE id = $4
            "#,
        )
        .bind(product.name.as_deref())
        .bind(product.price)
        .bind(product.stock)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if patch_result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn delete_product_by_id(&self, id: i32) -> Result<(), sqlx::Error> {
        query(
            r#"
                DELETE FROM products
                WHERE id = $1;
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_product(&self, product: &CreateProduct) -> Result<(), sqlx::Error> {
        query(
            r#"
            INSERT INTO products (name, category_id)
            VALUES ($1,$2)
        "#,
        )
        .bind(&product.name)
        .bind(&product.category_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_bulk_product(
        &self,
        bulk_products: Vec<CreateProduct>,
    ) -> Result<(), sqlx::Error> {
        let mut builder =
            QueryBuilder::<Postgres>::new("INSERT INTO products (name, category_id, created_at)");

        builder.push_values(bulk_products, |mut row, product| {
            row.push_bind(product.name).push_bind(product.category_id);
        });

        builder.build().execute(&self.pool).await?;

        Ok(())
    }

    /// Owner Data Fetching
    pub async fn count_products_by_owner(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        owner_id: i32,
    ) -> Result<i64, sqlx::Error> {
        let shop_ids: Vec<i32> = sqlx::query_scalar(
            r#"
           SELECT shop_id FROM employee_shops WHERE employee_id = $1
        "#,
        )
        .bind(owner_id)
        .fetch_all(&mut **tx)
        .await?;

        let stats = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM inventory WHERE shop_id = ANY($1)           
        "#,
        )
        .bind(&shop_ids)
        .fetch_one(&mut **tx)
        .await?;

        Ok(stats)
    }

    pub async fn get_products_by_owner(
        &self,
        page: i64,
        limit: i64,
        tx: &mut Transaction<'_, Postgres>,
        owner_id: i32,
    ) -> Result<Vec<Product>, sqlx::Error> {
        let shop_ids: Vec<i32> = sqlx::query_scalar(
            r#"
            SELECT shop_id FROM employee_shops WHERE employee_id = $1
        "#,
        )
        .bind(owner_id)
        .fetch_all(&mut **tx)
        .await?;

        let stats: Vec<i32> = sqlx::query_scalar(
            r#"         
            SELECT product_id FROM inventory WHERE shop_id = ANY($1)           
        "#,
        )
        .bind(&shop_ids)
        .fetch_all(&mut **tx)
        .await?;

        let products = query_as::<_, Product>(
            r#"
                    SELECT * FROM products WHERE product_id = ANY($1)
                "#,
        )
        .bind(stats)
        .fetch_all(&self.pool)
        .await?;

        Ok(products)
    }

    pub async fn count_products(
        &self,
        name: Option<String>,
        min_price: Option<i64>,
        max_price: Option<i64>,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<i64, sqlx::Error> {
        let mut builder = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM products");
        let mut has_where = false;
        if let Some(name) = name {
            let search = format!("%{}%", name);
            builder.push(" WHERE name ILIKE ");
            builder.push_bind(search);
            has_where = true;
        };

        if let Some(min_price) = min_price {
            if has_where {
                builder.push(" AND price >= ");
                builder.push_bind(min_price);
            } else {
                builder.push(" WHERE price >= ");
                builder.push_bind(min_price);
                has_where = true;
            }
        }

        if let Some(max_price) = max_price {
            if has_where {
                builder.push(" AND price <= ");
                builder.push_bind(max_price);
            } else {
                builder.push(" WHERE price <= ");
                builder.push_bind(max_price);
                //    has_where = true;
            }
        }

        // builder.push(" ORDER BY ");

        // match sort.as_deref() {
        //     Some("price") => builder.push("price"),
        //     Some("stock") => builder.push("stock"),
        //     Some("name") => builder.push("name"),
        //     _ => builder.push("id"),
        // };

        // match order.as_deref() {
        //     Some("desc") => builder.push("DESC"),
        //     _ => builder.push("ASC"),
        // };

        let result = builder
            .build_query_scalar::<i64>()
            .fetch_one(&mut **tx)
            .await?;
        Ok(result)
        // builder.push(" LIMIT ");
        // builder.push_bind(limit);

        // builder.push(" OFFSET ");
        // builder.push_bind(offset);
    }
}
