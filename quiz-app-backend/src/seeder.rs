use sqlx::PgPool;
use chrono::Utc;

use crate::{
    models::User,
    auth::password::hash_password,
    error::AppError,
};

pub async fn seed_database(pool: &PgPool) -> Result<(), AppError> {
    // First clean up any existing data
    sqlx::query!("DELETE FROM answers").execute(pool).await?;
    sqlx::query!("DELETE FROM questions").execute(pool).await?;
    sqlx::query!("DELETE FROM quiz_attempts").execute(pool).await?;
    sqlx::query!("DELETE FROM quizzes").execute(pool).await?;
    sqlx::query!("DELETE FROM users").execute(pool).await?;

    // Create a test user
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        "test_user",
        "test@example.com",
        hash_password("password123")?,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    // Create some test quizzes
    let quiz_id = sqlx::query!(
        r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "#,
        "Sample Quiz",
        Some("A sample quiz for testing"),
        user.id,
        Utc::now()
    )
    .fetch_one(pool)
    .await?
    .id;

    // Create some test questions
    let question_id = sqlx::query!(
        r#"
        INSERT INTO questions (quiz_id, question_text, points, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "#,
        quiz_id,
        "What is the capital of France?",
        10,
        Utc::now()
    )
    .fetch_one(pool)
    .await?
    .id;

    // Create some test answers
    sqlx::query!(
        r#"
        INSERT INTO answers (question_id, answer_text, is_correct, created_at, updated_at)
        VALUES 
        ($1, $2, $3, $4, $4),
        ($1, $5, $6, $4, $4),
        ($1, $7, $8, $4, $4),
        ($1, $9, $10, $4, $4)
        "#,
        question_id,
        "Paris", true,
        Utc::now(),
        "London", false,
        "Berlin", false,
        "Madrid", false
    )
    .execute(pool)
    .await?;

    Ok(())
}
