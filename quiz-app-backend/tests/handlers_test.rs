use actix_web::http::StatusCode;
use serde_json::json;

use quiz_app_backend::{
    models::{
        user::{CreateUser, User},
        quiz::{Quiz, CreateQuiz},
        question::{Question, CreateQuestion},
        answer::{Answer, CreateAnswer},
    },
    test_helpers::TestContext,
};

#[actix_web::test]
async fn test_create_user() {
    let ctx = TestContext::new().await;
    
    let user = CreateUser {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/users/register",
            Some(serde_json::to_string(&user).unwrap()),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    
    ctx.cleanup().await.unwrap();
}

#[actix_web::test]
async fn test_login_user() {
    let ctx = TestContext::new().await;
    
    // First create a user
    let user = CreateUser {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
        name: "Test User".to_string(),
    };

    let _ = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/users/register",
            Some(serde_json::to_string(&user).unwrap()),
        )
        .await
        .unwrap();

    // Now try to login
    let login_data = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/users/login",
            Some(login_data.to_string()),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    
    ctx.cleanup().await.unwrap();
}

#[actix_web::test]
async fn test_create_quiz() {
    let ctx = TestContext::new().await;
    
    let quiz = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: ctx.user_id,
    };

    let resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/quizzes",
            Some(serde_json::to_string(&quiz).unwrap()),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    
    ctx.cleanup().await.unwrap();
}

#[actix_web::test]
async fn test_get_quizzes() {
    let ctx = TestContext::new().await;
    
    // Create a quiz first
    let quiz = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: ctx.user_id,
    };

    let _ = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/quizzes",
            Some(serde_json::to_string(&quiz).unwrap()),
        )
        .await
        .unwrap();

    // Now try to get all quizzes
    let resp = ctx
        .make_request(
            actix_web::http::Method::GET,
            "/api/quizzes",
            None,
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    
    ctx.cleanup().await.unwrap();
}

#[actix_web::test]
async fn test_create_question() {
    let ctx = TestContext::new().await;
    
    // First create a quiz
    let quiz = ctx.create_test_quiz("Test Quiz".to_string()).await.unwrap();
    
    let question = CreateQuestion {
        quiz_id: quiz.id,
        text: "What is 2+2?".to_string(),
        order: 1,
    };

    let resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/questions",
            Some(serde_json::to_string(&question).unwrap()),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    
    ctx.cleanup().await.unwrap();
}

#[actix_web::test]
async fn test_create_answer() {
    let ctx = TestContext::new().await;
    
    // Create quiz and question first
    let quiz = ctx.create_test_quiz("Test Quiz".to_string()).await.unwrap();
    
    let question = CreateQuestion {
        quiz_id: quiz.id,
        text: "What is 2+2?".to_string(),
        order: 1,
    };

    let question_resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/questions",
            Some(serde_json::to_string(&question).unwrap()),
        )
        .await
        .unwrap();
    
    let question_body = test::read_body(question_resp).await;
    let question: Question = serde_json::from_slice(&question_body).unwrap();

    let answer = CreateAnswer {
        question_id: question.id,
        text: "4".to_string(),
        is_correct: true,
    };

    let resp = ctx
        .make_request(
            actix_web::http::Method::POST,
            "/api/answers",
            Some(serde_json::to_string(&answer).unwrap()),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    
    ctx.cleanup().await.unwrap();
}
