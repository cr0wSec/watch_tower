mod error;
mod templates;
mod models;
mod controllers;
mod middlewares;
mod static_data;

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


    let api_routes = Router::new()
        .route("/layers", get(controllers::api::layers::get_all_layers))
        .route("/layers/pipelines", get(controllers::api::layers::get_pipelines))
        .route("/layers/straits", get(controllers::api::layers::get_straits))
        .route(
            "/layers/military-bases",
            get(controllers::api::layers::get_military_bases),
        )
        .route(
            "/layers/nuclear-plants",
            get(controllers::api::layers::get_nuclear_plants),
        )
        .route("/auth/login", get(controllers::api::auth::get_login).post(controllers::api::auth::post_login))
        .route(
            "/auth/register",
            get(controllers::api::auth::get_register).post(controllers::api::auth::post_register),
        );

    let app = Router::new()
        .route("/", get(controllers::ui::views::get_index))
        .route("/home", get(controllers::ui::views::get_home)
        )
        .nest("/api", api_routes);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to address");

    tracing::info!("server listening on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
