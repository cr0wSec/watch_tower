pub mod config;
pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod schema;
pub mod static_data;
pub mod templates;
pub mod utils;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use controllers::ui::views::get_index;

pub fn app() -> Router {
    Router::new()
        .route("/", get(get_index))
        .layer(TraceLayer::new_for_http())
}