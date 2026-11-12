use crate::auth::role_middleware::require_admin;
use axum::{
    Router, middleware,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    handlers::product_handler::{
        create_bulk_product_list, create_product, create_products_list, delete_product_by_id,
        get_product_by_id, get_products, patch_product_by_id,
    },
    services::product_service::ProductService,
};

pub fn product_routes(service: Arc<ProductService>) -> Router {
    Router::new()
        .route("/api001/product", get(get_products).post(create_product))
        .route(
            "/api001/product/{id}",
            get(get_product_by_id)
                .patch(patch_product_by_id)
                .delete(delete_product_by_id),
        )
        .route(
            "/service/admin/product/bulk",
            post(create_bulk_product_list).route_layer(middleware::from_fn(require_admin)),
        )
        .route("/service/admin/productList", post(create_products_list))
        .with_state(service)
}
