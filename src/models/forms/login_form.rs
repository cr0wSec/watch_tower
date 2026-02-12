use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_form() -> LoginForm {
        LoginForm {
            email: "jdoe@gmail.com".to_string(),
            password: "password123".to_string(),
        }
    }
    #[test]
    fn valid_form_passes_validation() {
        assert!(valid_form().validate().is_ok());
    }

    #[test]
    fn invalid_email_fails() {
        let mut bad_form = valid_form();
        bad_form.email = "bad_email".to_string();
        assert!(bad_form.validate().is_err());
    }

    #[test]
    fn password_too_short_fails() {
        let mut bad_form = valid_form();
        bad_form.password = "".to_string();
        assert!(bad_form.validate().is_err());
    }
}
