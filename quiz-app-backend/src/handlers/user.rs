use crate::{
    auth::Claims,
    error::AppError,
    models::{CreateUser, LoginCredentials, UpdateUser, User},
};
use actix_web::{get, post, put, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use serde_json::json;
use sqlx::{postgres::PgPool, query_as};

type Result<T> = std::result::Result<T, AppError>;

fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|_| AppError::HashError)
}

fn verify_password(password: &str, hashed_password: &str) -> Result<()> {
    if verify(password, hashed_password).map_err(|_| AppError::InvalidCredentials)? {
        Ok(())
    } else {
        Err(AppError::InvalidCredentials)
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse> {
    let hashed_password = hash_password(&user_data.password)?;
    let now = Utc::now();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, 'user', $3, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        user_data.username,
        hashed_password,
        now
    )
    .fetch_one(pool.get_ref())
    .await?;

    let token = crate::auth::generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Created().json(json!({ "token": token })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_one(pool.get_ref())
    .await?;

    verify_password(&credentials.password, &user.password_hash)?;

    let token = crate::auth::generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}

#[get("/me")]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
        claims.user_id
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(user))
}

#[put("/me")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    claims: Claims,
    user_data: web::Json<UpdateUser>,
) -> Result<HttpResponse> {
    let now = Utc::now();

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET username = COALESCE($1, username),
            password_hash = COALESCE($2, password_hash),
            updated_at = $3
        WHERE id = $4
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        user_data.username,
        user_data.password.map(|p| hash_password(&p)).transpose()?,
        now,
        claims.user_id
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(user))
}
