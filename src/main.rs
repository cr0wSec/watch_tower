mod error;
mod handlers;
mod templates;

use axum::routing::get;
use axum::Router;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app = Router::new()
        .route("/", get(handlers::get_index))
        .route("/home", get(handlers::get_home))
        .route("/login", get(handlers::get_login).post(handlers::post_login))
        .route(
            "/register",
            get(handlers::get_register).post(handlers::post_register),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to address");

    tracing::info!("server listening on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
