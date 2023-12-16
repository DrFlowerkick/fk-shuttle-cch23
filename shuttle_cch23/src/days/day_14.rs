//!day_14.rs

use crate::app_error::{AppError, AppResult};
use axum::{
    routing::post,
    Json, Router, response::{IntoResponse, Html},
};
use serde::Deserialize;
use askama::Template;

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

async fn unsafe_render(Json(input): Json<UnsafeTemplate>) -> AppResult<Html<String>> {
    eprintln!("{}", input.content);
    let html = input.render().map_err(AppError::to_bad_request)?.trim().to_owned();
    eprintln!("{}", html);
    //Ok(input)
    Ok(Html(html))
}


#[derive(Template, Deserialize)]
#[template(path = "index.html")]
struct SafeTemplate {
    content: String,
}

async fn safe_render(Json(input): Json<SafeTemplate>) -> impl IntoResponse {
    input
}