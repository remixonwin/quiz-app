#[allow(unused_imports)]
#[allow(dead_code)]
use actix_web::{test, App};
use sqlx::PgPool;
use uuid::Uuid;

use quiz_app_backend::{
    auth::jwt::Claims,
    handlers,
    models::{
        user::{CreateUser, User},
        DbModel,
    },
    test_helpers::TestContext,
};

#[allow(unused_imports)]
#[actix_web::test]
async fn test_helpers_work_correctly() {
    let mut ctx = TestContext::new().await;
    let _user = ctx.pool.get_ref().await.unwrap();
    let quiz = ctx.pool.get_ref().await.unwrap();

    assert_eq!(quiz.title, "Test Quiz");
    let _ = ctx.cleanup().await;
}
