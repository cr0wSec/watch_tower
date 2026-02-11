
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(length(min = 3, max = 32, message = "Username must be between 3 and 32 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 8, message = "Password confirmation must be at least 8 characters"))]
    pub password_confirm: String,
}

impl RegisterForm {
    pub(crate) fn validate_passwords_match(&self) -> Result<(), &'static str> {
        if self.password != self.password_confirm {
            return Err("Passwords do not match");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_form() -> RegisterForm {
        RegisterForm {
            username: "johndoe".to_string(),
            email: "jdoe@gmail.com".to_string(),
            password: "password123".to_string(),
            password_confirm: "password123".to_string(),
        }
    }

    #[test]
    fn valid_form_passes_validation() {
        assert!(valid_form().validate().is_ok());
    }

    #[test]
    fn username_too_short_fails() {
        let mut bad_form = valid_form();
        bad_form.username = "to".to_string();
        assert!(bad_form.validate().is_err());
    }

    #[test]
    fn username_too_long_fails() {
        let mut bad_form = valid_form();
        bad_form.username = "a".repeat(80);
        assert!(bad_form.validate().is_err());
    }

    #[test]
    fn invalid_email_fails() {
        let mut bad_form = valid_form();
        bad_form.email = "invalid_email".to_string();
        assert!(bad_form.validate().is_err());
    }

    #[test]
    fn password_too_short_fails() {
        let mut bad_form = valid_form();
        bad_form.password = "a".to_string();
        bad_form.password_confirm = "a".to_string();
        assert!(bad_form.validate().is_err());
    }

    // --------
    #[test]
    fn matching_passwords_returns_ok() {
        assert!(valid_form().validate_passwords_match().is_ok());
    }

    #[test]
    fn mismatched_passwords_returns_err() {
        let mut bad_form = valid_form();
        bad_form.password = "".to_string();
        assert!(bad_form.validate_passwords_match().is_err());
    }



}