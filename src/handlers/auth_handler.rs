use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::extractor::AuthUser,
    errors::auth_error::UserError,
    models::{LoginRequest, LoginResponse, RegisterRequest, ResponseStatus, UserProfile},
    services::user_service::UserRegisterService,
};

pub async fn create_user(
    State(service): State<Arc<UserRegisterService>>,
    Json(user): Json<RegisterRequest>,
) -> Result<Json<ResponseStatus>, UserError> {
    service.create_user(user).await?;
    Ok(Json(ResponseStatus {
        status: 201,
        state: true,
        msg: "User created successfully.".to_string(),
    }))
}

pub async fn login_user(
    State(service): State<Arc<UserRegisterService>>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, UserError> {
    let access_token = service.login_user(login_req).await?;
    Ok(Json(LoginResponse {
        status: 200,
        state: true,
        msg: "Login Successfully.".to_string(),
        token: access_token.0,
        role: access_token.1,
    }))
}

pub async fn get_user_data(
    State(service): State<Arc<UserRegisterService>>,
    AuthUser { user_id, role }: AuthUser,
) -> Result<Json<UserProfile>, UserError> {
    let data = service.get_user_data(user_id).await?;
    Ok(Json(UserProfile {
        user_name: data.0,
        email: data.1,
        join_date: data.2,
    }))
}
