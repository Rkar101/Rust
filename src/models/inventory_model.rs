use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Debug, Deserialize, FromRow)]
pub struct InventoryData {
    pub id: i32,
    pub shop_id: i32,
    pub product_id: i32,
    pub price: i32,
    pub quantity: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Deserialize, FromRow)]
pub struct ResponseInventoryData {
    pub status: i32,
    pub state: bool,
    pub msg: String,
    pub data_list: Vec<InventoryData>,
}
#[derive(Serialize, Debug, Deserialize, FromRow)]

pub struct InventoryItemView {
    pub id: i32,
    pub shop_id: i32,
    pub product_id: i32,
    pub product_name: String,
    pub price: i32,
    pub quantity: i32,
    pub image_path: Option<String>,
    pub image_url:Option<String>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Serialize, Debug, Deserialize, FromRow)]
pub struct ResponseInventoryViewData {
    pub status: i32,
    pub state: bool,
    pub msg: String,
    pub data_list: Vec<InventoryItemView>,
}
