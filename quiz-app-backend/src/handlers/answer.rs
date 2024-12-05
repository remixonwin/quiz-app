use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Answer, Question, Quiz, CreateAnswer, UpdateAnswer},
};

#[post("")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    claims: Claims,
    form: web::Json<CreateAnswer>,
) -> Result<HttpResponse, AppError> {
    let question = Question::find_by_id(&pool, form.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let answer = Answer::create(&pool, question.id, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::find_by_question(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answers))
}

#[get("/{answer_id}")]
pub async fn get_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answer = Answer::find_by_id(&pool, answer_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answer))
}

#[put("/{answer_id}")]
pub async fn update_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<UpdateAnswer>,
) -> Result<HttpResponse, AppError> {
    let answer_id = answer_id.into_inner();
    let answer = Answer::find_by_id(&pool, answer_id).await?;
    let question = Question::find_by_id(&pool, answer.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let answer = Answer::update(&pool, answer_id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answer))
}

#[delete("/{answer_id}")]
pub async fn delete_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let answer_id = answer_id.into_inner();
    let answer = Answer::find_by_id(&pool, answer_id).await?;
    let question = Question::find_by_id(&pool, answer.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Answer::delete(&pool, answer_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
