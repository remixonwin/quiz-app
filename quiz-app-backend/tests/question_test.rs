#[cfg(test)]
mod question_tests {
    use actix_web::{
        test, web, App,
        http::StatusCode,
    };
    use quiz_app_backend::{
        models::{CreateUser, CreateQuiz, CreateQuestion},
        handlers::question,
        auth::{generate_token, Auth},
    };
    use sqlx::PgPool;
    use std::env;
    use chrono::Utc;

    async fn setup() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // Set JWT secret for testing
        env::set_var("JWT_SECRET", "test_secret");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[actix_web::test]
    async fn test_create_question() {
        let pool = setup().await;
        let user_id = create_test_user(&pool).await;
        let token = generate_token(user_id, "user".to_string()).unwrap();
        let quiz_id = create_test_quiz(&pool, user_id).await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::scope("/api")
                        .wrap(Auth)
                        .service(
                            web::scope("/quizzes")
                                .route("/{quiz_id}/questions", web::post().to(question::create_question))
                                .route("/{quiz_id}/questions", web::get().to(question::list_questions))
                        )
                )
        ).await;

        let question_data = CreateQuestion {
            quiz_id: quiz_id,
            question_text: "Test Question".to_string(),
            order_num: Some(1),
        };

        let req = test::TestRequest::post()
            .uri(&format!("/api/quizzes/{}/questions", quiz_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&question_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Cleanup
        let _ = sqlx::query!("DELETE FROM questions WHERE quiz_id = $1", quiz_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await;
    }

    async fn create_test_user(pool: &PgPool) -> i32 {
        let new_user = CreateUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };

        sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            new_user.username,
            new_user.email,
            new_user.password,
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test user")
        .id
    }

    async fn create_test_quiz(pool: &PgPool, user_id: i32) -> i32 {
        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
        };

        sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id
            "#,
            quiz_data.title,
            quiz_data.description,
            user_id,
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test quiz")
        .id
    }
}
