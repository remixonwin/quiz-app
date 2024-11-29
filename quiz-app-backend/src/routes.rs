use actix_web::{web, Scope};

pub fn quiz_routes() -> Scope {
    web::scope("/api/quizzes")
        .route("", web::get().to(handlers::quiz::list_quizzes))
        .route("", web::post().to(handlers::quiz::create_quiz))
        .route("/{id}", web::get().to(handlers::quiz::get_quiz))
        .route("/{id}", web::put().to(handlers::quiz::update_quiz))
        .route("/{id}", web::delete().to(handlers::quiz::delete_quiz))
        .route("/{id}/submit", web::post().to(handlers::quiz::submit_quiz))
}

pub fn user_routes() -> Scope {
    web::scope("/api/users")
        .route("/register", web::post().to(handlers::user::register))
        .route("/login", web::post().to(handlers::user::login))
        .route("/profile", web::get().to(handlers::user::get_profile))
        .route("/profile", web::put().to(handlers::user::update_profile))
}
