use crate::auth::{generate_token, Claims};
use crate::models::{CreateUser, LoginCredentials, User};
use actix_web::{
    web, 
    Error, 
    HttpResponse, 
    error::{
        ErrorInternalServerError, 
        ErrorUnauthorized, 
        ErrorConflict
    }
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sqlx::PgPool;
use serde_json::json;

pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<HttpResponse, Error> {
    let hashed_password = hash(user_data.password.as_bytes(), DEFAULT_COST)
        .map_err(ErrorInternalServerError)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING 
            id, 
            username, 
            email, 
            password_hash, 
            created_at
        "#,
        user_data.username,
        user_data.email,
        hashed_password,
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        if e.to_string().contains("unique constraint") {
            ErrorConflict("Username or email already exists")
        } else {
            ErrorInternalServerError(e)
        }
    })?;

    let token = generate_token(user.id)?;

    Ok(HttpResponse::Created().json(json!({
        "user": user,
        "token": token
    })))
}

pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, 
            username, 
            email, 
            password_hash, 
            created_at 
        FROM users
        WHERE email = $1
        "#,
        credentials.email
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorUnauthorized("Invalid credentials"))?;

    let valid = verify(credentials.password.as_bytes(), &user.password_hash)
        .map_err(ErrorInternalServerError)?;

    if !valid {
        return Err(ErrorUnauthorized("Invalid credentials"));
    }

    let token = generate_token(user.id)?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "token": token
    })))
}

pub async fn get_profile(
    pool: web::Data<PgPool>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, 
            username, 
            email, 
            password_hash, 
            created_at 
        FROM users
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorUnauthorized("User not found"))?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn update_profile(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, Error> {
    let hashed_password = hash(user_data.password.as_bytes(), DEFAULT_COST)
        .map_err(ErrorInternalServerError)?;

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET 
            username = $1, 
            email = $2, 
            password_hash = $3
        WHERE id = $4
        RETURNING 
            id, 
            username, 
            email, 
            password_hash, 
            created_at
        "#,
        user_data.username,
        user_data.email,
        hashed_password,
        claims.sub
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
