use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use crate::auth::extractor::{AuthRole, AuthUser};


pub async fn require_admin(
    auth_user: AuthUser,
    request: Request,
    next: Next,
) -> Response {
    if auth_user.role != AuthRole::Admin {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "state": false,
                "status": 403,
                "msg": "Admin access required"
            })),
        )
            .into_response();
    }

    next.run(request).await
}