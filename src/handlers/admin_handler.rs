use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::extractor::AuthUser,
    errors::auth_error::UserError,
    models::{ResponseAdminStatus, ResponseCategory},
    services::admin_service::AdminService,
};

pub async fn get_admin_dashboard(
    State(service): State<Arc<AdminService>>,
    AuthUser { user_id, role }: AuthUser,
) -> Result<Json<ResponseAdminStatus>, UserError> {
    match role {
        crate::auth::extractor::AuthRole::Admin => {
            let dashboard = service.get_admin_dashboard(role, user_id).await?;
            Ok(Json(ResponseAdminStatus {
                status: 200,
                state: true,
                msg: "".to_string(),
                data: Some(dashboard),
            }))
        }
        crate::auth::extractor::AuthRole::Owner => {
           let dashboard = service.get_admin_dashboard(role, user_id).await?;
            Ok(Json(ResponseAdminStatus {
                status: 200,
                state: true,
                msg: "".to_string(),
                data: Some(dashboard),
            }))
        }
        crate::auth::extractor::AuthRole::Employee => {
            Ok(Json(ResponseAdminStatus {
                status: 200,
                state: false,
                msg: "You don't have permission.".to_string(), // Fixed typo "Yon" -> "You"
                data: None,                                    // Fixed: replaced todo!() with None
            }))
        }
    }
}

pub async fn get_category(
    State(service): State<Arc<AdminService>>,
    AuthUser { user_id, role }: AuthUser,
) -> Result<Json<ResponseCategory>, UserError> {
    match role {
        crate::auth::extractor::AuthRole::Admin => {
            let data = service.get_admin_categories().await?;
            Ok(Json(ResponseCategory {
                status: 200,
                state: true,
                msg: "".to_string(),

                data: Some(data),
            }))
        }
        crate::auth::extractor::AuthRole::Owner => {
            let data = service.get_admin_categories().await?;
            Ok(Json(ResponseCategory {
                status: 200,
                state: true,
                msg: "".to_string(),

                data: Some(data),
            }))
        }
        crate::auth::extractor::AuthRole::Employee => Ok(Json(ResponseCategory {
            status: 403,
            state: false,
            msg: "You don't have permission.".to_string(),
            data: None,
        })),
    }
}
