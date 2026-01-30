mod error;
mod templates;
mod models;
mod controllers;
mod middlewares;

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
        .route("/", get(controllers::ui::views::get_index))
        .route("/home", get(controllers::ui::views::get_home))
        .route("/auth/login", get(controllers::api::auth::get_login).post(controllers::api::auth::post_login))
        .route(
            "/auth/register",
            get(controllers::api::auth::get_register).post(controllers::api::auth::post_register),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to address");

    tracing::info!("server listening on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
