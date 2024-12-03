use actix_web::{web, App, HttpServer, middleware::Logger, HttpResponse};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use env_logger::Env;

mod auth;
mod handlers;
mod models;
mod error;
mod middleware;
mod seeder;

use crate::{
    auth::Auth,
    handlers::{
        user::{register, login, update_profile},
        quiz::{create_quiz, get_quizzes, get_quiz, update_quiz, delete_quiz, submit_quiz},
        question::{get_questions, create_question, get_question, update_question, delete_question, create_answer, get_answers},
        answer::{get_answer, update_answer, delete_answer},
    },
};

#[actix_web::get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .service(
                web::scope("/api/users")
                    .service(register)
                    .service(login)
                    .service(update_profile)
            )
            .service(
                web::scope("/api/quizzes")
                    .wrap(Auth)
                    .service(create_quiz)
                    .service(get_quizzes)
                    .service(get_quiz)
                    .service(update_quiz)
                    .service(delete_quiz)
                    .service(submit_quiz)
                    .service(
                        web::scope("/{quiz_id}/questions")
                            .wrap(Auth)
                            .service(get_questions)
                            .service(create_question)
                            .service(get_question)
                            .service(update_question)
                            .service(delete_question)
                            .service(create_answer)
                            .service(get_answers)
                            .service(
                                web::scope("/{question_id}/answers")
                                    .wrap(Auth)
                                    .service(get_answer)
                                    .service(update_answer)
                                    .service(delete_answer)
                            )
                    )
            )
    })
    .bind(address)?
    .run()
    .await
}
