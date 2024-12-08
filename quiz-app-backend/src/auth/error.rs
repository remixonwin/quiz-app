use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum AuthError {
    #[display(fmt = "Invalid token")]
    InvalidToken,
    #[display(fmt = "Token expired")]
    TokenExpired,
    #[display(fmt = "Invalid credentials")]
    InvalidCredentials,
    #[display(fmt = "Hash error")]
    HashError,
    #[display(fmt = "Token creation failed")]
    TokenCreation,
    #[display(fmt = "User not found")]
    UserNotFound,
    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(ErrorResponse {
            message: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::HashError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
