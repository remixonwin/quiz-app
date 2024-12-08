use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password.as_bytes(), DEFAULT_COST)
        .map_err(|_| AppError::InternalServerError("Failed to hash password".to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password.as_bytes(), hash)
        .map_err(|_| AppError::InternalServerError("Failed to verify password".to_string()))
}
