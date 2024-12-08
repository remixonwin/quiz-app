#[cfg(test)]
mod question_tests {
    use sqlx::PgPool;
    use uuid::Uuid;

    use quiz_app_backend::models::{
        question::{CreateQuestion, Question},
        DbModel,
    };

    #[sqlx::test]
    async fn test_create_question(pool: PgPool) {
        let quiz_id = Uuid::new_v4();
        let question = Question::create(
            &pool,
            CreateQuestion {
                quiz_id,
                question_text: "What is the capital of France?".to_string(),
                order_num: 1,
                points: 1,
            },
        )
        .await
        .unwrap();

        assert_eq!(question.quiz_id, quiz_id);
        assert_eq!(question.question_text, "What is the capital of France?");
        assert_eq!(question.order_num, 1);
        assert_eq!(question.points, 1);
    }
}
