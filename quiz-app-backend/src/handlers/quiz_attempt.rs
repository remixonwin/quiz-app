use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    auth::Claims,
    error::AppError,
    models::{QuizAttempt, CreateQuizAttempt, SubmitAnswer, Quiz},
};

pub async fn start_quiz_attempt(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // Check if quiz exists
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner()).await?;
    
    // Create a new quiz attempt
    let quiz_attempt = QuizAttempt::create(&pool, CreateQuizAttempt {
        quiz_id: quiz.id,
        user_id: claims.user_id,
        started_at: Utc::now(),
    }).await?;

    Ok(HttpResponse::Created().json(quiz_attempt))
}

pub async fn get_quiz_attempt(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    Ok(HttpResponse::Ok().json(attempt))
}

pub async fn submit_answer(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
    answer_data: web::Json<SubmitAnswer>,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    // Submit the answer
    let submitted_answer = QuizAttempt::submit_answer(
        &pool,
        attempt.id,
        answer_data.into_inner()
    ).await?;

    Ok(HttpResponse::Ok().json(submitted_answer))
}

pub async fn finish_quiz_attempt(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    // Finish the attempt and calculate score
    let completed_attempt = QuizAttempt::finish(
        &pool,
        attempt.id,
        Utc::now(),
    ).await?;

    Ok(HttpResponse::Ok().json(completed_attempt))
}

pub async fn get_user_quiz_attempts(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempts = QuizAttempt::get_by_user_id(&pool, claims.user_id).await?;
    Ok(HttpResponse::Ok().json(attempts))
}

pub async fn get_quiz_attempts_by_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // First check if user owns the quiz
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner()).await?;
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let attempts = QuizAttempt::get_by_quiz_id(&pool, quiz.id).await?;
    Ok(HttpResponse::Ok().json(attempts))
}
