use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuiz {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuiz {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: Uuid,
    pub text: String,
    pub order_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestion {
    pub text: Option<String>,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnswer {
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub order_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnswer {
    pub text: Option<String>,
    pub is_correct: Option<bool>,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizAttempt {
    pub quiz_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptAnswer {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAttemptAnswer {
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
}

impl User {
    pub async fn create(pool: &PgPool, user: CreateUser) -> Result<User, AppError> {
        let password_hash = auth::hash_password(&user.password).await?;
        
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, 'user', $4, $5)
            RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
            user.username,
            user.email,
            password_hash,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &PgPool, id: Uuid, user: UpdateUser) -> Result<User, AppError> {
        let current_user = Self::find_by_id(pool, id).await?;
        
        let password_hash = match user.password {
            Some(password) => Some(auth::hash_password(&password).await?),
            None => None,
        };

        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                password_hash = COALESCE($3, password_hash),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
            user.username,
            user.email,
            password_hash,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Quiz {
    pub async fn create(pool: &PgPool, creator_id: Uuid, quiz: CreateQuiz) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            INSERT INTO quizzes (title, description, creator_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, creator_id, created_at, updated_at
            "#,
            quiz.title,
            quiz.description,
            creator_id,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_creator(pool: &PgPool, creator_id: Uuid) -> Result<Vec<Quiz>, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            WHERE creator_id = $1
            "#,
            creator_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Quiz>, AppError> {
        let quizzes = sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(quizzes)
    }

    pub async fn update(pool: &PgPool, id: Uuid, quiz: UpdateQuiz) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            UPDATE quizzes
            SET 
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, title, description, creator_id, created_at, updated_at
            "#,
            quiz.title,
            quiz.description,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Question {
    pub async fn create(pool: &PgPool, question: CreateQuestion) -> Result<Question, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            INSERT INTO questions (quiz_id, text, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, quiz_id, text, order_num, created_at, updated_at
            "#,
            question.quiz_id,
            question.text,
            question.order_num,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_quiz(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Question>, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            SELECT id, quiz_id, text, order_num, created_at, updated_at
            FROM questions
            WHERE quiz_id = $1
            ORDER BY order_num
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Question, AppError> {
        let question = sqlx::query_as!(Question,
            r#"
            SELECT id, quiz_id, text, order_num, created_at, updated_at
            FROM questions
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(question)
    }

    pub async fn update(pool: &PgPool, id: Uuid, question: UpdateQuestion) -> Result<Question, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            UPDATE questions
            SET 
                text = COALESCE($1, text),
                order_num = COALESCE($2, order_num),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, quiz_id, text, order_num, created_at, updated_at
            "#,
            question.text,
            question.order_num,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM questions
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Answer {
    pub async fn create(pool: &PgPool, answer: CreateAnswer) -> Result<Answer, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            INSERT INTO answers (question_id, text, is_correct, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
            "#,
            answer.question_id,
            answer.text,
            answer.is_correct,
            answer.order_num,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_question(pool: &PgPool, question_id: Uuid) -> Result<Vec<Answer>, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE question_id = $1
            ORDER BY order_num
            "#,
            question_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Answer, AppError> {
        let answer = sqlx::query_as!(Answer,
            r#"
            SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(answer)
    }

    pub async fn update(pool: &PgPool, id: Uuid, answer: UpdateAnswer) -> Result<Answer, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            UPDATE answers
            SET 
                text = COALESCE($1, text),
                is_correct = COALESCE($2, is_correct),
                order_num = COALESCE($3, order_num),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
            "#,
            answer.text,
            answer.is_correct,
            answer.order_num,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM answers
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl QuizAttempt {
    pub async fn create(pool: &PgPool, user_id: Uuid, attempt: CreateQuizAttempt) -> Result<QuizAttempt, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            INSERT INTO quiz_attempts (quiz_id, user_id)
            VALUES ($1, $2)
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            attempt.quiz_id,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<QuizAttempt>, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT id, quiz_id, user_id, score, completed_at, created_at, updated_at
            FROM quiz_attempts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn complete(pool: &PgPool, id: Uuid, score: i32) -> Result<QuizAttempt, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            UPDATE quiz_attempts
            SET 
                score = $1,
                completed_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            score,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }
}

impl AttemptAnswer {
    pub async fn create(pool: &PgPool, answer: CreateAttemptAnswer) -> Result<AttemptAnswer, AppError> {
        // First, verify if the answer is correct
        let is_correct = sqlx::query_scalar!(
            r#"
            SELECT is_correct
            FROM answers
            WHERE id = $1 AND question_id = $2
            "#,
            answer.answer_id,
            answer.question_id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)?;

        sqlx::query_as!(
            AttemptAnswer,
            r#"
            INSERT INTO attempt_answers (attempt_id, question_id, answer_id, is_correct)
            VALUES ($1, $2, $3, $4)
            RETURNING id, attempt_id, question_id, answer_id, is_correct, created_at
            "#,
            answer.attempt_id,
            answer.question_id,
            answer.answer_id,
            is_correct
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_attempt(pool: &PgPool, attempt_id: Uuid) -> Result<Vec<AttemptAnswer>, AppError> {
        sqlx::query_as!(
            AttemptAnswer,
            r#"
            SELECT id, attempt_id, question_id, answer_id, is_correct, created_at
            FROM attempt_answers
            WHERE attempt_id = $1
            "#,
            attempt_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }
}
