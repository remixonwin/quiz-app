use crate::{
    auth::{generate_token, Claims},
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
    Ok(hash(password, DEFAULT_COST)?)
}

fn verify_password(password: &str, hashed_password: &str) -> Result<()> {
    verify(password, hashed_password)?;
    Ok(())
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse> {
    let hashed_password = hash_password(&user_data.password)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        user_data.username,
        hashed_password,
        "user",
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .await?;

    let token = generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Created().json(json!({
        "id": user.id,
        "username": user.username,
        "role": user.role,
        "token": token
    })))
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
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    verify_password(&credentials.password, &user.password_hash)?;

    let token = generate_token(user.id, &user.role)?;

    Ok(HttpResponse::Ok().json(json!({
        "id": user.id,
        "username": user.username,
        "role": user.role,
        "token": token
    })))
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

    Ok(HttpResponse::Ok().json(json!({
        "id": user.id,
        "username": user.username,
        "role": user.role
    })))
}

#[put("/me")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    update_data: web::Json<UpdateUser>,
    claims: Claims,
) -> Result<HttpResponse> {
    let mut query_str = String::from("UPDATE users SET updated_at = CURRENT_TIMESTAMP");
    let mut values: Vec<String> = vec![];

    if let Some(username) = &update_data.username {
        query_str.push_str(", username = $1");
        values.push(username.clone());
    }

    if let Some(password) = &update_data.password {
        let hashed_password = hash_password(password)?;
        if values.is_empty() {
            query_str.push_str(", password_hash = $1");
        } else {
            query_str.push_str(", password_hash = $2");
        }
        values.push(hashed_password);
    }

    if values.is_empty() {
        return Err(AppError::BadRequest("No fields to update".into()));
    }

    query_str.push_str(" WHERE id = $");
    query_str.push_str(&(values.len() + 1).to_string());
    query_str.push_str(" RETURNING id, username, password_hash, role, created_at, updated_at");

    let mut query = query_as::<_, User>(&query_str);
    for value in &values {
        query = query.bind(value);
    }
    query = query.bind(claims.user_id);

    let user = query.fetch_one(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(json!({
        "id": user.id,
        "username": user.username,
        "role": user.role
    })))
}
