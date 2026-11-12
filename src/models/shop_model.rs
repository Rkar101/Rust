use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::Product;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ShopData {
    pub shop_name: String,
    pub contact: String,
    pub address: String,
    pub contact_link: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseShopListStatus {
    pub status: u16,
    pub state: bool,
    pub msg: String,
    pub shop_list: Vec<ResponseShopData>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ResponseShopData {
    pub shop_id: i32,
    pub shop_name: String,
    pub contact: String,
    pub address: String,
    pub contact_link: String,
    pub active: bool,
}

#[derive(Debug, Deserialize, Serialize , FromRow)]
pub struct ResponseShopProducts {
    pub status: u16,
    pub state: bool,
    pub msg: String,
    pub product_list: Vec<Product>,
}