#[cfg(test)]
mod test {

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;
    use watch_tower::{app, create_pool, DbPool};

    fn test_pool() -> DbPool {
        dotenvy::dotenv().ok();
        let db_url = std::env::var("DATABASE_URL_TEST")
            .unwrap_or_else(|_| "postgres://localhost/watch_tower_test".to_string());
        create_pool(&db_url)
    }

    #[tokio::test]
    async fn index_returns_200() {
        let response = app(test_pool())
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_returns_html_content_type() {
        let response = app(test_pool())
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let content_type = response
            .headers()
            .get("content-type")
            .expect("missing content-type header");
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let response = app(test_pool())
            .oneshot(Request::get("/inxed").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
