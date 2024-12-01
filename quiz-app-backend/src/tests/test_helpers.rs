use actix_web::{
    test,
    web,
    App,
    Error,
    dev::{Service, ServiceResponse, ServiceRequest},
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::{
    routes::configure_routes,
    models::{User, Quiz, CreateQuiz},
    auth::generate_token,
};

#[derive(Debug, Deserialize)]
pub struct TestUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub struct TestContext {
    pub pool: PgPool,
    pub user_id: i32,
    pub token: String,
    pub app: Box<dyn Service<actix_http::Request, Response = ServiceResponse, Error = Error, Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ServiceResponse, Error>> + 'static>>> + 'static,
}

pub fn init_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
    std::env::set_var("DATABASE_URL", "postgres://postgres:password@localhost:5432/quiz_app_test");
}

pub async fn create_test_user(pool: &PgPool) -> (i32, String) {
    let mut attempt = 0;
    let max_attempts = 5;

    while attempt < max_attempts {
        let username = format!("testuser_{}", attempt);
        let password_hash = bcrypt::hash("testpass", bcrypt::DEFAULT_COST).unwrap();

        match sqlx::query!(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'user') RETURNING id",
            username,
            password_hash
        )
        .fetch_one(pool)
        .await {
            Ok(record) => {
                let token = generate_token(record.id, "user").unwrap();
                return (record.id, token);
            }
            Err(_) => attempt += 1,
        }
    }

    panic!("Failed to create unique test user after multiple attempts");
}

pub async fn setup_test_app(pool: PgPool) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = Error> + 'static {
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await
}

pub async fn create_test_quiz(ctx: &TestContext) -> Quiz {
    let quiz = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: ctx.user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", ctx.token)))
        .set_json(&quiz)
        .to_request();

    test::call_and_read_body_json(&ctx.app, req).await
}

pub async fn setup_test_context(pool: PgPool) -> TestContext {
    let (user_id, token) = create_test_user(&pool).await;
    let app = setup_test_app(pool.clone()).await;

    TestContext {
        pool,
        user_id,
        token,
        app: Box::new(app),
    }
}
