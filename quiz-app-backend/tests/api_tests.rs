use actix_web::{test, web, App};
use quiz_app_backend::routes;
use serde_json::{json, Value};
mod test_utils;
use test_utils::{setup_test_db, reset_db};
use tokio::time::{sleep, Duration};

#[actix_web::test]
async fn test_register_user() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser1",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Register response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}

#[actix_web::test]
async fn test_login_user() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    // First register the user
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser2",
            "password": "password123"
        }))
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    let register_status = register_resp.status();
    let body = test::read_body(register_resp).await;
    println!("Register response in login test: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(register_status.is_success());

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "username": "testuser2",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Login response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}

#[actix_web::test]
async fn test_create_quiz() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    // First register and login
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser3",
            "password": "password123"
        }))
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    let register_status = register_resp.status();
    println!("Register status: {:?}", register_status);
    assert!(register_status.is_success());

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "username": "testuser3",
            "password": "password123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    let login_status = login_resp.status();
    let login_body = test::read_body(login_resp).await;
    let login_json: Value = serde_json::from_slice(&login_body).unwrap();
    let token = login_json["token"].as_str().unwrap();
    println!("Login response: {:?}", String::from_utf8(login_body.to_vec()).unwrap());
    println!("Login status: {:?}", login_status);
    assert!(login_status.is_success());
    
    // Now create a quiz
    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "Test Quiz",
            "description": "A test quiz"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Create quiz status: {:?}", status);
    println!("Create quiz response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}
