use actix_web::{web, HttpResponse, Responder, post, get, put, delete};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth,
    error::AppError,
    models::{CreateUser, UpdateUser, User},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    let user = User::create(&pool, user_data.0).await?;
    let token = auth::generate_token(&user).await?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user = User::find_by_email(&pool, &login_data.email).await?;
    
    if !auth::verify_password(&login_data.password, &user.password_hash).await? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = auth::generate_token(&user).await?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

#[get("/profile")]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user = User::find_by_id(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/profile")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
    user_data: web::Json<UpdateUser>,
) -> Result<impl Responder, AppError> {
    let user = User::update(&pool, user_id.into_inner(), user_data.0).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/profile")]
pub async fn delete_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    User::delete(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
