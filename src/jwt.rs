use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub exp: i64, // expiration time
}

pub fn create_token(user_id: i32) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub fn verify_token(token: &str) -> Result<Claims, Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ErrorUnauthorized(e.to_string()))
}

pub fn get_token_from_headers(req: &HttpRequest) -> Result<String, Error> {
    req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_header| {
            if auth_header.starts_with("Bearer ") {
                Some(auth_header[7..].to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| ErrorUnauthorized("No valid Authorization header found"))
}