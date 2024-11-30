#[cfg(test)]
mod handlers_tests {
    use actix_web::{
        test, web, App, HttpResponse,
        http::StatusCode,
    };
    use quiz_app_backend::{
        handlers::quiz,
        models::{CreateUser, CreateQuiz},
        auth::{generate_token, Auth},
    };
    use sqlx::PgPool;
    use std::env;
    use serde_json::json;
    use chrono;

    async fn setup() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // Set JWT secret for testing
        env::set_var("JWT_SECRET", "test_secret");
        PgPool::connect(&database_url).await.unwrap()
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
            chrono::Utc::now()
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test user")
        .id
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api")
                    .route("/health", web::get().to(|| async { HttpResponse::Ok().finish() })))
        ).await;

        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_quiz() {
        let pool = setup().await;
        let user_id = create_test_user(&pool).await;
        let token = generate_token(user_id, "user".to_string()).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::scope("/api")
                        .wrap(Auth)
                        .route("/quizzes", web::post().to(quiz::create_quiz))
                )
        ).await;

        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/api/quizzes")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&quiz_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Test without auth token
        let req = test::TestRequest::post()
            .uri("/api/quizzes")
            .set_json(&quiz_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Cleanup
        let _ = sqlx::query!("DELETE FROM quizzes WHERE title = $1", quiz_data.title)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await;
    }

    #[actix_web::test]
    async fn test_update_quiz() {
        let pool = setup().await;

        // First create a test user
        let user_id = create_test_user(&pool).await;

        // Create a quiz first
        let quiz_data = CreateQuiz {
            title: "Original Quiz".to_string(),
            description: Some("Original Description".to_string()),
        };

        let quiz = sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id
            "#,
            quiz_data.title,
            quiz_data.description,
            user_id,
            chrono::Utc::now()
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to create test quiz");

        // Generate token for the user
        let token = generate_token(user_id, "user".to_string()).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::scope("/api")
                        .wrap(Auth)
                        .route("/quizzes/{id}", web::put().to(quiz::update_quiz))
                )
        ).await;

        let update_data = json!({
            "title": "Updated Quiz",
            "description": "Updated Description"
        });

        let req = test::TestRequest::put()
            .uri(&format!("/api/quizzes/{}", quiz.id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&update_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["title"], "Updated Quiz");
        assert_eq!(body["description"], "Updated Description");

        // Test unauthorized update
        let another_user = create_test_user(&pool).await;

        let another_token = generate_token(another_user, "user".to_string()).unwrap();

        let req = test::TestRequest::put()
            .uri(&format!("/api/quizzes/{}", quiz.id))
            .insert_header(("Authorization", format!("Bearer {}", another_token)))
            .set_json(&update_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Cleanup
        let _ = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz.id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", another_user)
            .execute(&pool)
            .await;
    }

    #[actix_web::test]
    async fn test_delete_quiz() {
        let pool = setup().await;
        std::env::set_var("JWT_SECRET", "test_secret");

        // First create a test user
        let user_id = create_test_user(&pool).await;

        // Create a quiz first
        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
        };

        let quiz = sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id
            "#,
            quiz_data.title,
            quiz_data.description,
            user_id,
            chrono::Utc::now()
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to create test quiz");

        // Generate token for the user
        let token = generate_token(user_id, "user".to_string()).unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(Auth)
                .service(web::scope("/api")
                    .route("/quizzes/{quiz_id}", web::delete().to(quiz::delete_quiz)))
        ).await;

        // Test unauthorized delete
        let another_user = create_test_user(&pool).await;

        let another_token = generate_token(another_user, "user".to_string()).unwrap();

        let req = test::TestRequest::delete()
            .uri(&format!("/api/quizzes/{}", quiz.id))
            .insert_header(("Authorization", format!("Bearer {}", another_token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Test successful delete
        let req = test::TestRequest::delete()
            .uri(&format!("/api/quizzes/{}", quiz.id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        // Verify quiz is deleted
        let quiz_exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM quizzes WHERE id = $1) as "exists!"
            "#,
            quiz.id
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to check quiz existence");

        assert!(!quiz_exists.exists, "Quiz should be deleted");

        // Cleanup users
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", another_user)
            .execute(&pool)
            .await;
    }
}
