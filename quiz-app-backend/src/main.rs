use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use log::{info, warn, error};

mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;

pub mod handlers {
    pub mod auth;
    pub mod user;
    pub mod quiz;
    pub mod quiz_attempt;
}

use crate::{
    config::get_config,
    auth::JwtAuthService,
    handlers::quiz::{get_quizzes, get_quiz, create_quiz, update_quiz, delete_quiz},
    handlers::quiz_attempt::{start_quiz_attempt, get_quiz_attempts_by_quiz, get_user_quiz_attempts, get_quiz_attempt, submit_answer, finish_quiz_attempt},
};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = get_config().expect("Failed to load config");
    
    // Initialize logging with the configured level
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::builder()
        .format_timestamp_millis()
        .init();
    
    info!("Starting Quiz App with configuration:");
    info!("Server: {}:{}", config.server_host, config.server_port);
    info!("Workers: {}", config.workers);
    info!("Log Level: {}", config.log_level);
    
    // Setup database connection pool
    info!("Initializing database connection pool...");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                info!("Database connection pool created successfully");
                pool
            },
            Err(e) => {
                error!("Failed to create database pool: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };

    // Initialize database schema
    info!("Initializing database schema...");
    if let Err(e) = db::init_db(&pool).await {
        error!("Failed to initialize database: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }
    info!("Database schema initialized successfully");

    info!("Configuring HTTP server...");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/quizzes")
                            .wrap(JwtAuthService)
                            .service(get_quizzes)
                            .service(get_quiz)
                            .service(create_quiz)
                            .service(update_quiz)
                            .service(delete_quiz)
                            .service(
                                web::scope("/{quiz_id}/attempts")
                                    .service(web::resource("")
                                        .route(web::post().to(start_quiz_attempt))
                                        .route(web::get().to(get_quiz_attempts_by_quiz))
                                    )
                            )
                    )
                    .service(
                        web::scope("/attempts")
                            .service(web::resource("")
                                .route(web::get().to(get_user_quiz_attempts))
                            )
                            .service(web::resource("/{attempt_id}")
                                .route(web::get().to(get_quiz_attempt))
                                .route(web::post().to(submit_answer))
                                .route(web::put().to(finish_quiz_attempt))
                            )
                    )
            )
            .service(web::resource("/health").to(health_check))
    })
    .bind((config.server_host.clone(), config.server_port))
    .map_err(|e| {
        error!("Failed to bind server to {}:{} - {}", config.server_host, config.server_port, e);
        e
    })?
    .workers(config.workers)
    .run()
    .await
}
