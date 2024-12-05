use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Question, Quiz, Answer, CreateQuestion, UpdateQuestion},
};

#[post("")]
pub async fn create_question(
    pool: web::Data<PgPool>,
    claims: Claims,
    form: web::Json<CreateQuestion>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, form.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let question = Question::create(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(question))
}

#[get("/{quiz_id}/questions")]
pub async fn get_questions(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let questions = Question::find_by_quiz(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[get("/{question_id}")]
pub async fn get_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let question = Question::find_by_id(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[put("/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<UpdateQuestion>,
) -> Result<HttpResponse, AppError> {
    let question_id = question_id.into_inner();
    let question = Question::find_by_id(&pool, question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let question = Question::update(&pool, question_id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[delete("/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let question_id = question_id.into_inner();
    let question = Question::find_by_id(&pool, question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Question::delete(&pool, question_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::get_by_question(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answers))
}
