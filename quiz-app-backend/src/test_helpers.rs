use sqlx::PgPool;
use chrono::{self, NaiveDateTime};
use crate::{
    routes::configure_routes,
    models::Quiz,
    config::get_config,
    auth::generate_token,
};
use actix_web::{
    test, 
    web, 
    App,
    dev::{Service, ServiceResponse},
    Error,
};
use actix_http::Request;
use bcrypt;
use uuid::Uuid;

pub struct TestContext {
    pub pool: PgPool,
    pub user_id: Uuid, // Changed from i32 to Uuid
    pub token: String,
}

pub fn init_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
}

pub async fn create_test_user(pool: &PgPool) -> Uuid { // Changed return type from i32 to Uuid
    let now = chrono::Utc::now();
    let username = format!("test_user_{}", uuid::Uuid::new_v4());
    let password = bcrypt::hash("test_password123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO users (username, password_hash, role)
        VALUES ($1, $2, 'test')
        RETURNING id
        "#,
        username,
        password
    )
    .fetch_one(pool)
    .await
    .unwrap();

    user_id
}

pub async fn create_test_quiz(pool: &PgPool, user_id: Uuid) -> Uuid { // Changed id type to Uuid
    let now = NaiveDateTime::from_timestamp(chrono::Utc::now().timestamp(), 0); // Use NaiveDateTime
    let quiz_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "#,
        "Test Quiz",
        "A quiz for testing",
        user_id,
        now
    )
    .fetch_one(pool)
    .await
    .unwrap();

    quiz_id
}

pub async fn create_test_quiz_with_title(pool: &PgPool, user_id: Uuid, title: &str) -> Quiz {
    let now = chrono::Utc::now();
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        INSERT INTO quizzes (title, description, creator_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, title, description, creator_id, created_at, updated_at
        "#,
        title,
        "A test quiz description",
        user_id,
        now
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test quiz");

    quiz
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Delete all quizzes first due to foreign key constraints
    match sqlx::query!("DELETE FROM quizzes")
        .execute(pool)
        .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("Error cleaning up quizzes: {}", e),
    }

    // Then delete all test users
    match sqlx::query!("DELETE FROM users")
        .execute(pool)
        .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("Error cleaning up users: {}", e),
    }

    // Verify cleanup
    let quiz_count = sqlx::query!("SELECT COUNT(*) as count FROM quizzes")
        .fetch_one(pool)
        .await
        .expect("Failed to count quizzes")
        .count
        .unwrap_or(0);

    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(pool)
        .await
        .expect("Failed to count users")
        .count
        .unwrap_or(0);

    assert_eq!(quiz_count, 0, "Failed to clean up all quizzes");
    assert_eq!(user_count, 0, "Failed to clean up all users");
}

pub async fn verify_quiz_in_db(pool: &PgPool, quiz_id: Uuid) -> Option<Quiz> {
    sqlx::query_as!(
        Quiz,
        r#"
        SELECT id, title, description, creator_id, created_at, updated_at
        FROM quizzes 
        WHERE id = $1
        "#,
        quiz_id
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to query database")
}

pub async fn setup_test_app(
    pool: PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await
}

pub async fn setup_test_context() -> TestContext {
    init_test_env();
    let config = get_config().expect("Failed to load config");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    // Clean up any existing test data
    sqlx::query!("DELETE FROM quizzes")
        .execute(&pool)
        .await
        .expect("Failed to clean up quizzes");

    sqlx::query!("DELETE FROM users")
        .execute(&pool)
        .await
        .expect("Failed to clean up users");

    // Create test user
    let now = chrono::Utc::now();
    let username = format!("test_user_{}", uuid::Uuid::new_v4());
    let password = bcrypt::hash("test_password123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user_id = sqlx::query_scalar!(
        r#"
        INSERT INTO users (username, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING id
        "#,
        username,
        password,
        now
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create test user");

    // Verify user was created
    let user_exists = sqlx::query!(
        "SELECT id FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to verify user creation");

    assert!(user_exists.is_some(), "Test user was not created successfully");
    
    let token = generate_token(user_id, "user").expect("Failed to generate token");
    
    TestContext {
        pool,
        user_id,
        token,
    }
}
