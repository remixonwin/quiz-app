use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

pub mod answer;
pub mod question;
pub mod quiz;
pub mod quiz_attempt;
pub mod user;

#[async_trait]
pub trait DbModel {
    type CreateForm;
    type UpdateForm;

    async fn create(pool: &PgPool, form: Self::CreateForm) -> Result<Self, AppError>
    where
        Self: Sized;

    async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError>
    where
        Self: Sized;

    async fn update(
        pool: &PgPool,
        id: Uuid,
        form: Self::UpdateForm,
    ) -> Result<Self, AppError>
    where
        Self: Sized;

    async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError>;
}
