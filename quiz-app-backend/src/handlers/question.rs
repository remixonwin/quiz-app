use crate::{
    auth::Claims,
    error::AppError,
    models::{CreateQuestion, DbModel, Question, Answer, CreateAnswer},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestionRequest {
    pub question_text: String,
    pub order_num: Option<i32>,
}

#[post("")]
pub async fn create_question(
    pool: web::Data<PgPool>,
    question_data: web::Json<CreateQuestion>,
    _claims: Claims,
) -> Result<HttpResponse> {
    let question = Question::create(&pool, question_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(question))
}

#[get("/{question_id}")]
pub async fn get_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>, // Changed from i32 to Uuid
) -> Result<HttpResponse> {
    let question = Question::find_by_id(&pool, question_id.into_inner()).await?;
    match question {
        Some(question) => Ok(HttpResponse::Ok().json(question)),
        None => Ok(HttpResponse::NotFound().finish())
    }
}

#[get("")]
pub async fn list_questions(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>, // Changed from i32 to Uuid
) -> Result<HttpResponse> {
    let questions = Question::find_by_quiz_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[put("/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>, // Changed from (i32, i32) to (Uuid, Uuid)
    question_data: web::Json<UpdateQuestionRequest>,
    claims: Claims,
) -> Result<HttpResponse> {
    let (quiz_id, question_id) = path.into_inner();
    
    // Check if the user is the creator of the quiz
    let quiz = sqlx::query!(
        r#"
        SELECT created_by
        FROM quizzes
        WHERE id = $1
        "#,
        quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to update this quiz".into()));
    }

    let question = sqlx::query_as!(
        Question,
        r#"
        UPDATE questions
        SET question_text = $1,
            order_num = $2,
            updated_at = $3
        WHERE id = $4 AND quiz_id = $5
        RETURNING id, quiz_id, question_text, order_num, created_at, updated_at
        "#,
        question_data.question_text,
        question_data.order_num.unwrap_or(0),
        NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        question_id,
        quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(question))
}

#[delete("/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>, // Changed from (i32, i32) to (Uuid, Uuid)
    claims: Claims,
) -> Result<HttpResponse> {
    let (quiz_id, question_id) = path.into_inner();

    // Check if the user is the creator of the quiz
    let quiz = sqlx::query!(
        r#"
        SELECT creator_id
        FROM quizzes
        WHERE id = $1
        "#,
        quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    if quiz.creator_id != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to delete questions from this quiz".into()));
    }

    let result = sqlx::query!(
        r#"
        DELETE FROM questions
        WHERE id = $1 AND quiz_id = $2
        "#,
        question_id,
        quiz_id
    )
    .execute(&**pool)
    .await?;

    if result.rows_affected() == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

#[post("/{question_id}/answers")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>, // Changed from (i32, i32) to (Uuid, Uuid)
    answer_data: web::Json<CreateAnswer>,
    claims: Claims,
) -> Result<HttpResponse> {
    let (quiz_id, question_id) = path.into_inner();

    let mut create_answer = answer_data.into_inner();
    create_answer.question_id = question_id; // Assigning question_id from path

    let answer = Answer::create(&pool, create_answer).await?;

    Ok(HttpResponse::Created().json(answer))
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>, // Changed from (i32, i32) to (Uuid, Uuid)
) -> Result<HttpResponse> {
    let (_quiz_id, question_id) = path.into_inner();

    let answers = sqlx::query_as!(
        Answer,
        r#"
        SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
        FROM answers
        WHERE question_id = $1
        ORDER BY order_num, id
        "#,
        question_id
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(answers))
}
