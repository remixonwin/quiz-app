use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::{self, Claims, generate_token, hash_password, verify_password},
    error::AppError,
    models::{CreateUser, LoginCredentials, User},
};

#[derive(serde::Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    // Hash the password
    let password_hash = hash_password(&form.password).await?;

    // Create the user
    let user = User::create(&pool, CreateUser {
        username: form.username.clone(),
        email: form.email.clone(),
        password: password_hash,
    }).await?;

    // Generate a token
    let token = generate_token(&user)?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "token": token
    })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse, AppError> {
    // Find the user by username
    let user = User::get_by_username(&pool, &credentials.username).await
        .map_err(|_| AppError::InvalidCredentials)?;

    // Verify the password
    let is_valid = verify_password(&credentials.password, &user.password_hash).await?;
    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Generate a token
    let token = generate_token(&user)?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "token": token
    })))
}

pub async fn me(claims: Claims, pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let user = User::get_by_id(&pool, claims.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}
