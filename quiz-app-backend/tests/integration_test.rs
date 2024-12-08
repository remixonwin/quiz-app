#[allow(unused_imports)]
use actix_web::{
    test,
    web,
    App,
    http::StatusCode,
    http::Method,
};
use quiz_app_backend::{
    models::{
        user::{CreateUser, LoginCredentials},
    },
    test_helpers::TestContext,
};
use quiz_app_backend::auth::jwt::{generate_token, validate_token};
use uuid::Uuid;

#[cfg(test)]
#[allow(unused_imports)]
mod integration_tests {
    #[allow(unused_imports)]
    use actix_web::http::StatusCode;
    use quiz_app_backend::{
        models::{
            user::{CreateUser, LoginCredentials},
        },
        test_helpers::TestContext,
    };
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_health_check() {
        let mut ctx = TestContext::new().await;
        let resp = ctx.make_request(actix_web::http::Method::GET, "/api/health", Option::<()>::None)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_registration() {
        let mut ctx = TestContext::new().await;

        let user_data = CreateUser {
            username: format!("testuser_{}", Uuid::new_v4()),
            email: format!("test_{}@example.com", Uuid::new_v4()),
            password: "password123".to_string(),
        };

        let resp = ctx.make_request(
            actix_web::http::Method::POST,
            "/api/auth/register",
            Some(user_data)
        )
        .await
        .unwrap();

        assert_eq!(resp.status(), StatusCode::CREATED);
        let _ = ctx.cleanup().await;
    }

    #[actix_web::test]
    async fn test_user_login() {
        let mut ctx = TestContext::new().await;
        let user = ctx.create_test_user().await.unwrap();

        let login_data = LoginCredentials {
            email: user.email,
            password: "password123".to_string(),
        };

        let resp = ctx.make_request(
            actix_web::http::Method::POST,
            "/api/auth/login",
            Some(login_data)
        )
        .await
        .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let _ = ctx.cleanup().await;
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[allow(unused_imports)]
    use actix_web::http::Method;
    use quiz_app_backend::{
        models::user::{CreateUser, LoginCredentials},
        test_helpers::TestContext,
    };
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_user_flow() {
        let mut ctx = TestContext::new().await;

        let username = format!("testuser_{}", Uuid::new_v4());
        let email = format!("test_{}@example.com", Uuid::new_v4());
        let password = "testpass123".to_string();

        // Register user
        let resp = ctx.make_request(
            Method::POST,
            "/api/auth/register",
            Some(CreateUser {
                username: username.clone(),
                email: email.clone(),
                password: password.clone(),
            }),
        )
        .await
        .expect("Failed to make register request");
        assert_eq!(resp.status().as_u16(), 201);

        // Login user
        let resp = ctx.make_request(
            Method::POST,
            "/api/auth/login",
            Some(LoginCredentials {
                email,
                password,
            }),
        )
        .await
        .expect("Failed to make login request");
        assert_eq!(resp.status().as_u16(), 200);

        // Clean up after all tests are done
        ctx.cleanup().await.unwrap();
    }
}
