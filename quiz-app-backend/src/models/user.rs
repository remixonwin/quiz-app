use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(pool: &PgPool, user: CreateUser) -> Result<Self, AppError> {
        let hashed_password = hash(user.password.as_bytes(), DEFAULT_COST)?;
        let now = Utc::now();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password, name, role, created_at, updated_at)
            VALUES ($1, $2, $3, 'User', $4, $5)
            RETURNING id, email, password, name, role as "role: UserRole", created_at, updated_at
            "#,
            user.email,
            hashed_password,
            user.name,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, name, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password, name, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user: CreateUser,
    ) -> Result<Self, AppError> {
        let user = Self::get_by_id(pool, id).await?.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        let now = Utc::now();

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = $1, email = $2, updated_at = $3
            WHERE id = $4
            RETURNING id, email, password, name, role as "role: UserRole", created_at, updated_at
            "#,
            user.name,
            user.email,
            now,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_user)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password).unwrap_or(false)
    }
}
