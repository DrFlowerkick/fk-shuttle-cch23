//!main.rs

use cch23_drflowerkick::router;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = router();
    Ok(router.into())
}
