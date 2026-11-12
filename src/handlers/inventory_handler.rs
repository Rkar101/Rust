use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::extractor::AuthUser,
    errors::product_error::ProductError,
    models::{InventoryData, ResponseInventoryData, ResponseInventoryViewData, ResponseStatus},
    services::inventory_service::InventoryService,
};

pub async fn get_inventory_handler(
    State(service): State<Arc<InventoryService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(shop_id): Json<Option<i32>>,
) -> Result<Json<ResponseInventoryViewData>, ProductError> {
    let data = service.get_inventory_data_service(role, shop_id).await?;
    Ok(Json(ResponseInventoryViewData {
        status: 201,
        state: true,
        msg: "Success".to_string(),
        data_list: data,
    }))
}

pub async fn add_inventory_product(
    State(service): State<Arc<InventoryService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(inv_data): Json<InventoryData>,
) -> Result<Json<ResponseStatus>, ProductError> {
    service
        .add_inventory_product_service(role, inv_data)
        .await?;
    Ok(Json(ResponseStatus {
        status: 201,
        state: true,
        msg: "Success.".to_string(),
    }))
}

pub async fn edit_inventory_product(
    State(service): State<Arc<InventoryService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(inv_data): Json<InventoryData>,
) -> Result<Json<ResponseStatus>, ProductError> {
    service
        .edit_inventory_product_service(role, inv_data)
        .await?;
    Ok(Json(ResponseStatus {
        status: 201,
        state: true,
        msg: "Update Successfully.".to_string(),
    }))
}
