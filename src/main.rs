use crate::libs::run;

mod config;
mod controllers;
mod libs;
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
