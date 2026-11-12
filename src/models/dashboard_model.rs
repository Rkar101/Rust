#[derive(serde::Serialize,sqlx::FromRow)]
pub struct AdminDashboardStats {
    pub total_employee: Option<i64>,
    pub total_shops: Option<i64>,
    pub total_products: Option<i64>,
    pub total_stock_quantity: Option<i64>,
    pub total_sales_count: Option<i64>,
    pub total_revenue: Option<i64>, // Or rust_decimal::Decimal if you use decimal types
}

#[derive(serde::Serialize)]
pub struct ResponseAdminStatus {
    pub status: u16,
    pub state: bool,
    pub msg: String,
    pub data : Option<AdminDashboardStats>
}