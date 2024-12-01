use crate::{
    auth::Claims,
    error::AppError,
    models::{CreateQuestion, DbModel, Question, Answer, CreateAnswer},
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

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
    question_id: web::Path<i32>,
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
    quiz_id: web::Path<i32>,
) -> Result<HttpResponse> {
    let questions = Question::find_by_quiz_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[put("/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    question_data: web::Json<UpdateQuestionRequest>,
    claims: Claims,
) -> Result<HttpResponse> {
    let (_quiz_id, question_id) = path.into_inner();
    
    // Check if the user is the creator of the quiz
    let quiz = sqlx::query!(
        r#"
        SELECT created_by
        FROM quizzes
        WHERE id = $1
        "#,
        _quiz_id
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
            order_num = COALESCE($2, order_num),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $3 AND quiz_id = $4
        RETURNING id, quiz_id, question_text, order_num, created_at, updated_at
        "#,
        question_data.question_text,
        question_data.order_num,
        question_id,
        _quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(question))
}

#[delete("/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    claims: Claims,
) -> Result<HttpResponse> {
    let (_quiz_id, question_id) = path.into_inner();

    // Check if the user is the creator of the quiz
    let quiz = sqlx::query!(
        r#"
        SELECT created_by
        FROM quizzes
        WHERE id = $1
        "#,
        _quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to delete questions from this quiz".into()));
    }

    let result = sqlx::query!(
        r#"
        DELETE FROM questions
        WHERE id = $1 AND quiz_id = $2
        "#,
        question_id,
        _quiz_id
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
    path: web::Path<(i32, i32)>,
    answer_data: web::Json<CreateAnswer>,
    claims: Claims,
) -> Result<HttpResponse> {
    let (_quiz_id, question_id) = path.into_inner();

    // Check if the user is the creator of the quiz
    let quiz = sqlx::query!(
        r#"
        SELECT created_by
        FROM quizzes
        WHERE id = $1
        "#,
        _quiz_id
    )
    .fetch_one(&**pool)
    .await?;

    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to add answers to this quiz".into()));
    }

    // Get the maximum order number for this question
    let max_order = sqlx::query!(
        r#"
        SELECT COALESCE(MAX(order_num), -1) as max_order
        FROM answers
        WHERE question_id = $1
        "#,
        question_id
    )
    .fetch_one(&**pool)
    .await?;

    let order_num = max_order.max_order.unwrap_or(-1) + 1;

    let answer = sqlx::query_as!(
        Answer,
        r#"
        INSERT INTO answers (question_id, text, is_correct, order_num, created_at, updated_at)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
        "#,
        question_id,
        answer_data.text,
        answer_data.is_correct,
        order_num
    )
    .fetch_one(&**pool)
    .await?;

    Ok(HttpResponse::Created().json(answer))
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse> {
    let (_quiz_id, question_id) = path.into_inner();

    let answers = sqlx::query_as!(
        Answer,
        r#"
        SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
        FROM answers
        WHERE question_id = $1
        ORDER BY order_num
        "#,
        question_id
    )
    .fetch_all(&**pool)
    .await?;

    Ok(HttpResponse::Ok().json(answers))
}
