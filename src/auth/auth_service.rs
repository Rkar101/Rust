use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub role: String,
}

pub fn generate_token(user_id: i32,role :String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        role: role
    };

    let secret = "flu-rust-my-key-zero-0-zero-1";

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(jwt_token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = "flu-rust-my-key-zero-0-zero-1";

    let token_data = decode::<Claims>(
        jwt_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
