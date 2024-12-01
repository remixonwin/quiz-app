use actix_web::{test, http::StatusCode};
use quiz_app_backend::{
    models::{Quiz, CreateQuiz},
    test_helpers::{
        setup_test_app,
        create_test_quiz_with_title,
        verify_quiz_in_db,
        setup_test_context,
        cleanup_test_data,
        create_test_user,
    },
    auth::generate_token,
};
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;
use bcrypt;

#[actix_web::test]
async fn test_create_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Create test user
    let user_id = create_test_user(&ctx.pool).await;

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let quiz_data = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&quiz_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Expected 201 Created for quiz creation");

    let quiz: Quiz = test::read_body_json(resp).await;
    assert_eq!(quiz.title, quiz_data.title);
    assert_eq!(quiz.description, quiz_data.description);
    assert_eq!(quiz.created_by, user_id);

    let db_quiz = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(db_quiz.is_some(), "Quiz should exist in database");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_update_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    let user_id = create_test_user(&ctx.pool).await;

    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Original Title").await;

    let update_data = CreateQuiz {
        title: "Updated Title".to_string(),
        description: Some("Updated description".to_string()),
        created_by: user_id,
    };

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let req = test::TestRequest::put()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK, "Expected 200 OK for quiz update");

    let updated_quiz: Quiz = test::read_body_json(resp).await;
    assert_eq!(updated_quiz.title, "Updated Title");
    assert_eq!(updated_quiz.description, Some("Updated description".to_string()));

    let db_quiz = verify_quiz_in_db(&ctx.pool, updated_quiz.id).await;
    assert!(db_quiz.is_some(), "Quiz should exist in database");
    let db_quiz = db_quiz.unwrap();
    assert_eq!(db_quiz.title, "Updated Title");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_delete_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    let user_id = create_test_user(&ctx.pool).await;

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Quiz to Delete").await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Expected 204 No Content for quiz deletion");

    // Verify quiz no longer exists
    let deleted_quiz = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(deleted_quiz.is_none(), "Quiz should be deleted from database");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_delete_quiz_unauthorized() {
    // Set up initial test context
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Create first test user
    let user_id = create_test_user(&ctx.pool).await;

    // Create a quiz with the first user
    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Quiz to Delete").await;

    // Verify quiz exists in the database
    let quiz_exists = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(quiz_exists.is_some(), "Quiz should exist in database");

    // Create second test user
    let other_user_id = create_test_user(&ctx.pool).await;

    // Generate token for the other user
    let other_token = generate_token(other_user_id, "user").expect("Failed to generate token");

    // Try to delete the quiz with the other user
    let req = test::TestRequest::delete()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN, "Expected 403 Forbidden when trying to delete another user's quiz");

    // Verify quiz still exists
    let quiz_exists = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(quiz_exists.is_some(), "Quiz should still exist in database");

    cleanup_test_data(&ctx.pool).await;
}
