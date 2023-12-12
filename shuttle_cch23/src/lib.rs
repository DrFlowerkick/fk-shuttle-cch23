pub mod app_error;
pub mod days;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

/// Day -1
async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .merge(days::day_01::get_routes())
        .merge(days::day_04::get_routes())
        .merge(days::day_06::get_routes())
        .merge(days::day_07::get_routes())
        .merge(days::day_08::get_routes())
        .merge(days::day_11::get_routes())
}
