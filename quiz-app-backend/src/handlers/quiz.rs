use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::jwt::Claims,
    error::AppError,
    models::{
        quiz::{CreateQuiz, Quiz, UpdateQuiz},
        question::Question,
        DbModel,
    },
};

#[post("/quizzes")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    form: web::Json<CreateQuiz>,
    claims: Claims,
) -> Result<impl Responder, AppError> {
    println!("Creating quiz with data: {:?}", form);
    println!("Claims: {:?}", claims);

    let mut form = form.into_inner();
    form.created_by = claims.sub;
    println!("Modified form: {:?}", form);

    let quiz = Quiz::create(&pool, form).await?;
    println!("Created quiz: {:?}", quiz);

    Ok(HttpResponse::Created().json(quiz))
}

#[get("/quizzes/{id}")]
pub async fn get_quiz(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let quiz = Quiz::get_by_id(&pool, id.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    Ok(HttpResponse::Ok().json(quiz))
}

#[put("/quizzes/{id}")]
pub async fn update_quiz(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    form: web::Json<UpdateQuiz>,
    claims: Claims,
) -> Result<impl Responder, AppError> {
    let quiz_id = id.into_inner();
    let quiz = Quiz::get_by_id(&pool, quiz_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Forbidden("You can only update your own quizzes".into()));
    }

    let updated_quiz = Quiz::update(&pool, quiz_id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_quiz))
}

#[derive(Debug, Deserialize)]
pub struct QuizFilters {
    pub user_id: Option<Uuid>,
    pub completed: Option<bool>,
}

#[get("/quizzes")]
pub async fn list_quizzes(
    pool: web::Data<PgPool>,
    filters: web::Query<QuizFilters>,
) -> Result<impl Responder, AppError> {
    let quizzes = sqlx::query_as!(
        Quiz,
        r#"
        SELECT *
        FROM quizzes
        WHERE ($1::uuid IS NULL OR created_by = $1)
        AND ($2::bool IS NULL OR (completed_at IS NOT NULL) = $2)
        "#,
        filters.user_id,
        filters.completed,
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(quizzes))
}

#[get("/quizzes")]
pub async fn get_quizzes(
    pool: web::Data<PgPool>,
    _claims: Claims,
) -> Result<impl Responder, AppError> {
    let quizzes = Quiz::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(quizzes))
}

#[delete("/quizzes/{id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    claims: Claims,
) -> Result<impl Responder, AppError> {
    println!("Attempting to delete quiz with id: {}", id);
    println!("User ID from claims: {}", claims.sub);

    let quiz_id = id.into_inner();
    let quiz = Quiz::get_by_id(&pool, quiz_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Forbidden("You can only delete your own quizzes".into()));
    }

    Quiz::delete(&pool, quiz_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/{id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz_id = id.into_inner();
    let quiz = Quiz::get_by_id(&pool, quiz_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Forbidden("You can only submit your own quizzes".into()));
    }

    // Get all questions for this quiz
    let _questions = Question::get_by_quiz_id(&pool, quiz.id).await?;

    Ok(HttpResponse::Ok().json(quiz))
}
