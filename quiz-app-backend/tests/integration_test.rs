use actix_web::{
    test,
    web,
    App,
    http::StatusCode,
};
use quiz_app_backend::{
    config::get_config,
    auth::generate_token,
    models::{
        User,
        CreateUser,
        LoginCredentials,
        CreateQuiz,
    },
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[actix_web::test]
    async fn test_health_check() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_registration() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());

        let create_user_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, create_user_req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_user_login() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let password = "testpass123".to_string();
        
        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, register_req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, login_req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_quiz_creation_and_retrieval() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        
        // Create a test user in the database
        let user = sqlx::query_as!(
            User,
            // ...existing code...
            "user"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let token = generate_token(user.id, "user").unwrap();

        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            // ...existing code...
            created_by: user.id,
        };

        let create_resp = test::TestRequest::post()
            .uri("/api/quizzes")
            // ...existing code...
            .to_request();

        let quiz: CreateQuiz = test::call_and_read_body_json(&app, create_resp).await;
        assert_eq!(quiz.title, "Test Quiz");
        assert_eq!(quiz.description, Some("A test quiz".to_string()));
        assert_eq!(quiz.created_by, user.id);
    }

    #[actix_web::test]
    async fn test_user_registration_and_login() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        // Test registration
        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let password = "testpass123".to_string();
        
        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();
            
        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status(), StatusCode::CREATED, "Registration failed with status: {}", register_resp.status());
        
        // Test login
        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            // ...existing code...
            .to_request();
            
        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_quiz() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let user_id = sqlx::query!(
            "INSERT INTO users (username, password_hash, role) 
            // ...existing code...
            "user"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .id;

        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            // ...existing code...
            created_by: user_id,
        };

        let create_resp = test::TestRequest::post()
            .uri("/api/quizzes")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, create_resp).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
