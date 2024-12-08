use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    web,
    HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use crate::{
    auth::jwt::{Claims, validate_token},
    error::AppError,
};

pub struct Auth;

impl Auth {
    pub fn new() -> Self {
        Auth
    }
}

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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let auth_header = match req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer ")) {
            Some(token) => token,
            None => return Box::pin(async {
                Err(AppError::Unauthorized("Missing or invalid authorization header".into()).into())
            }),
        };

        // Validate the token and extract claims
        let claims = match validate_token(auth_header) {
            Ok(claims) => claims,
            Err(_) => return Box::pin(async {
                Err(AppError::Unauthorized("Invalid token".into()).into())
            }),
        };

        // Store claims in request extensions
        req.extensions_mut().insert(claims.clone());

        // Call the next service
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
