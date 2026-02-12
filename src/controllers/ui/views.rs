use crate::templates::{IndexTemplate, LoginTemplate, RegisterTemplate};
use crate::utils::error::AppError;
use askama::Template;
use axum::response::Html;

pub async fn get_index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate.render()?))
}

pub async fn get_login() -> Result<Html<String>, AppError> {
    Ok(Html(LoginTemplate { error: None }.render()?))
}

pub async fn get_register() -> Result<Html<String>, AppError> {
    Ok(Html(RegisterTemplate { error: None }.render()?))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_index_renders_successfully() {
        let result = get_index().await;
        assert!(
            result.is_ok(),
            "template rendering failed: {:?}",
            result.err()
        );

        let html = result.unwrap().0;
        assert!(html.contains("Watch Tower"));
    }

    #[tokio::test]
    async fn get_login_renders_successfully() {
        let result = get_login().await;
        assert!(
            result.is_ok(),
            "template rendering failed: {:?}",
            result.err()
        );

        let html = result.unwrap().0;
        assert!(html.contains("<form action=\"/login\" method=\"POST\" class=\"space-y-6\">"));
    }

    #[tokio::test]
    async fn get_register_renders_successfully() {
        let result = get_register().await;
        assert!(
            result.is_ok(),
            "template rendering failed: {:?}",
            result.err()
        );

        let html = result.unwrap().0;
        assert!(html.contains("<form action=\"/register\" method=\"POST\" class=\"space-y-6\">"));
    }
}
