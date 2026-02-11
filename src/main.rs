mod config;
mod controllers;

use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
mod middlewares;
mod models;
mod schema;
mod static_data;
mod templates;
mod utils;

use crate::controllers::ui::views::get_index;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app())
        .await
        .expect("failed to start the server");
}

fn app() -> Router {
    Router::new()
        .route("/", get(get_index))
        // middleware
        .layer(TraceLayer::new_for_http())
}
