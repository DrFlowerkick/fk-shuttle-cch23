//!day_11.rs

use crate::AppResult;
use axum::{routing::post, Router, extract::Multipart};
use tower_http::services::ServeDir;
use anyhow::anyhow;

pub fn get_routes() -> Router {
    Router::new()
        .nest_service("/11/assets", ServeDir::new("assets"))
        .route("/11/red_pixels", post(magical_red))
}

async fn magical_red(mut multipart: Multipart) -> AppResult<String> {
    let mut num_magical_red = 0;
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await?;

        if name == "image" {
            let image = image::load_from_memory(&data)?;
            num_magical_red = image.as_rgb8()
                .unwrap()
                .pixels()
                .map(|p| (p.0[0] as u16, p.0[1] as u16, p.0[2] as u16))
                .filter(|p| p.0 > p.1 + p.2)
                .count();
        } else {
            return Err(crate::AppError(anyhow!("unknown filed type")));
        }
    }
    Ok(format!("{}", num_magical_red))
}