use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
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

    #[error("Not Found: {0}")]
    NotFound(String),

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
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();
        
        HttpResponse::build(status).json(ErrorResponse { message })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
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
