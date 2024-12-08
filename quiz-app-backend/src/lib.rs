pub mod auth;
pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;

pub use auth::jwt::{Claims, validate_token};
pub use models::quiz::Quiz;
pub use models::user::User;
pub use error::AppError;

pub mod test_helpers;

#[cfg(test)]
mod tests {
    // Internal test modules can go here
}

use actix_web::{
    web,
    HttpResponse,
};

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health").route(web::get().to(|| async { HttpResponse::Ok().finish() })))
            .service(
                web::scope("/auth")
                    .service(handlers::user::register)
                    .service(handlers::user::login)
            )
            .service(
                web::scope("/quizzes")
                    .wrap(middleware::auth::Auth)
                    .service(handlers::quiz::create_quiz)
                    .service(handlers::quiz::get_quiz)
                    .service(handlers::quiz::update_quiz)
                    .service(handlers::quiz::delete_quiz)
                    .service(handlers::quiz::submit_quiz)
            )
    );
}
