#[cfg(test)]
mod db_tests {
    use sqlx::PgPool;
    use quiz_app_backend::models::user::{CreateUser, User};
    use quiz_app_backend::models::DbModel;
    use chrono::Utc;
    use dotenv::dotenv;
    use uuid::Uuid;

    async fn setup() -> PgPool {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let new_user = CreateUser {
            username: "testuser".to_string(),
            email: format!("test{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
        };

        let result = User::create(&pool, new_user).await;
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_quiz_creation(pool: PgPool) {
        // First create a user to be the creator
        let user_result = sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at, role, email)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            format!("quizcreator_{}", Uuid::new_v4()),
            "password123",
            Utc::now(),
            "user",
            format!("quizcreator_{}@example.com", Uuid::new_v4())
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
