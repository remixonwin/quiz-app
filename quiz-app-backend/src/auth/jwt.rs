use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpRequest, HttpMessage};
use futures::future::{ready, Ready};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            sub: user_id,
            exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
        }
    }

    pub fn generate_token(&self) -> Result<String, AppError> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-256-bit-secret".to_string());
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Get claims from request extensions (set by Auth middleware)
        match req.extensions().get::<Claims>() {
            Some(claims) => ready(Ok(claims.clone())),
            None => ready(Err(ErrorUnauthorized("Not authenticated"))),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    TokenCreation,
    InvalidCredentials,
}

pub fn validate_token(token: &str) -> Result<Claims, AuthError> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-256-bit-secret".to_string());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AuthError::InvalidToken)
}

// Password hashing functions
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    hash(password.as_bytes(), DEFAULT_COST)
        .map_err(|_| AuthError::TokenCreation) // Reusing TokenCreation error for now
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    verify(password.as_bytes(), hash)
        .map_err(|_| AuthError::InvalidCredentials)
}
