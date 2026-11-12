use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    handlers::admin_handler::{get_admin_dashboard, get_category},
    services::admin_service::AdminService,
};

pub fn admin_routes(service: Arc<AdminService>) -> Router {
    Router::new()
        .route("/api001/auth/admin/dashboard", get(get_admin_dashboard))
        .route("/api001/admin/category", get(get_category))
        .with_state(service)
}
