pub mod days;

use axum::{routing::get, Router, response::IntoResponse, http::StatusCode};

/// Day -1
async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .merge(days::day_01::get_routes())
        .merge(days::day_04::get_routes())
        .merge(days::day_06::get_routes())
        .merge(days::day_07::get_routes());

    Ok(router.into())
}