use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::auth_handler::{create_user, get_user_data, login_user},
    services::user_service::UserRegisterService,
};

pub fn auth_routes(service: Arc<UserRegisterService>) -> Router {
    Router::new()
        .route("/api001/auth/register", post(create_user))
        .route("/api001/auth/login", post(login_user))
        .route("/api001/auth/profile", get(get_user_data))
        .with_state(service)
}
