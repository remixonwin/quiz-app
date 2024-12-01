use actix_web::web;
use crate::handlers::{user, quiz, question};
use crate::auth;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/auth")
                .service(user::register)
                .service(user::login)
        )
        .service(
            web::scope("/api/users")
                .service(user::get_profile)
                .service(user::update_profile)
                .wrap(auth::Auth)
        )
        .service(
            web::scope("/api/quizzes")
                .wrap(auth::Auth)
                .service(quiz::create_quiz)
                .service(quiz::get_quizzes)
                .service(quiz::get_quiz)
                .service(quiz::update_quiz)
                .service(quiz::delete_quiz)
                .service(quiz::submit_quiz)
                .service(
                    web::scope("/{quiz_id}/questions")
                        .service(question::create_question)
                        .service(question::list_questions)
                        .service(question::get_question)
                        .service(question::update_question)
                        .service(question::delete_question)
                        .service(
                            web::scope("/{question_id}/answers")
                                .service(question::create_answer)
                                .service(question::get_answers)
                        )
                )
        );
}
