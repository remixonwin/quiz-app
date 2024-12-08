use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::error::AppError;
use super::DbModel;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuiz {
    pub title: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuiz {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[async_trait]
impl DbModel for Quiz {
    type CreateForm = CreateQuiz;
    type UpdateForm = UpdateQuiz;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            INSERT INTO quizzes (title, description, created_by)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            form.title,
            form.description,
            form.created_by
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz)
    }

    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            SELECT * FROM quizzes WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(quiz)
    }

    async fn update(pool: &PgPool, id: Uuid, form: Self::UpdateForm) -> Result<Self, AppError> {
        let quiz = sqlx::query_as!(
            Quiz,
            r#"
            UPDATE quizzes
            SET title = COALESCE($1, title),
                description = COALESCE($2, description),
                completed_at = COALESCE($3, completed_at),
                updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
            form.title,
            form.description,
            form.completed_at,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(quiz)
    }

    async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM quizzes WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[allow(dead_code)]
impl Quiz {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at, completed_at
            FROM quizzes
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn get_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Self>, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at, completed_at
            FROM quizzes 
            WHERE created_by = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, created_by, created_at, updated_at, completed_at
            FROM quizzes WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
}
