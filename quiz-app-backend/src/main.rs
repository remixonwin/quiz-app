mod auth;
mod error;
mod handlers;
mod middleware;
mod models;
mod seeder;

use actix_web::{
    web, App, HttpServer, HttpResponse,
    middleware::Logger,
};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenv::dotenv;

use crate::{
    handlers::{quiz, user},
    auth::Auth,
    middleware::CacheMiddleware,
};

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)  
        .min_connections(2)   
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    println!("Starting server at {}", address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let cache = CacheMiddleware::new();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .wrap(cache)  
                    .service(health_check)
                    .service(user::login)
                    .service(user::register)
                    .service(
                        web::scope("/quizzes")
                            .wrap(Auth)  
                            .service(quiz::create_quiz)
                            .service(quiz::get_quizzes)
                            .service(quiz::get_quiz)
                            .service(quiz::update_quiz)
                            .service(quiz::delete_quiz)
                            .service(quiz::submit_quiz)
                    )
            )
    })
    .bind(address)?
    .run()
    .await
}
