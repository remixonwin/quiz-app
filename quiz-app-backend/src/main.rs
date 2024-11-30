mod auth;
mod error;
mod handlers;
mod models;
mod seeder;

use actix_web::{
    web, App, HttpServer,
    middleware::Logger,
};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::{
    handlers::{quiz, user},
    auth::Auth,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Starting server at http://127.0.0.1:8080");

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
            .service(
                web::scope("/api")
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
