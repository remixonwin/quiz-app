use actix_web::test;
use sqlx::PgPool;
use quiz_app_backend::{
    models::CreateQuiz,
    test_helpers::{
        setup_test_app,
        create_test_quiz_with_title,
        setup_test_context,
        cleanup_test_data,
    },
};

#[actix_web::test]
async fn test_helpers_work_correctly() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Test creating a quiz
    let quiz_data = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz description".to_string()),
        created_by: ctx.user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", ctx.token)))
        .set_json(&quiz_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test creating quiz with title
    let quiz = create_test_quiz_with_title(&ctx.pool, ctx.user_id, "Another Test Quiz").await;
    assert_eq!(quiz.title, "Another Test Quiz");

    cleanup_test_data(&ctx.pool).await;
}
