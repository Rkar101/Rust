use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{
    handlers::inventory_handler::{
        add_inventory_product, edit_inventory_product, get_inventory_handler,
    },
    services::inventory_service::InventoryService,
};

pub fn inventory_routes(service: Arc<InventoryService>) -> Router {
    Router::new()
        .route("/api001/service/inventory", post(get_inventory_handler))
        .route("/api001/service/inventory/add", post(add_inventory_product))
        .route(
            "/api001/service/inventory/update",
            post(edit_inventory_product),
        )
        .with_state(service)
}
