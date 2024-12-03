use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpRequest,
};
use futures::future::{ready, Ready};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use chrono::{Duration, Utc};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub user_id: Uuid, 
    pub role: String,
}

#[allow(dead_code)]
pub trait AuthService {
    fn generate_token(user_id: Uuid, role: &str) -> Result<String, AppError>;
    fn decode_token(token: &str) -> Result<Claims, AppError>;
}

pub struct JwtAuthService;

impl AuthService for JwtAuthService {
    fn generate_token(user_id: Uuid, role: &str) -> Result<String, AppError> { 
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: "auth".to_string(),
            exp: expiration as usize,
            user_id, 
            role: role.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
        )
        .map_err(|_| AppError::TokenCreationError)
    }

    fn decode_token(token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::InvalidToken)
    }
}

pub fn generate_token(user_id: Uuid, role: &str) -> Result<String, AppError> { 
    JwtAuthService::generate_token(user_id, role)
}

impl FromRequest for Claims {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        match token {
            Some(token) => {
                match JwtAuthService::decode_token(token) {
                    Ok(claims) => ready(Ok(claims)),
                    Err(_) => ready(Err(AppError::InvalidToken)),
                }
            },
            None => ready(Err(AppError::MissingToken)),
        }
    }
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".into()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".into()))
}
