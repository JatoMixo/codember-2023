use challenge_05::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let valid_user = User {
            id: "1a421fa".to_string(),
            username: "alex".to_string(),
            email: "asdasd@asd.com".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(valid_user.is_valid_id(), true);

        let invalid_user = User {
            id: "___asdf5".to_string(),
            username: "asdas".to_string(),
            email: "asdsad".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(invalid_user.is_valid_id(), false);
    }

    #[test]
    fn test_username() {
        let valid_user = User {
            id: "1a421fa".to_string(),
            username: "alex".to_string(),
            email: "asdasd@asd.com".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(valid_user.is_valid_username(), true);

        let invalid_user = User {
            id: "___asdf5".to_string(),
            username: "___---asdas".to_string(),
            email: "asdsad".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(invalid_user.is_valid_username(), false);
    }

    #[test]
    fn test_email() {
        let valid_user = User {
            id: "asdasd".to_string(),
            username: "asdasd".to_string(),
            email: "alex@gmail.com".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(valid_user.is_valid_email(), true);

        let mut invalid_user = User {
            id: "asdasd".to_string(),
            username: "asdasd".to_string(),
            email: "@@@.com".to_string(),
            age: None,
            location: None,
        };
        assert_eq!(invalid_user.is_valid_email(), false);

        invalid_user.email = "potato.com".to_string();
        assert_eq!(invalid_user.is_valid_email(), false);

        invalid_user.email = "user@potato".to_string();
        assert_eq!(invalid_user.is_valid_email(), false);

        invalid_user.email = "user".to_string();
        assert_eq!(invalid_user.is_valid_email(), false);
    }

    #[test]
    fn test_user() {
        let valid_user = User {
            id: "1a421fa".to_string(),
            username: "alex".to_string(),
            email: "alex@gmail.com".to_string(),
            age: Some("18".to_string()),
            location: Some("Barcelona".to_string()),
        };

        assert_eq!(valid_user.is_valid(), true);

        let invalid_user = User {
            id: "___".to_string(),
            username: "___".to_string(),
            email: "user@google".to_string(),
            age: None,
            location: None,
        };

        assert_eq!(invalid_user.is_valid(), false);
    }
}
