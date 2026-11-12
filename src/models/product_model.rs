use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::pagination_model::*;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProduct {
  
    pub name: String,
    pub category_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkProductRequest {
    pub products: Vec<CreateProduct>,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct ResponseStatus {
    pub status: u16,
    pub state: bool,
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseProductListStatus {
    pub status: u16,
    pub state: bool,
    pub msg: String,
    pub product_list: ProductListResponse,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductListResponse {
    pub data: Vec<Product>,
    pub pagination: Pagination,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchProduct {
    pub name: Option<String>,
    pub price: Option<i64>,
    pub stock: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub name: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub category_id: i32,
    pub name: String,
}

#[derive(Serialize)]
pub struct ResponseCategory {
    pub status : u16,
    pub state : bool,
    pub msg : String,
    pub data : Option<Vec<Category>>
}


