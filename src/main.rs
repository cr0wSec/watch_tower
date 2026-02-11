
mod config;
mod controllers;
use watch_tower::run;
mod middlewares;
mod models;
mod schema;
mod static_data;
mod templates;
mod utils;

#[tokio::main]
async fn main() {
    run().await;
}
