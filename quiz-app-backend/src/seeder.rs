use sqlx::PgPool;

use crate::{
    auth::hash_password,
    models::{User, Answer},
};

#[allow(dead_code)]
struct QuizData {
    title: String,
    description: String,
    questions: Vec<QuestionData>,
}

#[allow(dead_code)]
struct QuestionData {
    text: String,
    points: i32,
    answers: Vec<Answer>,
}

#[allow(dead_code)]
pub async fn seed_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Confirm that the connection pool uses the correct DATABASE_URL

    // Create a test user
    let password_hash = hash_password("password123").unwrap();
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        "testuser",
        password_hash,
        "user"
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
                INSERT INTO answers (question_id, answer_text, is_correct, order_num)
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
