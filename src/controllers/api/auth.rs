use askama::Template;
use axum::Form;
use axum::response::Html;
use validator::Validate;
use crate::models::forms::login_form::LoginForm;
use crate::models::forms::register_form::RegisterForm;
use crate::templates::{LoginTemplate, RegisterTemplate};
use crate::utils::error::AppError;

pub async fn get_login() -> Result<Html<String>, AppError> {
    Ok(Html(LoginTemplate {error: None}.render()?))
}


pub async fn get_register() -> Result<Html<String>, AppError> {
    Ok(Html(RegisterTemplate {error: None}.render()?))
}


pub async fn post_login(Form(form): Form<LoginForm>) -> Result<Html<String>, AppError> {
    if let Err(errors) = form.validate() {
        let template = LoginTemplate {
            error: Some(AppError::validation(errors).to_string()),
        };
        return Ok(Html(template.render()?));
    }

    // Fetch user from database

    // Verify password hash with Argon2

    // Create a session

    // Redirect to homepage

    let template = LoginTemplate {
        error: Some("Login functionality not yet implemented".to_string()),
    };
    Ok(Html(template.render()?))
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

    // Check if user already exists

    // Hash password with Argon2

    // Store user in database

    // Redirect to login page

    let template = RegisterTemplate {
        error: Some("Registration functionality not yet implemented".to_string()),
    };
    Ok(Html(template.render()?))
}