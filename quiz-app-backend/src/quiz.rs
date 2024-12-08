use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}
