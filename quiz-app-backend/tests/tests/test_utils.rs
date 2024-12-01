use sqlx::PgPool;
use quiz_app_backend::{
    models::*,
    error::AppError,
};
use std::sync::Once;
use rand::{thread_rng, Rng};

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
        
    // Clean up test database
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to clean test database");
        
    pool
}

pub async fn create_test_user(pool: &PgPool) -> Result<User, AppError> {
    let username = format!("test_user_{}", thread_rng().gen::<u32>());
    let create_user = CreateUser {
        username: username.clone(),
        password: "test_password123".to_string(),
    };
    
    User::create(pool, create_user).await
}

pub async fn create_test_quiz(pool: &PgPool, user_id: i32) -> Result<Quiz, AppError> {
    let create_quiz = CreateQuiz {
        title: format!("Test Quiz {}", thread_rng().gen::<u32>()),
        description: Some("A test quiz".to_string()),
        created_by: user_id,
    };
    
    Quiz::create(pool, create_quiz).await
}

pub async fn create_test_question(pool: &PgPool, quiz_id: i32) -> Result<Question, AppError> {
    let create_question = CreateQuestion {
        quiz_id,
        question_text: format!("Test question {}", thread_rng().gen::<u32>()),
        order_num: Some(1),
    };
    
    Question::create(pool, create_question).await
}

pub async fn create_test_answer(pool: &PgPool, question_id: i32) -> Result<Answer, AppError> {
    let create_answer = CreateAnswer {
        text: format!("Test answer {}", thread_rng().gen::<u32>()),
        is_correct: true,
    };
    
    sqlx::query_as!(
        Answer,
        r#"
        INSERT INTO answers (question_id, text, is_correct, order_num)
        VALUES ($1, $2, $3, $4)
        RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
        "#,
        question_id,
        create_answer.text,
        create_answer.is_correct,
        1  // Default order_num
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_create_test_user() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        assert!(!user.username.is_empty());
        assert!(!user.password_hash.is_empty());
    }

    #[actix_web::test]
    async fn test_create_test_quiz() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        assert!(!quiz.title.is_empty());
        assert!(quiz.description.is_some());
    }

    #[actix_web::test]
    async fn test_create_test_question() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        let question = create_test_question(&pool, quiz.id).await.unwrap();
        assert!(!question.question_text.is_empty());
        assert_eq!(question.quiz_id, quiz.id);
    }

    #[actix_web::test]
    async fn test_create_test_answer() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        let question = create_test_question(&pool, quiz.id).await.unwrap();
        let answer = create_test_answer(&pool, question.id).await.unwrap();
        assert!(!answer.text.is_empty());
        assert_eq!(answer.question_id, question.id);
    }
}
