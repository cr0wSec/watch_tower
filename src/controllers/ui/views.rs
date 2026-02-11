use crate::utils::error::AppError;
use askama::Template;
use axum::response::Html;
use crate::templates::IndexTemplate;

pub async fn get_index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate.render()?))
}

#[cfg(test)]
mod test {
    use crate::controllers::ui::views::get_index;

    #[tokio::test]
    async fn get_index_renders_successfully() {
        let result = get_index().await;
        assert!(result.is_ok(), "template rendering failed: {:?}", result.err());

        let html = result.unwrap().0;
        assert!(html.contains("Watch Tower"));
    }

    
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;
    use crate::app;

    #[tokio::test]
    async fn index_returns_200() {
        let response = app()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_returns_html_content_type() {
        let response = app()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let content_type = response.headers().get("content-type").expect("missing content-type header");
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let response = app()
            .oneshot(Request::get("/inxed").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
