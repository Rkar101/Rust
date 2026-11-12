use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{str::FromStr, sync::Arc};

mod auth;
mod database;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use repositories::ProductRepository;
use routes::product_routes::product_routes;

use crate::{
    database::{
        emp_shop_table::emp_shop_table_create, inventory_table::inventory_table_create,
        product_table::product_table_create, sale_items_table::sale_item_table_create,
        sales_table::sales_table_create, shop_table::shop_table_create,
        user_table::user_table_create,
    },
    repositories::{
        admin_repository::AdminRepository, inventory_repository::InventoryRepo,
        shop_repository::ShopRepository, user_repository::UserRepository,
    },
    routes::{
        admin_routes::admin_routes, auth_routes::auth_routes, inventory_routes::inventory_routes,
        shop_routes::shop_routes,
    },
    services::{
        admin_service::{self, AdminService},
        inventory_service::InventoryService,
        product_service::ProductService,
        shop_service::ShopDataService,
        storage_service::SupabaseStorage,
        user_service::UserRegisterService,
    },
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("S_LINK").unwrap();

    let local_url = std::env::var("LOCAL_ADDRESS_URL").unwrap();

    println!("URL Connected.",);
    println!("DATABASE URL: {}", database_url);

    let connect_options = PgConnectOptions::from_str(&database_url)?.statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect_with(connect_options)
        .await?;

    user_table_create(pool.clone()).await?;
    println!("User table is ready!");

    shop_table_create(pool.clone()).await?;
    println!("Shop table is ready!");

    emp_shop_table_create(pool.clone()).await?;
    println!("Emp-Shop table is ready!");

    product_table_create(pool.clone()).await?;
    println!("Products table is ready!");

    sales_table_create(pool.clone()).await?;
    println!("Sales table is ready!");

    sale_item_table_create(pool.clone()).await?;
    println!("Sale Items table is ready!");

    inventory_table_create(pool.clone()).await?;
    println!("Inventory table is ready!");

    let invent_repo = InventoryRepo::new(pool.clone());

    let shop_repo = ShopRepository::new(pool.clone());

    let admin_repo = AdminRepository::new(pool.clone());

    let product_repo = ProductRepository::new(pool.clone());

    let user_repo = UserRepository::new(pool.clone());

    // ----------------------------------------------------------------------------------------

    let storage = SupabaseStorage::new();

    let invent_service = InventoryService::new(invent_repo, storage);

    let admin_service = AdminService::new(admin_repo);

    let shop_service = ShopDataService::new(shop_repo);

    let product_service = ProductService::new(product_repo, pool);

    let auth_service = UserRegisterService::new(user_repo);

    // ----------------------------------------------------------------------------------------

    let invent_service_arc = Arc::new(invent_service);

    let admin_service_arc = Arc::new(admin_service);

    let product_service_arc = Arc::new(product_service);

    let auth_service_arc = Arc::new(auth_service);

    let shop_data_service = Arc::new(shop_service);

    // ----------------------------------------------------------------------------------------

    let app_router = Router::new()
        .merge(auth_routes(auth_service_arc))
        .merge(admin_routes(admin_service_arc))
        .merge(shop_routes(shop_data_service))
        .merge(product_routes(product_service_arc))
        .merge(inventory_routes(invent_service_arc));

    let listener = tokio::net::TcpListener::bind(&local_url).await.unwrap();

    println!("Server running on http://{local_url}");

    axum::serve(listener, app_router).await.unwrap();

    Ok(())
}
