use actix_web::{web, HttpResponse, post, get, put};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::jwt::Claims,
    error::AppError,
    models::user::{CreateUser, LoginCredentials, User},
};

#[post("/users/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    let user = User::create(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(user))
}

#[post("/users/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse, AppError> {
    let user = User::get_by_email(&pool, &credentials.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    if !user.verify_password(&credentials.password) {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let claims = Claims::new(user.id);
    let token = claims.generate_token()?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

#[get("/users/me")]
pub async fn get_me(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user = User::get_by_id(&pool, claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/me")]
pub async fn update_user(
    pool: web::Data<PgPool>,
    claims: Claims,
    form: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    let user = User::update(&pool, claims.sub, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}
