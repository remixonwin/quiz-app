use actix_web::{
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::HeaderValue,
    Error, FromRequest, HttpRequest, HttpMessage,
};
use futures::{
    future::{err, ok, Ready, ready},
    Future,
};
use std::{
    env,
    pin::Pin,
    task::{Context, Poll},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub role: String,
    pub exp: usize,
}

#[allow(dead_code)]
pub trait AuthService {
    fn generate_token(user_id: i32, role: String) -> Result<String, AppError>;
}

pub struct JwtAuthService;

impl AuthService for JwtAuthService {
    fn generate_token(user_id: i32, role: String) -> Result<String, AppError> {
        if user_id <= 0 {
            return Err(AppError::ValidationError("Invalid user ID".to_string()));
        }

        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            user_id,
            role,
            exp: expiration,
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalServerError("Failed to create token".to_string()))
    }
}

pub fn generate_token(user_id: i32, role: &str) -> Result<String, AppError> {
    if user_id <= 0 {
        return Err(AppError::ValidationError("Invalid user ID".to_string()));
    }

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        user_id,
        role: role.to_string(),
        exp: expiration as usize,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET must be set").as_bytes()),
    )
    .map_err(|_| AppError::TokenCreationError)?;

    Ok(token)
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        println!("=== Processing auth request ===");
        let auth_header: Option<&HeaderValue> = req.headers().get("Authorization");

        match auth_header {
            Some(auth_str) => {
                println!("Found auth header: {:?}", auth_str);
                let auth_str = auth_str.to_str().unwrap_or("");
                if !auth_str.starts_with("Bearer ") {
                    println!("Invalid token format");
                    return err(ErrorUnauthorized("Invalid token format"));
                }

                let token = &auth_str[7..];
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(data) => {
                        println!("Token decoded successfully: {:?}", data.claims);
                        ok(data.claims)
                    },
                    Err(e) => {
                        println!("Token validation error: {:?}", e);
                        err(ErrorUnauthorized("Invalid token"))
                    },
                }
            }
            None => {
                println!("No authorization header found");
                err(ErrorUnauthorized("No authorization header"))
            }
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
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddleware<S> {
    #[allow(dead_code)]
    service: S,
}

impl<S> AuthMiddleware<S> {
    #[allow(dead_code)]
    pub fn new(service: S) -> Self {
        Self { service }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ").trim();
                    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(data) => {
                            req.extensions_mut().insert(data.claims);
                            return Box::pin(self.service.call(req));
                        }
                        Err(e) => {
                            println!("Token validation error: {:?}", e);
                            return Box::pin(ready(Err(ErrorUnauthorized("Invalid token"))))
                        }
                    }
                }
            }
        }
        Box::pin(ready(Err(ErrorUnauthorized("Missing or invalid authorization header"))))
    }
}
