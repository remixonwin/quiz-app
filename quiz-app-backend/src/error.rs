use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde_json::json;
use sqlx::error::Error as SqlxError;
use std::convert::From;
use tokio::task::JoinError;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Configuration Error: {}", _0)]
    ConfigError(String),

    #[display(fmt = "Token Creation Error")]
    TokenCreationError,

    #[display(fmt = "Invalid Token")]
    InvalidToken,

    #[display(fmt = "Password Hashing Error")]
    PasswordHashingError,

    #[display(fmt = "Password Verification Error")]
    PasswordVerificationError,

    #[display(fmt = "Forbidden: {}", _0)]
    Forbidden(String),

    #[display(fmt = "Hash Error")]
    HashError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": self.to_string()
                }))
            }
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(json!({
                    "error": message
                }))
            }
            AppError::Unauthorized(ref message) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": message
                }))
            }
            AppError::NotFound(ref message) => {
                HttpResponse::NotFound().json(json!({
                    "error": message
                }))
            }
            AppError::DatabaseError(ref message) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": format!("Database error: {}", message)
                }))
            }
            AppError::ConfigError(ref message) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": format!("Configuration error: {}", message)
                }))
            }
            AppError::TokenCreationError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Could not create authentication token"
                }))
            }
            AppError::InvalidToken => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid or expired token"
                }))
            }
            AppError::PasswordHashingError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to hash password"
                }))
            }
            AppError::PasswordVerificationError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to verify password"
                }))
            }
            AppError::Forbidden(ref message) => {
                HttpResponse::Forbidden().json(json!({
                    "error": message
                }))
            }
            AppError::HashError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Hash error"
                }))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TokenCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::PasswordHashingError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordVerificationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::HashError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> AppError {
        match error {
            SqlxError::RowNotFound => {
                AppError::NotFound("Requested resource not found".to_string())
            }
            SqlxError::Database(ref err) => {
                if err.code().as_deref() == Some("23505") {
                    AppError::BadRequest("Resource already exists".to_string())
                } else {
                    AppError::DatabaseError(err.to_string())
                }
            }
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}

impl From<JoinError> for AppError {
    fn from(_: JoinError) -> AppError {
        AppError::InternalServerError
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> AppError {
        AppError::ConfigError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> AppError {
        AppError::InvalidToken
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> AppError {
        AppError::PasswordHashingError
    }
}
