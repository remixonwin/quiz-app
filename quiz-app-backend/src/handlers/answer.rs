use actix_web::{web, HttpResponse, get, post, put, delete};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::Claims;
use crate::error::AppError;
use crate::models::{Answer, CreateAnswer, UpdateAnswerRequest, DbModel};

#[get("")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    #[allow(unused_variables)]
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::get_by_question_id(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answers))
}

#[post("")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    form: web::Json<CreateAnswer>,
    #[allow(unused_variables)]
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let mut answer_data = form.into_inner();
    answer_data.question_id = question_id.into_inner();
    let answer = Answer::create(&pool, answer_data).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[get("/{answer_id}")]
pub async fn get_answer(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    #[allow(unused_variables)]
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_question_id, answer_id) = path.into_inner();
    let answer = Answer::get_by_id(&pool, answer_id).await?;
    match answer {
        Some(answer) => Ok(HttpResponse::Ok().json(answer)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[put("/{answer_id}")]
pub async fn update_answer(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    form: web::Json<UpdateAnswerRequest>,
    #[allow(unused_variables)]
    _claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_question_id, answer_id) = path.into_inner();
    let mut answer_data = form.into_inner();
    answer_data.id = answer_id;
    let answer = Answer::update(&pool, answer_data).await?;
    Ok(HttpResponse::Ok().json(answer))
}

#[delete("/{answer_id}")]
pub async fn delete_answer(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_question_id, answer_id) = path.into_inner();
    Answer::delete(&pool, answer_id, claims.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
