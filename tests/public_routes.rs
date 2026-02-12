#[cfg(test)]
mod test {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;
    use watch_tower::public_routes;

    #[tokio::test]
    async fn index_returns_200() {
        let response = public_routes()
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn index_returns_html_content_type() {
        let response = public_routes()
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
        let response = public_routes()
            .oneshot(Request::get("/inxed").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    
    #[tokio::test]
    async fn login_returns_200() {
        let response = public_routes()
            .oneshot(Request::get("/login").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn login_returns_html_content_type() {
        let response = public_routes()
            .oneshot(Request::get("/login").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let content_type = response
            .headers()
            .get("content-type")
            .expect("missing content-type header");
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }


    #[tokio::test]
    async fn register_returns_200() {
        let response = public_routes()
            .oneshot(Request::get("/register").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn register_returns_html_content_type() {
        let response = public_routes()
            .oneshot(Request::get("/register").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let content_type = response
            .headers()
            .get("content-type")
            .expect("missing content-type header");
        assert!(content_type.to_str().unwrap().contains("text/html"));
    }
}
