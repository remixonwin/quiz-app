use quiz_app_backend::{
    models::{CreateUser, CreateQuiz, CreateQuestion, CreateAnswer, User, Quiz, Question, Answer, DbModel},
    error::AppError,
};
use sqlx::PgPool;
use rand::prelude::*;
use rand::SeedableRng;
use std::env;

pub async fn setup_test_db() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to database")
}

pub async fn reset_db(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers, quiz_attempts CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_test_user(pool: &PgPool) -> Result<i32, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_user = CreateUser {
        username: format!("testuser_{}", rng.gen::<u32>()),
        password: "password123".to_string(),
    };
    
    let user = User::create(pool, create_user).await?;
    Ok(user.id)
}

pub async fn create_test_quiz(pool: &PgPool, user_id: i32) -> Result<Quiz, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_quiz = CreateQuiz {
        title: format!("Test Quiz {}", rng.gen::<u32>()),
        description: Some("Test Description".to_string()),
        created_by: user_id,
    };
    
    Quiz::create(pool, create_quiz).await
}

pub async fn create_test_question(pool: &PgPool, quiz_id: i32) -> Result<Question, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_question = CreateQuestion {
        quiz_id,
        question_text: format!("Test Question {}", rng.gen::<u32>()),
        order_num: Some(1),
    };
    
    Question::create(pool, create_question).await
}

pub async fn create_test_answer(pool: &PgPool, _question_id: i32) -> Result<Answer, AppError> {
    let create_answer = CreateAnswer {
        text: "Test Answer".to_string(),
        is_correct: true,
    };
    
    Answer::create(pool, create_answer).await
}

pub async fn cleanup_test_data(pool: &PgPool) {
    reset_db(pool).await.expect("Failed to reset database");
}
