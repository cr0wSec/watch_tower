use crate::templates::IndexTemplate;
use crate::utils::error::AppError;
use askama::Template;
use axum::response::Html;

pub async fn get_index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate.render()?))
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

}
