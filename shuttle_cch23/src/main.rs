//!main.rs

use cch23_drflowerkick::{router, SharedState};

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let state = SharedState::default();
    let router = router(&state);
    Ok(router.into())
}
