use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::DbModel;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub answer_text: String,
    pub order_num: i32,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAnswer {
    pub question_id: Uuid,
    pub answer_text: String,
    pub order_num: i32,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAnswer {
    pub answer_text: String,
    pub order_num: i32,
    pub is_correct: bool,
}

#[async_trait::async_trait]
impl DbModel for Answer {
    type CreateForm = CreateAnswer;
    type UpdateForm = UpdateAnswer;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError> {
        let answer = sqlx::query_as!(
            Answer,
            r#"
            INSERT INTO answers (question_id, answer_text, order_num, is_correct)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            form.question_id,
            form.answer_text,
            form.order_num,
            form.is_correct
        )
        .fetch_one(pool)
        .await?;

        Ok(answer)
    }

    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let answer = sqlx::query_as!(
            Answer,
            r#"
            SELECT * FROM answers WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(answer)
    }

    async fn update(pool: &PgPool, id: Uuid, form: Self::UpdateForm) -> Result<Self, AppError> {
        let answer = sqlx::query_as!(
            Answer,
            r#"
            UPDATE answers
            SET answer_text = $1,
                order_num = $2,
                is_correct = $3,
                updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
            form.answer_text,
            form.order_num,
            form.is_correct,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(answer)
    }

    async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM answers WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl Answer {
    pub async fn get_by_question_id(pool: &PgPool, question_id: Uuid) -> Result<Vec<Self>, AppError> {
        let answers = sqlx::query_as!(
            Answer,
            r#"
            SELECT * FROM answers WHERE question_id = $1
            ORDER BY order_num ASC
            "#,
            question_id
        )
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, AppError> {
        let answers = sqlx::query_as!(
            Answer,
            r#"
            SELECT * FROM answers
            ORDER BY order_num ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }

    pub async fn get_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Self>, AppError> {
        let answers = sqlx::query_as!(
            Answer,
            r#"
            SELECT a.* 
            FROM answers a
            JOIN questions q ON a.question_id = q.id
            WHERE q.quiz_id = $1
            ORDER BY q.order_num ASC, a.order_num ASC
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }
}
