pub mod app_error;
pub mod days;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

/// Day -1
async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    pub db: HashMap<String, Instant>,
}

pub fn router(state: &SharedState, pool: PgPool) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .merge(days::day_01::get_routes())
        .merge(days::day_04::get_routes())
        .merge(days::day_06::get_routes())
        .merge(days::day_07::get_routes())
        .merge(days::day_08::get_routes())
        .merge(days::day_11::get_routes())
        .merge(days::day_12::get_routes(state))
        .merge(days::day_13::get_routes(pool))
}
