use actix_web::{
    delete, get, post, put,
    web,
    HttpResponse,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::jwt::Claims;
use crate::{
    error::AppError,
    models::{
        answer::{Answer, CreateAnswer, UpdateAnswer},
        question::Question,
        quiz::Quiz,
        DbModel,
    },
};

#[get("/quiz/{quiz_id}/answers")]
pub async fn get_quiz_answers(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = match Quiz::get_by_id(&pool, quiz_id.into_inner()).await? {
        Some(quiz) => quiz,
        None => return Err(AppError::NotFound("Quiz not found".into())),
    };

    let answers = Answer::get_by_quiz_id(&pool, quiz.id).await?;
    Ok(HttpResponse::Ok().json(answers))
}

#[post("/quiz/{quiz_id}/answer")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<CreateAnswer>,
) -> Result<HttpResponse, AppError> {
    let quiz = match Quiz::get_by_id(&pool, quiz_id.into_inner()).await? {
        Some(quiz) => quiz,
        None => return Err(AppError::NotFound("Quiz not found".into())),
    };

    if quiz.created_by != claims.sub {
        return Err(AppError::Unauthorized("Not authorized to create answers for this quiz".into()));
    }

    let answer = Answer::create(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[put("/quiz/{quiz_id}/answer/{answer_id}")]
pub async fn update_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
    form: web::Json<UpdateAnswer>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let form = form.into_inner();
    let answer_id = answer_id.into_inner();
    let updated_answer = Answer::update(&pool, answer_id, form).await?;
    Ok(HttpResponse::Ok().json(updated_answer))
}

#[get("/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(answers))
}

#[delete("/quiz/{quiz_id}/answer/{answer_id}")]
pub async fn delete_answer(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    answer_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = match Quiz::get_by_id(&pool, quiz_id.into_inner()).await? {
        Some(quiz) => quiz,
        None => return Err(AppError::NotFound("Quiz not found".into())),
    };

    if quiz.created_by != claims.sub {
        return Err(AppError::Unauthorized("Not authorized to delete this answer".into()));
    }

    let answer_id = answer_id.into_inner();
    let answer = match Answer::get_by_id(&pool, answer_id).await? {
        Some(answer) => answer,
        None => return Err(AppError::NotFound("Answer not found".into())),
    };

    let question = match Question::get_by_id(&pool, answer.question_id).await? {
        Some(question) => question,
        None => return Err(AppError::NotFound("Question not found".into())),
    };

    if question.quiz_id != quiz.id {
        return Err(AppError::BadRequest("Answer does not belong to this quiz".into()));
    }

    Answer::delete(&pool, answer_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
