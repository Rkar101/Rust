use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    auth::extractor::AuthUser,
    errors::product_error::ProductError,
    models::{
        BulkProductRequest, CreateProduct, PatchProduct, Product, ProductQuery,
        ResponseProductListStatus, ResponseStatus,
    },
    services::product_service::ProductService,
};

pub async fn create_product(
    State(service): State<Arc<ProductService>>,
    AuthUser { user_id, role }: AuthUser,
    Json(product): Json<CreateProduct>,
) -> Result<Json<ResponseStatus>, ProductError> {
    service.create_product(product, role).await?;
    Ok(Json(ResponseStatus {
        status: 201,
        state: true,
        msg: "Created successfully".to_string(),
    }))
    // } else {
    //     Ok(Json(ResponseStatus {
    //         state: false,
    //         status: 403,
    //         msg: "You don't have permission. Code[403]".to_string(),
    //     }))
    // }
}

pub async fn create_bulk_product_list(
    State(service): State<Arc<ProductService>>,
    Json(bulk_product): Json<BulkProductRequest>,
) -> Result<Json<ResponseStatus>, ProductError> {
    match service.create_bulk_product_list(bulk_product).await {
        Ok(_) => Ok(Json(ResponseStatus {
            status: 201,
            state: true,
            msg: "Product list create successfully.".to_string(),
        })),
        Err(e) => {
            println!("GET PRODUCTS ERROR: {:?}", e);
            Err(ProductError::NotFound)
        }
    }
}

pub async fn get_products(
    State(service): State<Arc<ProductService>>,
    Query(pagination): Query<ProductQuery>,
    AuthUser { user_id, role }: AuthUser,
) -> Result<Json<ResponseProductListStatus>, StatusCode> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let name = pagination.name;
    let sort = pagination.sort;
    let order = pagination.order;
    let min_price = pagination.min_price;
    let max_price = pagination.max_price;
    match service
        .get_products(
            page, limit, name, sort, order, min_price, max_price, role, user_id,
        )
        .await
    {
        Ok(product_list) => Ok(Json(ResponseProductListStatus {
            status: 200,
            state: true,
            msg: "".to_string(),

            product_list: product_list,
        })),
        Err(e) => {
            println!("GET PRODUCTS ERROR: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_products_list(
    State(service): State<Arc<ProductService>>,
    Json(product_list): Json<Vec<CreateProduct>>,
) -> Result<Json<ResponseStatus>, ProductError> {
    service.create_product_list(product_list).await?;
    Ok(Json(ResponseStatus {
        status: 201,
        state: true,
        msg: "Created successfully".to_string(),
    }))
}

pub async fn get_product_by_id(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, ProductError> {
    println!("ID = {}", id);
    let product = service.get_product_by_id(id).await?;

    Ok(Json(product))
}

pub async fn patch_product_by_id(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<i32>,
    Json(product): Json<PatchProduct>,
) -> Result<Json<ResponseStatus>, ProductError> {
    match service.patch_product_by_id(id, &product).await {
        Ok(_) => Ok(Json(ResponseStatus {
            status: 200,
            state: true,
            msg: "Update Successfully.".to_string(),
        })),
        Err(e) => {
            println!("GET PRODUCTS ERROR: {:?}", e);
            Err(ProductError::NotFound)
        }
    }
}

pub async fn delete_product_by_id(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<i32>,
) -> Result<Json<ResponseStatus>, ProductError> {
    service.delete_product_by_id(id).await?;
    Ok(Json(ResponseStatus {
        status: 200,
        state: true,
        msg: "Delete Successfully.".to_string(),
    }))
}
