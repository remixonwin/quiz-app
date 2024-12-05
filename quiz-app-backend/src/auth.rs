use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
    FromRequest,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    body::EitherBody,
    http::header::AUTHORIZATION,
    web,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use chrono::{Duration, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use std::rc::Rc;

use crate::{config::Config, error::AppError, models::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    pub role: String,
    pub exp: i64,
}

pub trait AuthService {
    fn generate_token(config: &Config, user: &User) -> Result<String, AppError>;
    fn decode_token(token: &str, config: &Config) -> Result<Claims, AppError>;
}

pub struct JwtAuthService;

impl AuthService for JwtAuthService {
    fn generate_token(config: &Config, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            user_id: user.id,
            role: user.role.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::TokenCreationError)
    }

    fn decode_token(token: &str, config: &Config) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::InvalidToken)
    }
}

pub async fn generate_token(user: &User) -> Result<String, AppError> {
    let config = Config::new()?;
    Ok(JwtAuthService::generate_token(&config, user)?)
}

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    web::block(move || hash(password.as_bytes(), DEFAULT_COST))
        .await
        .map_err(AppError::from)?
        .map_err(|_| AppError::PasswordHashingError)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    web::block(move || verify(password.as_bytes(), hash))
        .await
        .map_err(AppError::from)?
        .map_err(|_| AppError::PasswordVerificationError)
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(auth_str) if auth_str.starts_with("Bearer ") => {
                auth_str[7..].to_string()
            }
            _ => return ready(Err(ErrorUnauthorized(json!({ 
                "error": "No valid authorization token" 
            })))),
        };

        let config = match Config::new() {
            Ok(config) => config,
            Err(_) => return ready(Err(ErrorUnauthorized(json!({
                "error": "Server configuration error"
            })))),
        };

        match JwtAuthService::decode_token(&token, &config) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.clone());
                ready(Ok(claims))
            }
            Err(_) => ready(Err(ErrorUnauthorized(json!({
                "error": "Invalid token"
            })))),
        }
    }
}

pub struct JwtMiddleware<S> {
    service: Rc<S>,
}

impl<S> JwtMiddleware<S> {
    pub fn new(service: S) -> Self {
        JwtMiddleware {
            service: Rc::new(service),
        }
    }
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let config = match Config::new() {
                Ok(config) => config,
                Err(_) => {
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(json!({
                                "error": "Server configuration error"
                            }))
                            .map_into_right_body(),
                    ))
                }
            };

            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok());

            let token = match auth_header {
                Some(auth_str) if auth_str.starts_with("Bearer ") => auth_str[7..].to_string(),
                _ => {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(json!({
                                "error": "Missing authorization token"
                            }))
                            .map_into_right_body(),
                    ))
                }
            };

            match JwtAuthService::decode_token(&token, &config) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    let res = svc.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(_) => Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "error": "Invalid token"
                        }))
                        .map_into_right_body(),
                )),
            }
        })
    }
}
