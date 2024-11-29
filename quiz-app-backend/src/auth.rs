use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::{
    encode, 
    EncodingKey, 
    Header
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{Utc, Duration};

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum AuthError {
    #[error("Unauthorized access")]
    Unauthorized,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,  // user id
    pub exp: i64,
}

pub fn generate_token(user_id: i32) -> Result<String, AuthError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| AuthError::Unauthorized)?;
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    ).map_err(|_| AuthError::Unauthorized)
}
