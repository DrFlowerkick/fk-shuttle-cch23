//!day_14.rs

use askama::Template;
use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

pub fn get_routes() -> Router {
    Router::new()
        .route("/14/unsafe", post(unsafe_render))
        .route("/14/safe", post(safe_render))
}

#[derive(Template, Deserialize)]
#[template(path = "index.html", escape = "none")]
struct UnsafeTemplate {
    content: String,
}

async fn unsafe_render(Json(input): Json<UnsafeTemplate>) -> impl IntoResponse {
    input
}

#[derive(Template, Deserialize)]
#[template(path = "index.html")]
struct SafeTemplate {
    content: String,
}

async fn safe_render(Json(input): Json<SafeTemplate>) -> impl IntoResponse {
    input
}
