use axum::{
    Json, http,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::models::ResponseStatus;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    Database(sqlx::Error),
    Validation(String),
}

impl From<sqlx::Error> for UserError {
    fn from(error: sqlx::Error) -> Self {
        UserError::Database(error)
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        match self {
            UserError::Validation(fail) => (
                StatusCode::NOT_ACCEPTABLE,
                Json(ResponseStatus {
                    state: false,
                    status: 400,
                    msg: "Invalid email or password.".to_string(),
                })
                .into_response(),
            )
                .into_response(),

            UserError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ResponseStatus {
                    state: false,
                    status: 404,
                    msg: "Invalid email or password.".to_string(),
                }),
            )
                .into_response(),

            UserError::Database(e) => {
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
