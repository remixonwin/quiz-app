use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::DbModel;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question_text: String,
    pub order_num: i32,
    pub points: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: Uuid,
    pub question_text: String,
    pub order_num: i32,
    pub points: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuestion {
    pub question_text: String,
    pub order_num: i32,
    pub points: i32,
}

#[async_trait::async_trait]
impl DbModel for Question {
    type CreateForm = CreateQuestion;
    type UpdateForm = UpdateQuestion;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let question = sqlx::query_as!(
            Question,
            r#"
            INSERT INTO questions (quiz_id, question_text, order_num, points)
            VALUES ($1, $2, $3, $4)
            RETURNING id, quiz_id, question_text, order_num, points, created_at, updated_at
            "#,
            form.quiz_id,
            form.question_text,
            form.order_num,
            form.points
        )
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let question = sqlx::query_as!(
            Question,
            r#"
            SELECT id, quiz_id, question_text, order_num, points, created_at, updated_at FROM questions WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(question)
    }

    async fn update(pool: &PgPool, id: Uuid, form: Self::UpdateForm) -> Result<Self, AppError> {
        let question = sqlx::query_as!(
            Question,
            r#"
            UPDATE questions
            SET question_text = $1,
                order_num = $2,
                points = $3,
                updated_at = NOW()
            WHERE id = $4
            RETURNING id, quiz_id, question_text, order_num, points, created_at, updated_at
            "#,
            form.question_text,
            form.order_num,
            form.points,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM questions WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Question {
    pub async fn get_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Self>, AppError> {
        let questions = sqlx::query_as!(
            Question,
            r#"
            SELECT id, quiz_id, question_text, order_num, points, created_at, updated_at FROM questions WHERE quiz_id = $1
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }
}
