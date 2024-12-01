use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::convert::From;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Database Error: {0}")]
    DatabaseError(SqlxError),

    #[error("Bcrypt Error: {0}")]
    BcryptError(bcrypt::BcryptError),

    #[error("Json Web Token Error: {0}")]
    JsonWebTokenError(jsonwebtoken::errors::Error),

    #[error("Environment Variable Error: {0}")]
    EnvironmentVariableError(std::env::VarError),

    #[error("Json Error: {0}")]
    JsonError(serde_json::Error),

    #[error("Invalid Credentials")]
    InvalidCredentials,

    #[error("Token Creation Error")]
    TokenCreationError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError(msg) => HttpResponse::InternalServerError().json(ErrorResponse { error: msg.clone() }),
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorResponse { error: msg.clone() }),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorResponse { error: msg.clone() }),
            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(ErrorResponse { error: msg.clone() }),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(ErrorResponse { error: msg.clone() }),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(ErrorResponse { error: msg.clone() }),
            AppError::DatabaseError(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Database error".to_string() }),
            AppError::BcryptError(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Bcrypt error".to_string() }),
            AppError::JsonWebTokenError(_) => HttpResponse::Unauthorized().json(ErrorResponse { error: "Json Web Token error".to_string() }),
            AppError::EnvironmentVariableError(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Environment Variable error".to_string() }),
            AppError::JsonError(_) => HttpResponse::BadRequest().json(ErrorResponse { error: "Json error".to_string() }),
            AppError::InvalidCredentials => HttpResponse::Unauthorized().json(ErrorResponse { error: "Invalid credentials".to_string() }),
            AppError::TokenCreationError => HttpResponse::InternalServerError().json(ErrorResponse { error: "Token creation error".to_string() }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonWebTokenError(_) => StatusCode::UNAUTHORIZED,
            AppError::EnvironmentVariableError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JsonError(_) => StatusCode::BAD_REQUEST,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::TokenCreationError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> AppError {
        AppError::DatabaseError(error)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> AppError {
        AppError::BcryptError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> AppError {
        AppError::JsonWebTokenError(error)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> AppError {
        AppError::EnvironmentVariableError(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> AppError {
        AppError::JsonError(error)
    }
}
