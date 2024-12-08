use actix_web::{test, web, App, dev};
use actix_web::http;
use actix_http::Request;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::jwt::Claims,
    error::AppError,
    handlers,
    models::{
        quiz::{Quiz, CreateQuiz},
        DbModel,
    },
};

pub struct TestContext {
    pub pool: PgPool,
    pub app: dev::Service<Request>,
    pub user_id: Uuid,
    pub token: String,
}

impl TestContext {
    pub async fn new() -> Self {
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/quiz_app_test")
            .await
            .unwrap();

        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id);
        let token = claims.generate_token().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::scope("/api")
                        .service(handlers::user::register)
                        .service(handlers::user::login)
                        .service(handlers::user::get_me)
                        .service(handlers::user::update_user)
                        .service(handlers::quiz::create_quiz)
                        .service(handlers::quiz::get_quiz)
                        .service(handlers::quiz::get_quizzes)
                        .service(handlers::quiz::update_quiz)
                        .service(handlers::quiz::delete_quiz)
                        .service(handlers::question::create_question)
                        .service(handlers::question::get_question)
                        .service(handlers::question::get_questions)
                        .service(handlers::question::update_question)
                        .service(handlers::question::delete_question)
                        .service(handlers::answer::create_answer)
                        .service(handlers::answer::get_answers)
                        .service(handlers::answer::update_answer)
                        .service(handlers::answer::delete_answer),
                ),
        )
        .await;

        Self {
            pool,
            app,
            user_id,
            token,
        }
    }

    pub async fn create_test_quiz(&self, title: String) -> Result<Quiz, AppError> {
        let quiz = CreateQuiz {
            title,
            description: Some("Test quiz".to_string()),
            created_by: self.user_id,
        };

        Quiz::create(&self.pool, quiz).await
    }

    pub async fn make_request(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<String>,
    ) -> Result<dev::ServiceResponse, actix_web::Error> {
        let mut req = test::TestRequest::with_uri(uri).method(method);

        req = req.insert_header(("Authorization", format!("Bearer {}", self.token)));

        if let Some(body) = body {
            req = req.set_payload(body);
        }

        test::call_service(&self.app, req.to_request()).await
    }

    pub async fn cleanup(&self) -> Result<(), AppError> {
        // Delete all test data
        sqlx::query!("DELETE FROM quiz_attempts").execute(&self.pool).await?;
        sqlx::query!("DELETE FROM answers").execute(&self.pool).await?;
        sqlx::query!("DELETE FROM questions").execute(&self.pool).await?;
        sqlx::query!("DELETE FROM quizzes").execute(&self.pool).await?;
        sqlx::query!("DELETE FROM users").execute(&self.pool).await?;
        Ok(())
    }
}
