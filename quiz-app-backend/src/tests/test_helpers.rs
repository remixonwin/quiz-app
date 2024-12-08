use actix_web::{
    test,
    web,
    App,
    Error,
    dev::{Service, ServiceResponse},
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{
    routes::configure_routes,
    models::{User, Quiz, CreateQuiz},
    auth::generate_token,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct TestUser {
    pub id: Uuid,
    pub username: String,
    pub token: String,
}

pub struct TestContext {
    pub pool: PgPool,
    pub app_state: web::Data<AppState>,
    pub user_id: Uuid,
    pub token: String,
}

pub fn init_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
    std::env::set_var("DATABASE_URL", "postgres://postgres:password@localhost:5432/quiz_app_test");
}

pub async fn create_test_user(pool: &PgPool) -> (Uuid, String) {
    let mut attempt = 0;
    let max_attempts = 5;

    while attempt < max_attempts {
        let username = format!("testuser_{}", attempt);
        let password_hash = bcrypt::hash("testpass", bcrypt::DEFAULT_COST).unwrap();
        let user_id = Uuid::new_v4();

        match sqlx::query!(
            "INSERT INTO users (id, username, password_hash, role) VALUES ($1, $2, $3, 'user') RETURNING id",
            user_id,
            username,
            password_hash
        )
        .fetch_one(pool)
        .await {
            Ok(record) => {
                let token = generate_token(user_id, "user").unwrap();
                return (user_id, token);
            }
            Err(_) => attempt += 1,
        }
    }

    panic!("Failed to create unique test user after multiple attempts");
}

pub async fn setup_test_app(pool: PgPool) -> App<
    impl Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
> {
    let app_state = web::Data::new(AppState { pool: pool.clone() });
    
    App::new()
        .app_data(app_state.clone())
        .configure(configure_routes)
}

pub async fn create_test_quiz(ctx: &TestContext) -> Quiz {
    let quiz = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: ctx.user_id,
        questions: Vec::new(),
    };

    let app = setup_test_app(ctx.pool.clone()).await;
    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", ctx.token)))
        .set_json(&quiz)
        .to_request();

    let app = test::init_service(app).await;
    test::call_and_read_body_json(&app, req).await
}

pub async fn setup_test_context(pool: PgPool) -> TestContext {
    let (user_id, token) = create_test_user(&pool).await;
    let app_state = web::Data::new(AppState { pool: pool.clone() });

    TestContext {
        pool,
        app_state,
        user_id,
        token,
    }
}
