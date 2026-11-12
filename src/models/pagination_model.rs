use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total_items: i64,
    pub total_pages: i64,
}
