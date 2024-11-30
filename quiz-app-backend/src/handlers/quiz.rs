use actix_web::{
    get, post, put, delete,
    web, HttpResponse, Responder
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Error as SqlxError};

use crate::{
    auth::Claims,
    error::AppError,
    models::{Quiz, CreateQuiz, QuizAttemptResponse, UserAnswer},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: i32,
    pub selected_answer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitQuizRequest {
    pub answers: Vec<QuestionAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct QuizPath {
    quiz_id: i32,
}

#[post("/quizzes")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    quiz: web::Json<CreateQuiz>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, description, created_by, created_at, updated_at
        "#,
        quiz.title,
        quiz.description,
        claims.user_id,
        Utc::now(),
        None::<chrono::DateTime<Utc>>,
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(quiz))
}

#[get("/quizzes")]
pub async fn get_quizzes(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let quizzes = sqlx::query_as!(
        Quiz,
        r#"
        SELECT id, title, description, created_by, created_at, updated_at
        FROM quizzes
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(quizzes))
}

#[get("/quizzes/{quiz_id}")]
pub async fn get_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = quiz_id.into_inner();
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        SELECT id, title, description, created_by, created_at, updated_at
        FROM quizzes
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match quiz {
        Some(quiz) => Ok(HttpResponse::Ok().json(quiz)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/quizzes/{id}")]
pub async fn update_quiz(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    quiz: web::Json<CreateQuiz>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();
    
    // Check if quiz exists and user owns it
    let existing_quiz = sqlx::query_as!(
        Quiz,
        r#"
        SELECT id, title, description, created_by, created_at, updated_at
        FROM quizzes
        WHERE id = $1 AND created_by = $2
        "#,
        id,
        claims.user_id
    )
    .fetch_one(&**pool)
    .await
    .map_err(|_| AppError::NotFound("Quiz not found or unauthorized".to_string()))?;

    // Update the quiz
    let updated_quiz = sqlx::query_as!(
        Quiz,
        r#"
        UPDATE quizzes
        SET title = $1, description = $2, updated_at = $3
        WHERE id = $4
        RETURNING id, title, description, created_by, created_at, updated_at
        "#,
        quiz.title,
        quiz.description,
        Utc::now(),
        id
    )
    .fetch_one(&**pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(HttpResponse::Ok().json(updated_quiz))
}

#[delete("/quizzes/{quiz_id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let id = quiz_id.into_inner();
    
    // Check if quiz exists and user owns it
    let quiz = sqlx::query!(
        r#"
        SELECT created_by FROM quizzes WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await?;

    let quiz = quiz.ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to delete this quiz".into()));
    }

    // Delete the quiz
    sqlx::query!(
        r#"
        DELETE FROM quizzes WHERE id = $1
        "#,
        id
    )
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/quizzes/{quiz_id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
    user_answers: web::Json<Vec<UserAnswer>>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let id = quiz_id.into_inner();
    let now = Utc::now();
    let quiz_attempt = sqlx::query_as!(
        QuizAttemptResponse,
        r#"
        INSERT INTO quiz_attempts (quiz_id, user_id, completed_at, created_at, updated_at)
        VALUES ($1, $2, $3, $3, $3)
        RETURNING 
            id,
            quiz_id,
            user_id,
            completed_at as "completed_at!: DateTime<Utc>",
            created_at as "created_at!: DateTime<Utc>",
            updated_at as "updated_at!: DateTime<Utc>"
        "#,
        id,
        claims.user_id,
        now
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(quiz_attempt))
}
