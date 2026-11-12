use sqlx::{PgPool, query_as};

use crate::models::{AdminDashboardStats, Category};

pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_admin_dashboard_stats(&self) -> Result<AdminDashboardStats, sqlx::Error> {
        let stats = sqlx::query_as::<_, AdminDashboardStats>(
            r#"
        SELECT 
            (SELECT COUNT(*) FROM users) AS total_employee,
            (SELECT COUNT(*) FROM shops) AS total_shops,
            (SELECT COUNT(*) FROM products) AS total_products,
            (SELECT COALESCE(SUM(quantity), 0) FROM inventory) AS total_stock_quantity,
            (SELECT COUNT(*) FROM sales) AS total_sales_count,
            (SELECT COALESCE(SUM(total_amount), 0)::BIGINT FROM sales) AS total_revenue
        "#,
        )
        .fetch_one(&self.pool)
        .await?;

        // Since FromRow maps directly to your struct, if stats is already AdminDashboardStats,
        // you can just return it directly (or unwrap the options if you want guaranteed non-null numbers):
        Ok(AdminDashboardStats {
            total_shops: Some(stats.total_shops.unwrap_or(0)),
            total_products: Some(stats.total_products.unwrap_or(0)),
            total_stock_quantity: Some(stats.total_stock_quantity.unwrap_or(0)),
            total_sales_count: Some(stats.total_sales_count.unwrap_or(0)),
            total_revenue: Some(stats.total_revenue.unwrap_or(0)),
            total_employee: Some(stats.total_employee.unwrap_or(0)),
        })
    }

    pub async fn get_owner_dashboard_stats(
        &self,
        owner_id: i32,
    ) -> Result<AdminDashboardStats, sqlx::Error> {
        // 1. Fetch all shop IDs for this employee (handles 0, 1, or many shops safely)
        let shop_ids: Vec<i32> = sqlx::query_scalar(
            r#"
        SELECT shop_id FROM employee_shops WHERE employee_id = $1
        "#,
        )
        .bind(owner_id)
        .fetch_all(&self.pool)
        .await?;

        // 2. Aggregate stats across ALL of their assigned shops using ANY($1)
        let stats = sqlx::query_as::<_, AdminDashboardStats>(
        r#"
        SELECT 
            (SELECT COUNT(*) FROM users) AS total_employee,
            (SELECT COUNT(*) FROM shops WHERE shop_id = ANY($1)) AS total_shops,
            (SELECT COUNT(*) FROM inventory WHERE shop_id = ANY($1)) AS total_products,
            (SELECT COALESCE(SUM(quantity), 0) FROM inventory WHERE shop_id = ANY($1)) AS total_stock_quantity,
            (SELECT COUNT(*) FROM sales WHERE shop_id = ANY($1)) AS total_sales_count,
            (SELECT COALESCE(SUM(total_amount), 0)::BIGINT FROM sales WHERE shop_id = ANY($1)) AS total_revenue
        "#,
    )
    .bind(&shop_ids) // Pass the vector of shop IDs
    .fetch_one(&self.pool)
    .await?;

        // 3. Return the mapped struct safely
        Ok(AdminDashboardStats {
            total_shops: Some(stats.total_shops.unwrap_or(0)),
            total_products: Some(stats.total_products.unwrap_or(0)),
            total_stock_quantity: Some(stats.total_stock_quantity.unwrap_or(0)),
            total_sales_count: Some(stats.total_sales_count.unwrap_or(0)),
            total_revenue: Some(stats.total_revenue.unwrap_or(0)),
            total_employee: Some(stats.total_employee.unwrap_or(0)),
        })
    }
    pub async fn get_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
        let category_list = query_as::<_, Category>(
            r#"
            SELECT * FROM categories
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(category_list)
    }
}
