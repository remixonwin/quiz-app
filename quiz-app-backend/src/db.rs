// Removed unused imports

pub async fn init_db(pool: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    // Skip database initialization if SKIP_MIGRATIONS is set
    if std::env::var("SKIP_MIGRATIONS").is_ok() {
        println!("Skipping database migrations due to SKIP_MIGRATIONS environment variable");
        return Ok(());
    }

    // Create UUID extension if it doesn't exist
    sqlx::query!("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"").execute(pool).await?;

    // Drop existing tables and triggers in reverse order of dependencies
    sqlx::query!("DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes CASCADE").execute(pool).await?;
    sqlx::query!("DROP FUNCTION IF EXISTS update_updated_at_column CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS submitted_answers CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS quiz_attempts CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS answers CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS questions CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS quizzes CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS users CASCADE").execute(pool).await?;

    // Create users table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create quizzes table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS quizzes (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            title VARCHAR(255) NOT NULL,
            description TEXT,
            creator_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger function for automatically updating updated_at
    sqlx::query!(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ language 'plpgsql'
        "#
    )
    .execute(pool)
    .await?;

    // Drop trigger if exists
    sqlx::query!(
        r#"
        DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger
    sqlx::query!(
        r#"
        CREATE TRIGGER update_quizzes_updated_at
            BEFORE UPDATE ON quizzes
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()
        "#
    )
    .execute(pool)
    .await?;

    // Create questions table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS questions (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            question_type VARCHAR(50) NOT NULL,
            order_num INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(quiz_id, order_num)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create answers table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS answers (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            order_num INTEGER NOT NULL,
            UNIQUE(question_id, order_num)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create quiz_attempts table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS quiz_attempts (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            user_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            completed_at TIMESTAMPTZ,
            score INTEGER,
            UNIQUE(quiz_id, user_id, created_at)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create submitted_answers table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS submitted_answers (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            attempt_id UUID NOT NULL REFERENCES quiz_attempts(id) ON DELETE CASCADE,
            question_id UUID NOT NULL REFERENCES questions(id),
            answer_id UUID NOT NULL REFERENCES answers(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(attempt_id, question_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        create_pool(&database_url).await
    }

    #[tokio::test]
    async fn test_database_connection() {
        let pool = create_test_pool().await.expect("Failed to create pool");
        let result = sqlx::query!("SELECT 1").fetch_one(&pool).await;
        assert!(result.is_ok());
    }
}
