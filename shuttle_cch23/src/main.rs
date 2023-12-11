//!main.rs

use shuttle_cch23::router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = router();
    Ok(router.into())
}
