use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::convert::From;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JsonWebTokenError;
use uuid::Error as UuidError;

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    ValidationError(String),
    AuthError(String),
    AuthenticationError(String),
    InternalServerError(String),
    HashError,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };
        match self {
            AppError::NotFound(_) => HttpResponse::NotFound().json(error_response),
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
            AppError::Unauthorized(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::Forbidden(_) => HttpResponse::Forbidden().json(error_response),
            AppError::ValidationError(_) => HttpResponse::BadRequest().json(error_response),
            AppError::AuthError(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::AuthenticationError(_) => HttpResponse::Unauthorized().json(error_response),
            AppError::InternalServerError(_) => HttpResponse::InternalServerError().json(error_response),
            AppError::HashError => HttpResponse::InternalServerError().json(error_response),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::HashError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<JsonWebTokenError> for AppError {
    fn from(err: JsonWebTokenError) -> Self {
        AppError::AuthError(err.to_string())
    }
}

impl From<BcryptError> for AppError {
    fn from(_err: BcryptError) -> Self {
        AppError::HashError
    }
}

impl From<UuidError> for AppError {
    fn from(err: UuidError) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

impl From<crate::auth::error::AuthError> for AppError {
    fn from(err: crate::auth::error::AuthError) -> Self {
        match err {
            crate::auth::error::AuthError::InvalidToken => AppError::AuthError("Invalid token".to_string()),
            crate::auth::error::AuthError::TokenExpired => AppError::AuthError("Token expired".to_string()),
            crate::auth::error::AuthError::TokenCreation => AppError::InternalServerError("Failed to create token".to_string()),
            crate::auth::error::AuthError::InvalidCredentials => AppError::Unauthorized("Invalid credentials".to_string()),
            crate::auth::error::AuthError::HashError => AppError::InternalServerError("Hash error".to_string()),
            crate::auth::error::AuthError::UserNotFound => AppError::NotFound("User not found".to_string()),
            crate::auth::error::AuthError::Unauthorized => AppError::Unauthorized("Unauthorized".to_string()),
        }
    }
}
