use sqlx::PgPool;
use crate::models::{Quiz, Question, Answer};

#[derive(Debug)]
struct QuizData {
    title: String,
    description: String,
    questions: Vec<QuestionData>,
}

#[derive(Debug)]
struct QuestionData {
    question_text: String,
    options: Vec<String>,
    correct_answer: String,
}

pub async fn seed_database(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Create a test user
    let user = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, role)
        VALUES ($1, $2, $3, 'user')
        RETURNING id
        "#,
        "testuser",
        "test@example.com",
        "hashed_password" // In production, use proper password hashing
    )
    .fetch_one(pool)
    .await?;

    // Create a test quiz
    let quiz = sqlx::query!(
        r#"
        INSERT INTO quizzes (title, description, created_by)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        "Test Quiz",
        "A quiz for testing",
        user.id
    )
    .fetch_one(pool)
    .await?;

    // Create test questions
    let questions = vec![
        ("What is 2 + 2?", 0),
        ("What is the capital of France?", 1),
        ("Who wrote Romeo and Juliet?", 2),
    ];

    for (question_text, order_num) in questions {
        let question = sqlx::query!(
            r#"
            INSERT INTO questions (quiz_id, question_text, order_num)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            quiz.id,
            question_text,
            order_num
        )
        .fetch_one(pool)
        .await?;

        // Create answers for each question
        let answers = match order_num {
            0 => vec![
                ("4", true, 0),
                ("3", false, 1),
                ("5", false, 2),
                ("6", false, 3),
            ],
            1 => vec![
                ("Paris", true, 0),
                ("London", false, 1),
                ("Berlin", false, 2),
                ("Madrid", false, 3),
            ],
            2 => vec![
                ("William Shakespeare", true, 0),
                ("Charles Dickens", false, 1),
                ("Jane Austen", false, 2),
                ("Mark Twain", false, 3),
            ],
            _ => vec![],
        };

        for (text, is_correct, order_num) in answers {
            sqlx::query!(
                r#"
                INSERT INTO answers (question_id, text, is_correct, order_num)
                VALUES ($1, $2, $3, $4)
                "#,
                question.id,
                text,
                is_correct,
                order_num
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
