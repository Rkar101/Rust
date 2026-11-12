use axum::{
    Json, http,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::models::ResponseStatus;

#[derive(Debug)]
pub enum ProductError {
    NotFound,
    Database(sqlx::Error),
    Validation(String),
}

impl From<sqlx::Error> for ProductError {
    fn from(error: sqlx::Error) -> Self {
        ProductError::Database(error)
    }
}

impl IntoResponse for ProductError {
    fn into_response(self) -> Response {
        match self {
            ProductError::Validation(fail) => (
                StatusCode::NOT_ACCEPTABLE,
                Json(ResponseStatus {
                    state: false,
                    status: 400,
                    msg: fail.to_string(),
                }),
            )
                .into_response(),

            ProductError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ResponseStatus {
                    state: false,
                    status: 404,
                    msg: "No data found".to_string(),
                }),
            )
                .into_response(),

            ProductError::Database(e) => {
                println!("Database error: {:?}", e);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseStatus {
                        state: false,
                        status: 500,
                        msg: "Internal server error".to_string(),
                    }),
                )
                    .into_response()
            }
        }
    }
}
