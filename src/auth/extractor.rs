use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthUser {
    pub user_id: i32,
    pub role: AuthRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthRole {
    Admin,
    Owner,
    Employee,
}

impl std::str::FromStr for AuthRole {
    type Err = ();

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            "admin" => Ok(AuthRole::Admin),
            "owner" => Ok(AuthRole::Owner),
            "employee" => Ok(AuthRole::Employee),
            _ => Err(()),
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {

        // Get Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Convert header to string
        let auth_header = auth_header
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Get token from "Bearer <token>"
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Verify JWT
        let claims = super::auth_service::verify_token(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Convert JWT role String → AuthRole
        let role = claims
            .role
            .parse::<AuthRole>()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Create authenticated user
        Ok(AuthUser {
            user_id: claims.sub,
            role,
        })
    }
}