use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Quiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQuiz {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Question {
    pub id: i32,
    pub quiz_id: i32,
    pub question_text: String,
    pub question_type: String,
    pub order_num: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQuestion {
    pub question_text: String,
    pub question_type: String,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer_text: String,
    pub is_correct: bool,
    pub order_num: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAnswer {
    pub answer_text: String,
    pub is_correct: bool,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct QuizAttempt {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateQuizAttempt {
    pub quiz_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitQuizAttempt {
    pub quiz_id: i32,
    pub answers: Vec<SubmittedAnswer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmittedAnswer {
    pub question_id: i32,
    pub answer_id: i32,
}
