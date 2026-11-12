use axum::{
    Json, http,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::models::ResponseStatus;

pub enum ShopDataError {
    NotFound,
    DatabaseError(sqlx::Error),
    Validation(String),
}

impl From<sqlx::Error> for ShopDataError {
    fn from(error: sqlx::Error) -> Self {
        ShopDataError::DatabaseError(error)
    }
}

impl IntoResponse for ShopDataError {
    fn into_response(self) -> Response {
        match self {
            ShopDataError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ResponseStatus {
                    state: false,
                    status: 404,
                    msg: "No data found".to_string(),
                }),
            )
                .into_response(),

            ShopDataError::DatabaseError(error) => {
                println!("ShopDataError::DatabaseError: {:?}", error);

                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ResponseStatus {
                        state: false,
                        status: 502,
                        msg: "Something Went Wrong. [ s_e:39 ]".to_string(),
                    }),
                )
                    .into_response()
            }

            ShopDataError::Validation(e) => (
                StatusCode::NOT_ACCEPTABLE,
                Json(ResponseStatus {
                    state: false,
                    status: 400,
                    msg: e.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
