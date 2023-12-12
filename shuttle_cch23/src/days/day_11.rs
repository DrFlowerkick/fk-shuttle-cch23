//!day_11.rs

use crate::app_error::{AppError, AppResult};
use axum::{extract::Multipart, routing::post, Router};
use tower_http::services::ServeDir;

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(magical_red))
}

async fn magical_red(mut multipart: Multipart) -> AppResult<String> {
    let mut num_magical_red = 0;
    while let Some(field) = multipart.next_field().await? {
        let name = field
            .name()
            .ok_or(AppError::bad_request("no field name provided"))?
            .to_string();
        let data = field.bytes().await?;

        if name == "image" {
            let image = image::load_from_memory(&data)?;
            num_magical_red = image
                .as_rgb8()
                .ok_or(AppError::bad_request("no rgb8 image provided"))?
                .pixels()
                .map(|p| (p.0[0] as u16, p.0[1] as u16, p.0[2] as u16))
                .filter(|p| p.0 > p.1 + p.2)
                .count();
        } else {
            return Err(AppError::bad_request("unknown field type"));
        }
    }
    Ok(format!("{}", num_magical_red))
}
