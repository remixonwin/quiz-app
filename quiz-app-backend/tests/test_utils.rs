#[allow(unused_imports)]
use quiz_app_backend::{
    models::{
        user::CreateUser,
        quiz::CreateQuiz,
        question::CreateQuestion,
        answer::CreateAnswer,
        user::User,
        quiz::Quiz,
        question::Question,
        answer::Answer,
        DbModel,
    },
    error::AppError,
};
#[allow(unused_imports)]
use sqlx::PgPool;
#[allow(unused_imports)]
use uuid::Uuid;
#[allow(unused_imports)]
use chrono::Utc;

pub async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/quiz_app_test".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

pub async fn reset_db(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers, quiz_attempts CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}

pub fn create_test_user() -> CreateUser {
    CreateUser {
        username: format!("test_user_{}", Uuid::new_v4()),
        email: format!("test_{}@example.com", Uuid::new_v4()),
        password: "password123".to_string(),
    }
}

pub async fn create_test_user_in_db(pool: &PgPool) -> Result<User, AppError> {
    User::create(pool, create_test_user()).await
}

pub fn create_test_quiz(user_id: Uuid) -> CreateQuiz {
    CreateQuiz {
        title: format!("Test Quiz {}", Uuid::new_v4()),
        description: Some("A test quiz description".to_string()),
        created_by: user_id,
    }
}

pub async fn create_test_quiz_in_db(pool: &PgPool, user_id: Uuid) -> Result<Quiz, AppError> {
    Quiz::create(pool, create_test_quiz(user_id)).await
}

pub fn create_test_question(quiz_id: Uuid) -> CreateQuestion {
    CreateQuestion {
        quiz_id,
        question_text: "What is the capital of France?".to_string(),
        order_num: 1,
        points: 1,
    }
}

pub async fn create_test_question_in_db(pool: &PgPool, quiz_id: Uuid) -> Result<Question, AppError> {
    Question::create(pool, create_test_question(quiz_id)).await
}

pub fn create_test_answer(question_id: Uuid) -> CreateAnswer {
    CreateAnswer {
        question_id,
        answer_text: "Paris".to_string(),
        order_num: 1,
        is_correct: true,
    }
}

pub async fn create_test_answer_in_db(pool: &PgPool, question_id: Uuid) -> Result<Answer, AppError> {
    Answer::create(pool, create_test_answer(question_id)).await
}

pub async fn cleanup_test_data(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
