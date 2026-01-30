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
