use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use async_trait::async_trait;
use crate::error::AppError;

#[async_trait]
pub trait DbModel {
    type CreateForm;
    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> where Self: Sized;
    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, AppError> where Self: Sized;
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[async_trait]
impl DbModel for User {
    type CreateForm = CreateUser;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO users (username, password_hash, role)
            VALUES ($1, $2, 'user')
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            form.username,
            form.password
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, username, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQuiz {
    pub title: String,
    pub description: Option<String>,
    pub created_by: i32,
}

#[async_trait]
impl DbModel for Quiz {
    type CreateForm = CreateQuiz;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, created_by, created_at, updated_at
            "#,
            form.title,
            form.description,
            form.created_by,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at
            FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}

impl Quiz {
    pub async fn create(pool: &PgPool, form: CreateQuiz) -> Result<Self, AppError> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id, title, description, created_by, created_at, updated_at
            "#,
            form.title,
            form.description,
            form.created_by,
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            println!("Error creating quiz: {:?}", e);
            AppError::DatabaseError(e)
        })?;

        Ok(quiz)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub quiz_id: i32,
    pub question_text: String,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: i32,
    pub question_text: String,
    pub order_num: Option<i32>,
}

#[async_trait]
impl DbModel for Question {
    type CreateForm = CreateQuestion;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO questions (quiz_id, question_text, order_num)
            VALUES ($1, $2, $3)
            RETURNING id, quiz_id, question_text, order_num, created_at, updated_at
            "#,
            form.quiz_id,
            form.question_text,
            form.order_num.unwrap_or(0) // default order_num
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, quiz_id, question_text, order_num, created_at, updated_at
            FROM questions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}

impl Question {
    pub async fn find_by_quiz_id(pool: &PgPool, quiz_id: i32) -> Result<Vec<Question>, sqlx::Error> {
        sqlx::query_as!(
            Question,
            r#"
            SELECT id, quiz_id, question_text, order_num, created_at, updated_at
            FROM questions
            WHERE quiz_id = $1
            ORDER BY created_at ASC
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub text: String,
    pub is_correct: bool,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub text: String,
    pub is_correct: bool,
}

#[async_trait]
impl DbModel for Answer {
    type CreateForm = CreateAnswer;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO answers (question_id, text, is_correct, order_num)
            VALUES ($1, $2, $3, $4)
            RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
            "#,
            0, // default question_id
            form.text,
            form.is_correct,
            0 // default order_num
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizSubmission {
    pub answers: Vec<SubmittedAnswer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmittedAnswer {
    pub question_id: i32,
    pub selected_answer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnswer {
    pub question_id: i32,
    pub selected_option: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizAttemptResponse {
    pub id: i32,
    pub quiz_id: i32,
    pub user_id: i32,
    pub completed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}
