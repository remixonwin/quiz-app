// Removed unused imports

pub async fn init_db(pool: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    // Drop existing tables in reverse order of dependencies
    sqlx::query!("DROP TABLE IF EXISTS quiz_attempts CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS answers CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS questions CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS quizzes CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS users CASCADE").execute(pool).await?;

    // Create users table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
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
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            creator_id INTEGER NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create questions table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS questions (
            id SERIAL PRIMARY KEY,
            quiz_id INTEGER NOT NULL REFERENCES quizzes(id),
            question_text TEXT NOT NULL,
            question_type VARCHAR(50) NOT NULL,
            order_num INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create answers table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS answers (
            id SERIAL PRIMARY KEY,
            question_id INTEGER NOT NULL REFERENCES questions(id),
            answer_text TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            order_num INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create quiz_attempts table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS quiz_attempts (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            quiz_id INTEGER NOT NULL REFERENCES quizzes(id),
            score INTEGER,
            completed_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions; 
    use dotenv::dotenv; 

    #[tokio::test]
    async fn test_database_connection() {
        dotenv().ok(); 
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        println!("DATABASE_URL: {}", database_url); 

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("Failed to create pool");

        init_db(&pool).await.expect("Failed to initialize database");
    }
}
