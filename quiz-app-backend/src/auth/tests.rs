#[cfg(test)]
mod tests {
    use crate::auth::jwt::{Claims, validate_token};
    use uuid::Uuid;

    #[test]
    fn test_jwt_flow() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id);
        let token = claims.generate_token().unwrap();
        let validated_claims = validate_token(&token).unwrap();
        assert_eq!(validated_claims.sub, user_id);
    }
}
