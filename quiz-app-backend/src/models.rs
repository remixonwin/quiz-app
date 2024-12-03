use crate::{
    error::AppError,
};
use chrono::{Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use async_trait::async_trait;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

#[async_trait]
pub trait DbModel {
    type CreateForm;
    type UpdateForm;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> where Self: Sized;
    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> where Self: Sized;
    async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> where Self: Sized;
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid, 
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
    pub username: String,
    pub role: String,
}

#[async_trait]
impl DbModel for User {
    type CreateForm = CreateUser;
    type UpdateForm = UpdateUser;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let user = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO users (username, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            form.username,
            hash(&form.password, DEFAULT_COST).map_err(|_| AppError::HashError)?,
            form.role.unwrap_or_else(|| "user".to_string()),
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let user_id = form.id.unwrap_or_else(|| Uuid::new_v4());

        // Fetch the existing user to get current values
        let existing_user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

        let username = form.username.unwrap_or(existing_user.username);
        let role = form.role.unwrap_or(existing_user.role);

        // If password is provided, hash it
        let password_hash = match form.password {
            Some(password) => hash(&password, DEFAULT_COST)
                .map_err(|_| AppError::HashError)?,
            None => existing_user.password_hash,
        };

        let user = sqlx::query_as!(
            Self,
            r#"
            UPDATE users
            SET username = $1, password_hash = $2, role = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            username,
            password_hash,
            role,
            now,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn delete(pool: &PgPool, id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl User {
    pub async fn get_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e))
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuiz {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_trait]
impl DbModel for Quiz {
    type CreateForm = CreateQuiz;
    type UpdateForm = CreateQuiz;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let quiz = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, created_by, created_at, updated_at
            "#,
            form.title,
            form.description,
            form.created_by.unwrap_or_else(|| Uuid::new_v4()),
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz)
    }

    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let quiz = sqlx::query_as!(
            Self,
            r#"
            UPDATE quizzes
            SET title = $1, description = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, title, description, created_by, created_at, updated_at
            "#,
            form.title,
            form.description,
            now,
            form.id.unwrap_or_else(|| Uuid::new_v4())
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz)
    }

    async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM quizzes
            WHERE id = $1 AND created_by = $2
            "#,
            id,
            user_id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound("Quiz not found or unauthorized".into()))
        } else {
            Ok(())
        }
    }
}

impl Quiz {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        let quizzes = sqlx::query_as!(
            Self,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at
            FROM quizzes
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(quizzes)
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let quiz = sqlx::query_as!(
            Self,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at
            FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(quiz)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Question {
    pub id: Uuid, 
    pub quiz_id: Uuid, 
    pub question_text: String,
    pub order_num: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: Uuid, 
    pub question_text: String,
    pub order_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestionRequest {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_text: String,
    pub order_num: i32,
    pub created_by: Uuid,
    pub updated_at: NaiveDateTime,
}

#[async_trait]
impl DbModel for Question {
    type CreateForm = CreateQuestion;
    type UpdateForm = UpdateQuestionRequest;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let question = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO questions (quiz_id, question_text, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, quiz_id, question_text, order_num, created_at, updated_at
            "#,
            form.quiz_id,
            form.question_text,
            form.order_num,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let question = sqlx::query_as!(
            Self,
            r#"
            UPDATE questions
            SET question_text = $1, order_num = $2, updated_at = $3
            WHERE id = $4 AND quiz_id = $5
            RETURNING id, quiz_id, question_text, order_num, created_at, updated_at
            "#,
            form.question_text,
            form.order_num,
            now,
            form.id,
            form.quiz_id
        )
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let quiz = sqlx::query!(
            r#"
            SELECT created_by
            FROM quizzes
            WHERE id = (SELECT quiz_id FROM questions WHERE id = $1)
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        if quiz.created_by != user_id {
            return Err(AppError::Unauthorized("Not authorized to delete this question".into()));
        }

        let result = sqlx::query!("DELETE FROM questions WHERE id = $1", id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound("Question not found".into()))
        } else {
            Ok(())
        }
    }
}

impl Question {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let question = sqlx::query_as!(
            Self,
            r#"
            SELECT id, quiz_id, question_text, order_num, created_at, updated_at
            FROM questions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(question)
    }

    pub async fn get_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Self>, AppError> {
        let questions = sqlx::query_as!(
            Self,
            r#"
            SELECT id, quiz_id, question_text, order_num, created_at, updated_at
            FROM questions
            WHERE quiz_id = $1
            ORDER BY order_num
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub answer_text: String,
    pub is_correct: bool,
    pub order_num: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub question_id: Uuid,
    pub answer_text: String,
    pub is_correct: bool,
    pub order_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAnswerRequest {
    pub id: Uuid,
    pub text: Option<String>,
    pub is_correct: Option<bool>,
}

#[async_trait]
impl DbModel for Answer {
    type CreateForm = CreateAnswer;
    type UpdateForm = UpdateAnswerRequest;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let answer = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO answers (question_id, answer_text, is_correct, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, question_id, answer_text, is_correct, order_num, created_at, updated_at
            "#,
            form.question_id,
            form.answer_text,
            form.is_correct,
            form.order_num,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(answer)
    }

    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let answer = sqlx::query_as!(
            Self,
            r#"
            UPDATE answers
            SET answer_text = $1, is_correct = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, question_id, answer_text, is_correct, order_num, created_at, updated_at
            "#,
            form.text.unwrap_or_else(|| "".to_string()),
            form.is_correct.unwrap_or(false),
            now,
            form.id
        )
        .fetch_one(pool)
        .await?;

        Ok(answer)
    }

    async fn delete(pool: &PgPool, id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM answers WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl Answer {
    pub async fn get_by_question_id(pool: &PgPool, question_id: Uuid) -> Result<Vec<Self>, AppError> {
        let answers = sqlx::query_as!(
            Self,
            r#"
            SELECT id, question_id, answer_text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE question_id = $1
            ORDER BY order_num
            "#,
            question_id
        )
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }

    pub async fn get_by_id(pool: &PgPool, answer_id: Uuid) -> Result<Option<Self>, AppError> {
        let answer = sqlx::query_as!(
            Answer,
            r#"
            SELECT id, question_id, answer_text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE id = $1
            "#,
            answer_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(answer)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub score: Option<i32>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnswer {
    pub question_id: Uuid,
    pub selected_answer_id: Uuid,
}

#[async_trait]
impl DbModel for QuizAttempt {
    type CreateForm = Self;
    type UpdateForm = Self;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let quiz_attempt = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO quiz_attempts (id, quiz_id, user_id, score, completed_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            form.id,
            form.quiz_id,
            form.user_id,
            form.score,
            form.completed_at,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz_attempt)
    }

    async fn update(pool: &PgPool, form: Self::UpdateForm) -> Result<Self, AppError> {
        let now = Utc::now().naive_utc();
        let quiz_attempt = sqlx::query_as!(
            Self,
            r#"
            UPDATE quiz_attempts
            SET score = $1, completed_at = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            form.score,
            form.completed_at,
            now,
            form.id
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz_attempt)
    }

    async fn delete(pool: &PgPool, id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM quiz_attempts WHERE id = $1", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl QuizAttempt {
    /// Finds all quiz attempts for a specific quiz.
    /// This method is currently unused but may be used in future analytics or reporting features.
    #[allow(dead_code)]
    pub async fn find_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<QuizAttempt>, AppError> {
        let quiz_attempts = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT id, quiz_id, user_id, score, completed_at, created_at, updated_at
            FROM quiz_attempts
            WHERE quiz_id = $1
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await?;

        Ok(quiz_attempts)
    }
}
