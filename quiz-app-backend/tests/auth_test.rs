#[cfg(test)]
mod auth_tests {
    use quiz_app_backend::auth::jwt::{generate_token, validate_token};
    use quiz_app_backend::auth::password::{hash_password, verify_password};
    use uuid::Uuid;

    #[test]
    fn test_token_generation_and_validation() {
        std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
        
        let user_id = Uuid::new_v4();
        let token = generate_token(user_id, "user")
            .expect("Failed to generate token");
        
        let claims = validate_token(&token)
            .expect("Failed to validate token");
        
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, "user");
    }

    #[test]
    fn test_invalid_token() {
        std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
        
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "test_password123";
        let hash = hash_password(password)
            .expect("Failed to hash password");
        
        let is_valid = verify_password(password, &hash)
            .expect("Failed to verify password");
        
        assert!(is_valid);
    }

    #[test]
    fn test_password_verification_with_wrong_password() {
        let password = "test_password123";
        let wrong_password = "wrong_password";
        let hash = hash_password(password)
            .expect("Failed to hash password");
        
        let is_valid = verify_password(wrong_password, &hash)
            .expect("Failed to verify password");
        
        assert!(!is_valid);
    }
}
