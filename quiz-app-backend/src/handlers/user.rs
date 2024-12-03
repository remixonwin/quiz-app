use crate::{
    auth::{Claims, hash_password, verify_password, generate_token},
    error::AppError,
    models::{CreateUser, LoginCredentials, UpdateUser, User, DbModel},
};
use actix_web::{web, HttpResponse, post, put, get};
use chrono::Utc;
use serde_json::json;
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> std::result::Result<HttpResponse, AppError> {
    let password_hash = hash_password(&user_data.password)?;
    let now = Utc::now().naive_utc();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, 'user', $3, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        user_data.username,
        password_hash,
        now
    )
    .fetch_one(pool.get_ref())
    .await?;

    let token = generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Created().json(json!({ "token": token })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> std::result::Result<HttpResponse, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password_hash, role, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        credentials.username
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AppError::Unauthorized("Invalid credentials".to_string()))?;

    verify_password(&credentials.password, &user.password_hash)?;

    let token = generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Ok().json(json!({ "token": token })))
}

#[get("/profile")]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> std::result::Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InvalidUuid)?;
    let user = User::get_by_id(&pool, user_id).await?;
    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/profile")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    form: web::Json<UpdateUser>,
    claims: Claims,
) -> std::result::Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
    
    let mut user_data = form.into_inner();
    user_data.id = Some(user_id);
    let user = User::update(&pool, user_data).await?;
    Ok(HttpResponse::Ok().json(user))
}
