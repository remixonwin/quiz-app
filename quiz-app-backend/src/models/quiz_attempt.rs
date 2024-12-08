use crate::{
    error::AppError,
    models::DbModel,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, types::BigDecimal};
use uuid::Uuid;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub score: Option<BigDecimal>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizAttempt {
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub score: Option<BigDecimal>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuizAttempt {
    pub id: Uuid,
    pub score: Option<BigDecimal>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[async_trait]
impl DbModel for QuizAttempt {
    type CreateForm = CreateQuizAttempt;
    type UpdateForm = UpdateQuizAttempt;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let attempt = sqlx::query_as!(
            QuizAttempt,
            r#"
            INSERT INTO quiz_attempts (quiz_id, user_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
            form.quiz_id,
            form.user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(attempt)
    }

    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let attempt = sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT * FROM quiz_attempts WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(attempt)
    }

    async fn update(pool: &PgPool, id: Uuid, form: Self::UpdateForm) -> Result<Self, AppError> {
        let attempt = sqlx::query_as!(
            QuizAttempt,
            r#"
            UPDATE quiz_attempts
            SET completed_at = $1,
                score = $2,
                updated_at = NOW()
            WHERE id = $3
            RETURNING *
            "#,
            form.completed_at,
            form.score,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(attempt)
    }

    async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM quiz_attempts WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl QuizAttempt {
    pub async fn get_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT *
            FROM quiz_attempts
            WHERE quiz_id = $1
            "#,
            quiz_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn get_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT *
            FROM quiz_attempts
            WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
