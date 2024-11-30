use actix_web::{test, web, App};
use quiz_app_backend::{routes, models};
use serde_json::json;

#[actix_web::test]
async fn test_register_user() {
    let app = test::init_service(
        App::new()
            .configure(routes::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_login_user() {
    let app = test::init_service(
        App::new()
            .configure(routes::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_create_quiz() {
    let app = test::init_service(
        App::new()
            .configure(routes::configure_routes)
    ).await;

    // First login to get token
    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    assert!(login_resp.status().is_success());
    
    // Now create a quiz
    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .set_json(json!({
            "title": "Test Quiz",
            "description": "A test quiz"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
