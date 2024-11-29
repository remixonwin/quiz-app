use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;
use log::info;

mod db;
mod models;
mod handlers;
mod auth;

use handlers::user::{register, login, get_profile, update_profile};
use handlers::quiz::{list_quizzes, create_quiz, get_quiz, submit_quiz};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Initialize database
    db::init_db(&pool).await.expect("Failed to initialize database");

    info!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            
            // User routes
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/profile", web::get().to(get_profile))
            .route("/profile", web::put().to(update_profile))
            
            // Quiz routes
            .route("/quizzes", web::get().to(list_quizzes))
            .route("/quizzes", web::post().to(create_quiz))
            .route("/quizzes/{quiz_id}", web::get().to(get_quiz))
            .route("/quizzes/{quiz_id}/submit", web::post().to(submit_quiz))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
