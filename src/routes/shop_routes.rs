use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::shopdata_handler::{
        create_shop_handler, edit_shop_handler, get_shop_byowner, get_shop_products_handler,
    },
    services::shop_service::ShopDataService,
};

pub fn shop_routes(service: Arc<ShopDataService>) -> Router {
    Router::new()
        .route(
            "/api001/service/shop",
            get(get_shop_byowner).post(create_shop_handler),
        )
        .route(
            "/api001/service/editShop",
            get(get_shop_byowner).post(edit_shop_handler),
        )
        .route(
            "/api001/service/ShopProducts",
            post(get_shop_products_handler),
        )
        .with_state(service)
}
