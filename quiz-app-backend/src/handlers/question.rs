use actix_web::{web, HttpResponse, get, post, put, delete};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::Claims;
use crate::error::AppError;
use crate::models::{Question, Answer, CreateQuestion, CreateAnswer, UpdateQuestionRequest, DbModel, Quiz};

#[get("")]
pub async fn get_questions(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // Check if user has access to the quiz
    let quiz_uuid = *quiz_id;
    let quiz = Quiz::get_by_id(&pool, quiz_uuid).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    // Implement basic authorization check
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to view these questions".to_string()));
    }

    let questions = Question::get_by_quiz_id(&pool, quiz_uuid).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[post("")]
pub async fn create_question(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    form: web::Json<CreateQuestion>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // Check if user owns the quiz
    let quiz_uuid = *quiz_id;
    let quiz = Quiz::get_by_id(&pool, quiz_uuid).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to create questions for this quiz".to_string()));
    }

    let mut question_data = form.into_inner();
    question_data.quiz_id = quiz_uuid;
    let question = Question::create(&pool, question_data).await?;
    Ok(HttpResponse::Created().json(question))
}

#[get("/{question_id}")]
pub async fn get_question(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_quiz_id, question_id) = path.into_inner();
    let question = Question::get_by_id(&pool, question_id).await?
        .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Check if user owns the quiz containing this question
    let quiz = Quiz::get_by_id(&pool, question.quiz_id).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to view this question".to_string()));
    }

    Ok(HttpResponse::Ok().json(question))
}

#[put("/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    form: web::Json<UpdateQuestionRequest>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_quiz_id, question_id) = path.into_inner();
    let question = Question::get_by_id(&pool, question_id).await?
        .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Check if user owns the quiz containing this question
    let quiz = Quiz::get_by_id(&pool, question.quiz_id).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to update this question".to_string()));
    }

    let mut question_data = form.into_inner();
    question_data.id = question_id;
    let question = Question::update(&pool, question_data).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[delete("/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (_quiz_id, question_id) = path.into_inner();
    let question = Question::get_by_id(&pool, question_id).await?
        .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Check if user owns the quiz containing this question
    let quiz = Quiz::get_by_id(&pool, question.quiz_id).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to delete this question".to_string()));
    }

    Question::delete(&pool, question_id, claims.user_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/answer")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    form: web::Json<CreateAnswer>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (question_id, _answer_id) = path.into_inner();
    let question = Question::get_by_id(&pool, question_id).await?
        .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Check if user owns the quiz containing this question
    let quiz = Quiz::get_by_id(&pool, question.quiz_id).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to create answers for this question".to_string()));
    }

    let mut answer_data = form.into_inner();
    answer_data.question_id = question_id;
    let answer = Answer::create(&pool, answer_data).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[get("/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let (question_id, _) = path.into_inner();
    let question = Question::get_by_id(&pool, question_id).await?
        .ok_or(AppError::NotFound("Question not found".to_string()))?;

    // Check if user owns the quiz containing this question
    let quiz = Quiz::get_by_id(&pool, question.quiz_id).await?
        .ok_or(AppError::NotFound("Quiz not found".to_string()))?;
    
    if quiz.created_by != claims.user_id {
        return Err(AppError::Unauthorized("Not authorized to view answers for this question".to_string()));
    }

    let answers = Answer::get_by_question_id(&pool, question_id).await?;
    Ok(HttpResponse::Ok().json(answers))
}
