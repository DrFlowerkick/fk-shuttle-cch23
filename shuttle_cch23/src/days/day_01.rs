//!day_01.rs
use crate::AppResult;
use axum::{extract::Path, routing::get, Router};

pub fn get_routes() -> Router {
    Router::new().route("/1/*nums", get(cube_sled))
}

async fn cube_sled(Path(args): Path<String>) -> AppResult<String> {
    let xor = args
        .split("/")
        .map(|s| s.parse::<i32>())
        .reduce(|acc, e| match (acc, e) {
            (Ok(x), Ok(y)) => Ok(x ^ y),
            (Ok(_), Err(err)) | (Err(err), Ok(_)) | (Err(err), Err(_)) => Err(err),
        });
    match xor {
        Some(cs) => Ok(format!("{}", cs?.pow(3))),
        None => Ok("0".into()),
    }
}
