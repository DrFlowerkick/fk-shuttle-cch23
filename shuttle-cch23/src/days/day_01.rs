//!day_01.rs
use axum::{extract::Path, routing::get, Router};

pub fn get_routes() -> Router {
    Router::new().route("/1/*nums", get(cube_sled))
}

async fn cube_sled(Path(args): Path<String>) -> String {
    let xor = args
        .split("/")
        .map(|s| s.parse::<i32>().expect("only integers allowed"))
        .reduce(|acc, e| acc ^ e);
    match xor {
        Some(cs) => format!("{}", cs.pow(3)),
        None => "0".into()
    }
}