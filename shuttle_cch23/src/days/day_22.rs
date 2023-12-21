//!day_22.rs

use crate::app_error::AppResult;
use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/22", get(day_22))
}

async fn day_22() -> AppResult<()> {
    Ok(())
}