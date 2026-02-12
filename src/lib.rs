pub mod config;
pub mod controllers;
pub mod middlewares;
pub mod models;
pub mod schema;
pub mod static_data;
pub mod templates;
pub mod utils;

use axum::{Router, routing::get};
use axum::routing::post;
use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use controllers::ui::views::get_index;
use tower_http::trace::TraceLayer;
use crate::controllers::api::auth::{post_login, post_register};
use crate::controllers::ui::views::{get_login, get_register};

pub type DbPool = Pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("infra/migrations");

pub fn create_pool(database_url: &str) -> DbPool {
    let manager = deadpool_diesel::postgres::Manager::new(
        database_url.to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );
    Pool::builder(manager).build().unwrap()
}

pub async fn run_migrations(pool: &DbPool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
}

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/login", get(get_login))
        .route("/register", get(get_register))
}

pub fn auth_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/auth/login", post(post_login))
        .route("/auth/register", post(post_register))
        .with_state(pool)
}

pub fn app(pool: DbPool) -> Router {

    public_routes()
        .nest("/api", auth_routes(pool))
        .layer(TraceLayer::new_for_http())
}
