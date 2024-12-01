use actix_web::{
    get, post, put, delete,
    web, HttpResponse
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Quiz, CreateQuiz, QuizAttemptResponse, UserAnswer},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizRequest {
    pub title: String,
    pub description: Option<String>,
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
    #[allow(dead_code)]
    quiz_id: i32,
}

#[post("")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    quiz_req: web::Json<CreateQuizRequest>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    println!("=== Starting quiz creation ===");
    println!("Request data - Title: {}, Description: {:?}", quiz_req.title, quiz_req.description);
    println!("User ID from claims: {}", claims.user_id);

    let quiz = CreateQuiz {
        title: quiz_req.title.clone(),
        description: quiz_req.description.clone(),
        created_by: claims.user_id,
    };
    println!("Created quiz struct: {:?}", quiz);

    match Quiz::create(&pool, quiz).await {
        Ok(created_quiz) => {
            println!("Quiz created successfully: {:?}", created_quiz);
            Ok(HttpResponse::Created().json(created_quiz))
        }
        Err(err) => {
            println!("Error creating quiz: {:?}", err);
            println!("Error details: {}", err);
            Err(err)
        }
    }
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

#[put("/{id}")]
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
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&**pool)
    .await
    .map_err(|_| AppError::NotFound("Quiz not found".to_string()))?;

    // Verify ownership
    if existing_quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("You don't have permission to update this quiz".to_string()));
    }

    // Update the quiz
    let updated_quiz = sqlx::query_as!(
        Quiz,
        r#"
        UPDATE quizzes
        SET title = $1, description = $2, updated_at = $3
        WHERE id = $4 AND created_by = $5
        RETURNING id, title, description, created_by, created_at, updated_at
        "#,
        quiz.title,
        quiz.description,
        Utc::now(),
        id,
        claims.user_id
    )
    .fetch_one(&**pool)
    .await
    .map_err(|e| {
        println!("Error updating quiz: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok(HttpResponse::Ok().json(updated_quiz))
}

#[delete("/{quiz_id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let id = quiz_id.into_inner();
    
    println!("Attempting to delete quiz with ID: {} by user: {}", id, claims.user_id);
    
    // Check if quiz exists and user owns it
    let quiz = sqlx::query!(
        r#"
        SELECT created_by FROM quizzes WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool.get_ref())
    .await?;

    if let Some(quiz) = &quiz {
        println!("Quiz found with ID: {}. Owned by user: {}", id, quiz.created_by);
    } else {
        println!("No quiz found with ID: {}", id);
    }

    let quiz = quiz.ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    // Change from Unauthorized to Forbidden when user doesn't own the quiz
    if quiz.created_by != claims.user_id {
        println!("User: {} is not authorized to delete quiz with ID: {}", claims.user_id, id);
        return Err(AppError::Forbidden("Not authorized to delete this quiz".into()));
    }

    // Delete the quiz
    let result = sqlx::query!(
        r#"
        DELETE FROM quizzes WHERE id = $1 AND created_by = $2
        "#,
        id,
        claims.user_id
    )
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        println!("No rows affected when trying to delete quiz with ID: {} by user: {}", id, claims.user_id);
        return Err(AppError::NotFound("Quiz not found or not authorized".into()));
    }

    println!("Quiz with ID: {} deleted successfully by user: {}", id, claims.user_id);

    Ok(HttpResponse::NoContent().finish())
}

#[post("/quizzes/{quiz_id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
    _user_answers: web::Json<Vec<UserAnswer>>,
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
