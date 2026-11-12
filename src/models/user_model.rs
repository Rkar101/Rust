use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing_subscriber::field::debug;
#[derive(Debug, sqlx::FromRow)]
pub struct UserInfo {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: u16,
    pub state: bool,
    pub msg: String,
    pub token: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub user_name: String,
    pub email: String,
    pub join_date: String,
}
