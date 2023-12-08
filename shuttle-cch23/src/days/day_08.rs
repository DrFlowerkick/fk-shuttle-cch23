//!day_08.rs

use axum::{extract::Path, routing::get, Router};
use rustemon::{pokemon::pokemon, client::RustemonClient};

pub fn get_routes() -> Router {
    Router::new()
        .route("/8/weight/:pid", get(poki_weight))
        .route("/8/drop/:pid", get(poki_momentum))
}

async fn poki_weight(Path(pid): Path<i64>) -> String {
    let poki = pokemon::get_by_id(pid, &RustemonClient::default()).await.expect("pokemon not found");
    format!("{}", (poki.weight as f64) / 10.0)
}

async fn poki_momentum(Path(pid): Path<i64>) -> String {
    let poki = pokemon::get_by_id(pid, &RustemonClient::default()).await.expect("pokemon not found");
    // mom: p = m * v; v = sqrt(2 * s * a)
    let mom = f64::sqrt(2.0 * 10.0 * 9.825) * (poki.weight as f64) / 10.0;
    
    format!("{:.10}", mom)
}