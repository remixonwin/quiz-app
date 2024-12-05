use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Quiz, CreateQuiz, UpdateQuiz},
};

#[get("")]
pub async fn get_quizzes(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let quizzes = Quiz::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(quizzes))
}

#[post("")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    claims: Claims,
    quiz_data: web::Json<CreateQuiz>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::create(&pool, claims.user_id, quiz_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(quiz))
}

#[get("/{quiz_id}")]
pub async fn get_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(quiz))
}

#[put("/{quiz_id}")]
pub async fn update_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
    quiz_data: web::Json<UpdateQuiz>,
) -> Result<HttpResponse, AppError> {
    let quiz_id = quiz_id.into_inner();
    let quiz = Quiz::find_by_id(&pool, quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let quiz = Quiz::update(&pool, quiz_id, quiz_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(quiz))
}

#[delete("/{quiz_id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz_id = quiz_id.into_inner();
    let quiz = Quiz::find_by_id(&pool, quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Quiz::delete(&pool, quiz_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/{quiz_id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Created().json(quiz))
}
