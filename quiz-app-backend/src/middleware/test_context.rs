
use sqlx::PgPool;
use chrono;
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
use uuid::Uuid;
use bcrypt;

pub struct TestContext {
    // ...existing code...
}

pub fn init_test_env() {
    // ...existing code...
}

pub async fn setup_test_context() -> TestContext {
    init_test_env();
    let config = get_config().expect("Failed to load config");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    cleanup_test_data(&pool).await;

    let now = chrono::Utc::now();
    let unique_id = Uuid::new_v4();
    let username = format!("test_user_{}", unique_id);
    let password = bcrypt::hash("test_password123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user_id = sqlx::query!(
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
    .expect("Failed to create test user")
    .id;

    TestContext {
        // ...existing code...
    }
}

pub async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query!("DELETE FROM quizzes").execute(pool).await.expect("Failed to clean up quizzes");
    sqlx::query!("DELETE FROM users").execute(pool).await expect("Failed to clean up users");
}