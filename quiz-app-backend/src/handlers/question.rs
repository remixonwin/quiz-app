use actix_web::{
    get, post, put, delete,
    web::{self, Json},
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::jwt::Claims,
    error::AppError,
    models::{
        DbModel,
        question::{Question, CreateQuestion, UpdateQuestion},
        answer::{Answer, CreateAnswer},
        quiz::Quiz,
    },
};

#[get("/quiz/{quiz_id}/questions")]
pub async fn get_questions(
    pool: web::Data<PgPool>,
    _quiz_id: web::Path<Uuid>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let questions = Question::get_by_quiz_id(&pool, _quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[post("/quiz/{quiz_id}/question")]
pub async fn create_question(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    question_data: Json<CreateQuestion>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Forbidden("You can only add questions to your own quizzes".into()));
    }

    let question = Question::create(&pool, question_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(question))
}

#[get("/quiz/{quiz_id}/question/{question_id}")]
pub async fn get_question(
    pool: web::Data<PgPool>,
    _quiz_id: web::Path<Uuid>,
    question_id: web::Path<Uuid>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let question = Question::get_by_id(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[put("/quiz/{quiz_id}/question/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    form: web::Json<UpdateQuestion>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let form = form.into_inner();
    let question_id = question_id.into_inner();
    let question = Question::update(&pool, question_id, form).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[delete("/quiz/{quiz_id}/question/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    _quiz_id: web::Path<Uuid>,
    question_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::get_by_id(&pool, _quiz_id.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Forbidden("You can only delete questions from your own quizzes".into()));
    }

    Question::delete(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/quiz/{quiz_id}/answer")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<CreateAnswer>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner())
        .await?
        .ok_or_else(|| AppError::NotFound("Quiz not found".into()))?;

    if quiz.created_by != claims.sub {
        return Err(AppError::Unauthorized("Not authorized to create answers for this quiz".into()));
    }

    let answer = Answer::create(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[get("/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(answers))
}
