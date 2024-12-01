#[cfg(test)]
mod db_tests {
    use sqlx::PgPool;
    use quiz_app_backend::models::CreateUser;
    use chrono::Utc;
    use dotenv::dotenv;

    async fn setup() -> PgPool {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_user_creation() {
        let pool = setup().await;

        let new_user = CreateUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, created_at
            "#,
            new_user.username,
            new_user.password,
            Utc::now()
        )
        .fetch_one(&pool)
        .await;

        assert!(result.is_ok());
        
        // Cleanup
        let user = result.unwrap();
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user.id)
            .execute(&pool)
            .await;
    }

    #[tokio::test]
    async fn test_quiz_creation() {
        let pool = setup().await;

        // First create a user to be the creator
        let user_result = sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            "quizcreator",
            "password123",
            Utc::now()
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to create test user");

        let title = "Test Quiz".to_string();
        let description = "Test Description".to_string();
        let created_at = Utc::now();

        let result = sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            title,
            description,
            user_result.id,
            created_at
        )
        .fetch_one(&pool)
        .await;

        if let Err(ref e) = result {
            eprintln!("Quiz creation failed: {:?}", e);
        }
        assert!(result.is_ok());
        
        // Cleanup
        if let Ok(quiz) = result {
            let _ = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz.id)
                .execute(&pool)
                .await;
        }
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_result.id)
            .execute(&pool)
            .await;
    }
}
