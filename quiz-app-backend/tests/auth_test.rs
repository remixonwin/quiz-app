#[cfg(test)]
mod auth_tests {
    use quiz_app_backend::auth;
    use dotenv::dotenv;

    fn setup() {
        dotenv().ok();
    }

    #[test]
    fn test_valid_token() {
        setup();
        let user_id = 1;
        let token = auth::generate_token(user_id, "user").unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_invalid_user_id() {
        setup();
        let result = auth::generate_token(-1, "user");
        assert!(result.is_err());
    }
}
