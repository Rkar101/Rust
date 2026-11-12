use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::extractor::AuthUser,
    errors::shop_error::ShopDataError,
    models::{
        ResponseShopData, ResponseShopListStatus, ResponseShopProducts, ResponseStatus, ShopData,
    },
    services::shop_service::ShopDataService,
};

pub async fn create_shop_handler(
    State(service): State<Arc<ShopDataService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(shop_data): Json<ShopData>,
) -> Result<Json<ResponseStatus>, ShopDataError> {
    service
        .create_shop_service(shop_data, user_id, role)
        .await?;

    Ok(Json(ResponseStatus {
        status: 202,
        state: true,
        msg: "Created Successfully.".to_string(),
    }))
}

pub async fn edit_shop_handler(
    State(service): State<Arc<ShopDataService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(shop_data): Json<ResponseShopData>,
) -> Result<Json<ResponseStatus>, ShopDataError> {
    service.edit_shop_service(shop_data, role).await?;

    Ok(Json(ResponseStatus {
        status: 202,
        state: true,
        msg: "Created Successfully.".to_string(),
    }))
}

pub async fn get_shop_byowner(
    State(service): State<Arc<ShopDataService>>,
    AuthUser { user_id, role }: AuthUser,
) -> Result<Json<ResponseShopListStatus>, ShopDataError> {
    let shop_data = service.get_shop_byowner(user_id, role).await?;

    Ok(Json(ResponseShopListStatus {
        status: 200,
        state: true,
        msg: "Confirm".to_string(),
        shop_list: shop_data,
    }))
}

pub async fn get_shop_products_handler(
    State(service): State<Arc<ShopDataService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(shop): Json<ResponseShopData>,
) -> Result<Json<ResponseShopProducts>, ShopDataError> {
    let product_data = service
        .get_shop_products_service(role, shop.shop_id)
        .await?;

    Ok(Json(ResponseShopProducts {
        status: 200,
        state: true,
        msg: "".to_string(),
        product_list: product_data,
    }))
}
