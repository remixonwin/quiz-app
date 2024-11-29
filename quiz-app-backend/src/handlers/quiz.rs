use actix_web::{
    web, 
    Error, 
    HttpResponse, 
    error::{
        ErrorInternalServerError, 
        ErrorBadRequest
    }
};
use chrono::Utc;
use sqlx::PgPool;
use serde_json::json;

use crate::auth::Claims;
use crate::models::{
    Answer, CreateQuiz, Question, Quiz, QuizAttempt, SubmitQuizAttempt
};

pub async fn list_quizzes(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let quizzes = sqlx::query_as!(
        Quiz,
        r#"
        SELECT 
            id as "id!", 
            title, 
            description, 
            creator_id as "creator_id!", 
            created_at 
        FROM quizzes
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(quizzes))
}

pub async fn create_quiz(
    pool: web::Data<PgPool>,
    quiz_data: web::Json<CreateQuiz>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, Error> {
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        INSERT INTO quizzes (title, description, creator_id, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING 
            id as "id!", 
            title, 
            description, 
            creator_id as "creator_id!", 
            created_at
        "#,
        quiz_data.title,
        quiz_data.description,
        claims.sub,
        Utc::now()
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(quiz))
}

pub async fn get_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        SELECT 
            id as "id!", 
            title, 
            description, 
            creator_id as "creator_id!", 
            created_at 
        FROM quizzes
        WHERE id = $1
        "#,
        quiz_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorBadRequest("Quiz not found"))?;

    // Fetch questions for the quiz
    let questions = sqlx::query_as!(
        Question,
        r#"
        SELECT 
            id as "id!", 
            quiz_id as "quiz_id!", 
            question_text, 
            question_type, 
            order_num, 
            created_at 
        FROM questions
        WHERE quiz_id = $1
        ORDER BY order_num
        "#,
        quiz.id
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    // Fetch answers for each question
    let mut quiz_with_details = json!({
        "quiz": quiz,
        "questions": []
    });

    for question in questions {
        let answers = sqlx::query_as!(
            Answer,
            r#"
            SELECT 
                id as "id!", 
                question_id as "question_id!", 
                answer_text, 
                is_correct, 
                order_num, 
                created_at 
            FROM answers
            WHERE question_id = $1
            ORDER BY order_num
            "#,
            question.id
        )
        .fetch_all(pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

        let question_with_answers = json!({
            "question": question,
            "answers": answers
        });

        quiz_with_details["questions"].as_array_mut().unwrap().push(question_with_answers);
    }

    Ok(HttpResponse::Ok().json(quiz_with_details))
}

pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<i32>,
    claims: web::ReqData<Claims>,
    submit_data: web::Json<SubmitQuizAttempt>,
) -> Result<HttpResponse, Error> {
    // Verify quiz exists
    let quiz_id_inner = *quiz_id;
    let _quiz = sqlx::query_as!(
        Quiz,
        r#"
        SELECT 
            id as "id!", 
            title, 
            description, 
            creator_id as "creator_id!", 
            created_at 
        FROM quizzes
        WHERE id = $1
        "#,
        quiz_id_inner
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(ErrorInternalServerError)?
    .ok_or_else(|| ErrorBadRequest("Quiz not found"))?;

    // Start a transaction
    let mut tx = pool.begin()
        .await
        .map_err(ErrorInternalServerError)?;

    // Create quiz attempt
    let attempt = sqlx::query_as!(
        QuizAttempt,
        r#"
        INSERT INTO quiz_attempts (user_id, quiz_id, created_at)
        VALUES ($1, $2, $3)
        RETURNING 
            id as "id!", 
            user_id as "user_id!", 
            quiz_id as "quiz_id!", 
            score, 
            completed_at, 
            created_at
        "#,
        claims.sub,
        quiz_id_inner,
        Utc::now()
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    // Calculate score
    let mut total_questions = 0;
    let mut correct_answers = 0;

    for submitted_answer in &submit_data.answers {
        // Verify the answer belongs to the correct question and is correct
        let is_correct = sqlx::query!(
            r#"
            SELECT is_correct 
            FROM answers 
            WHERE id = $1 AND question_id = $2
            "#,
            submitted_answer.answer_id,
            submitted_answer.question_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(ErrorInternalServerError)?
        .map(|row| row.is_correct)
        .unwrap_or(false);

        total_questions += 1;
        if is_correct {
            correct_answers += 1;
        }
    }

    // Convert score to Option<i32> for compatibility
    let score = if total_questions > 0 {
        Some((correct_answers * 100 / total_questions) as i32)
    } else {
        None
    };

    // Update quiz attempt with final score
    let attempt = sqlx::query_as!(
        QuizAttempt,
        r#"
        UPDATE quiz_attempts 
        SET 
            score = $1, 
            completed_at = $2
        WHERE id = $3
        RETURNING 
            id as "id!", 
            user_id as "user_id!", 
            quiz_id as "quiz_id!", 
            score, 
            completed_at, 
            created_at
        "#,
        score,
        Utc::now(),
        attempt.id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(ErrorInternalServerError)?;

    // Commit transaction
    tx.commit()
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(json!({
        "attempt": attempt,
        "total_questions": total_questions,
        "correct_answers": correct_answers
    })))
}
