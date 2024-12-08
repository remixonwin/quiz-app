use actix_cors::Cors;
use actix_web::{
    web, App, HttpServer,
    middleware::Logger,
};
use actix_web::http::header;
use sqlx::postgres::PgPool;

use quiz_app_backend::{
    get_config,
    configure_app,
};

mod auth;
mod handlers;
mod models;
mod error;
mod middleware;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = get_config().expect("Failed to load config");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(configure_app)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
