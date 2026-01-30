use askama::Template;
use axum::extract::Form;
use axum::response::Html;
use serde::Deserialize;
use validator::Validate;

use crate::error::AppError;
use crate::templates::{HomeTemplate, IndexTemplate, LoginTemplate, RegisterTemplate};

#[derive(Debug, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

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
    fn validate_passwords_match(&self) -> Result<(), &'static str> {
        if self.password != self.password_confirm {
            return Err("Passwords do not match");
        }
        Ok(())
    }
}

pub async fn get_index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate.render()?))
}

pub async fn get_home() -> Result<Html<String>, AppError> {
    Ok(Html(HomeTemplate.render()?))
}

pub async fn get_login() -> Result<Html<String>, AppError> {
    Ok(Html(LoginTemplate { error: None }.render()?))
}

pub async fn post_login(Form(form): Form<LoginForm>) -> Result<Html<String>, AppError> {
    if let Err(errors) = form.validate() {
        let template = LoginTemplate {
            error: Some(AppError::validation(errors).to_string()),
        };
        return Ok(Html(template.render()?));
    }

    // TODO: Implement actual authentication logic
    // - Fetch user from database
    // - Verify password hash with argon2
    // - Create session

    let template = LoginTemplate {
        error: Some("Login functionality not yet implemented".to_string()),
    };
    Ok(Html(template.render()?))
}

pub async fn get_register() -> Result<Html<String>, AppError> {
    Ok(Html(RegisterTemplate { error: None }.render()?))
}

pub async fn post_register(Form(form): Form<RegisterForm>) -> Result<Html<String>, AppError> {
    if let Err(errors) = form.validate() {
        let template = RegisterTemplate {
            error: Some(AppError::validation(errors).to_string()),
        };
        return Ok(Html(template.render()?));
    }

    if let Err(msg) = form.validate_passwords_match() {
        let template = RegisterTemplate {
            error: Some(msg.to_string()),
        };
        return Ok(Html(template.render()?));
    }

    // TODO: Implement actual registration logic
    // - Check if user already exists
    // - Hash password with argon2
    // - Store user in database
    // - Send verification email

    let template = RegisterTemplate {
        error: Some("Registration functionality not yet implemented".to_string()),
    };
    Ok(Html(template.render()?))
}
