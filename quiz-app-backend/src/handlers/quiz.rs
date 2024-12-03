use actix_web::{web, HttpResponse, get, post, put, delete};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::auth::Claims;
use crate::error::AppError;
use crate::models::{Quiz, QuizAttempt, UserAnswer, CreateQuiz, DbModel};

#[get("")]
pub async fn get_quizzes(
    pool: web::Data<PgPool>,
    #[allow(unused_variables)]
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quizzes = Quiz::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(quizzes))
}

#[post("")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    form: web::Json<CreateQuiz>,
    #[allow(unused_variables)]
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let mut quiz_data = form.into_inner();
    quiz_data.created_by = Some(claims.user_id);
    quiz_data.id = None;
    quiz_data.created_at = Utc::now().naive_utc();
    quiz_data.updated_at = Utc::now().naive_utc();
    let quiz = Quiz::create(&pool, quiz_data).await?;
    Ok(HttpResponse::Created().json(quiz))
}

#[get("/{quiz_id}")]
pub async fn get_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    #[allow(unused_variables)]
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner()).await?;
    match quiz {
        Some(quiz) => Ok(HttpResponse::Ok().json(quiz)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/{quiz_id}")]
pub async fn update_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    form: web::Json<CreateQuiz>,
    #[allow(unused_variables)]
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let mut quiz_data = form.into_inner();
    quiz_data.id = Some(quiz_id.into_inner());
    quiz_data.updated_at = Utc::now().naive_utc();
    let quiz = Quiz::update(&pool, quiz_data).await?;
    Ok(HttpResponse::Ok().json(quiz))
}

#[delete("/{quiz_id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    Quiz::delete(&pool, quiz_id.into_inner(), claims.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/{quiz_id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    _answers: web::Json<Vec<UserAnswer>>,
    #[allow(unused_variables)]
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let now = Utc::now().naive_utc();
    let quiz_attempt = QuizAttempt::create(&pool, QuizAttempt {
        id: Uuid::new_v4(),
        quiz_id: quiz_id.into_inner(),
        user_id: claims.user_id,
        score: None,
        completed_at: Some(now),
        created_at: now,
        updated_at: now,
    }).await?;

    Ok(HttpResponse::Created().json(quiz_attempt))
}
